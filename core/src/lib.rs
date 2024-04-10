use chrono;
use std::clone::Clone;
use std::collections::HashMap;

const DOC_ID_FIELD_INDEX: usize = 0;

#[derive(Clone, Debug)]
pub enum DocField {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    DateTime(chrono::DateTime<chrono::Utc>),
    Doc(Doc),
    BoolArray(Vec<bool>),
    IntArray(Vec<i64>),
    FloatArray(Vec<f64>),
    StringArray(Vec<String>),
    DateTimeArray(Vec<chrono::DateTime<chrono::Utc>>),
    DocArray(Vec<Doc>),
    MaybeBool(Option<bool>),
    MaybeInt(Option<i64>),
    MaybeFloat(Option<f64>),
    MaybeString(Option<String>),
    MaybeDateTime(Option<chrono::DateTime<chrono::Utc>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum DocumentStatus {
    Active,
    Deleted,
}

#[derive(Clone, Debug)]
pub struct Doc {
    pub hidden: bool,
    pub fields: Vec<DocField>,
    pub status: DocumentStatus,
    pub schema_version_id: String,
}

impl Doc {
    pub fn new() -> Self {
        let mut fields = Vec::new();
        fields.resize(DOC_ID_FIELD_INDEX + 1, DocField::String("".to_string()));

        Self {
            hidden: false,
            fields: fields,
            status: DocumentStatus::Active,
            schema_version_id: String::new(),
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        if let DocField::String(id) = &self.fields[DOC_ID_FIELD_INDEX] {
            Some(id)
        } else {
            None
        }
    }

    pub fn set_id(&mut self, doc_id: String) {
        self.fields[DOC_ID_FIELD_INDEX] = DocField::String(doc_id);
    }

    pub fn clone(&self) -> Self {
        let fields = self.fields.iter().map(|field| field.clone()).collect();

        Self {
            hidden: self.hidden,
            fields,
            status: self.status.clone(),
            schema_version_id: self.schema_version_id.clone(),
        }
    }
}

pub struct RenderKey {
    pub index: usize,
    pub key: String,
}

#[derive(Clone)]
pub struct MappingTypeInfo {
    pub index: usize,
    pub name: String,
}

pub struct DocumentMapping {
    pub type_info: Option<MappingTypeInfo>,
    pub render_keys: Vec<RenderKey>,
    pub indexes_by_name: HashMap<String, Vec<usize>>,
    pub next_index: usize,
    pub child_mappings: Vec<Option<DocumentMapping>>,
}

impl DocumentMapping {
    pub fn new() -> Self {
        Self {
            type_info: None,
            render_keys: Vec::new(),
            indexes_by_name: HashMap::new(),
            next_index: 0,
            child_mappings: Vec::new(),
        }
    }

    pub fn clone_without_render(&self) -> Self {
        let child_mappings = self
            .child_mappings
            .iter()
            .map(|child_opt| child_opt.as_ref().map(|child| child.clone_without_render()))
            .collect();

        Self {
            type_info: self.type_info.clone(),
            render_keys: Vec::new(),
            indexes_by_name: self.indexes_by_name.clone(),
            next_index: self.next_index,
            child_mappings,
        }
    }

    pub fn get_next_index(&self) -> usize {
        self.next_index
    }

    pub fn next_doc(&self) -> Doc {
        let mut doc = Doc::new();
        doc.fields = vec![DocField::Int(0); self.next_index];
        doc
    }

    // Set the first field of this name with the given value. Panics if the field does not exist.
    pub fn set_first_of_name(&mut self, doc: &mut Doc, name: &str, value: DocField) {
        if let Some(indexes) = self.indexes_by_name.get(name) {
            if let Some(&index) = indexes.first() {
                if index < doc.fields.len() {
                    doc.fields[index] = value;
                } else {
                    panic!("Field index out of bounds");
                }
            } else {
                panic!("Field name not found");
            }
        } else {
            panic!("Field name not found");
        }
    }

    // Tries to set the first field of this name with the given value.
    // Returns `true` if successful, `false` if the field does not exist.
    pub fn try_set_first_of_name(&mut self, doc: &mut Doc, name: &str, value: DocField) -> bool {
        if let Some(indexes) = self.indexes_by_name.get(name) {
            if let Some(&index) = indexes.first() {
                if index < doc.fields.len() {
                    doc.fields[index] = value;
                    return true;
                }
            }
        }
        false
    }

    // Returns the value of the first field of the given name. Panics if the field does not exist.
    pub fn first_of_name<'a>(&self, doc: &'a Doc, name: &str) -> &'a DocField {
        let index = self.first_index_of_name(name);
        &doc.fields[index]
    }

    // Returns the first field index of the given name. Panics if the field does not exist.
    pub fn first_index_of_name(&self, name: &str) -> usize {
        *self
            .indexes_by_name
            .get(name)
            .and_then(|indexes| indexes.first())
            .expect("Field name not found")
    }

    // Renders the given document to a `HashMap<String, DocField>` format using the given mapping.
    pub fn to_map(&self, doc: &Doc) -> HashMap<String, DocField> {
        let mut mapped_doc = HashMap::new();
        for render_key in &self.render_keys {
            if let Some(field) = doc.fields.get(render_key.index) {
                mapped_doc.insert(render_key.key.clone(), field.clone());
            }
            // Handle nested documents, types, and rendering logic as needed.
        }
        mapped_doc
    }

    // Appends the given index and name to the mapping.
    pub fn add(&mut self, index: usize, name: String) {
        let entry = self.indexes_by_name.entry(name).or_insert_with(Vec::new);
        entry.push(index);

        if index >= self.next_index {
            self.next_index = index + 1;
        }
    }

    // Sets the type name for this mapping.
    pub fn set_type_name(&mut self, type_name: String) {
        let index = self.get_next_index();
        self.add(index, "type_name".to_string()); // Assuming "type_name" is how you mark the type name field.
        self.type_info = Some(MappingTypeInfo {
            index,
            name: type_name,
        });
    }

    // Sets the given child mapping at the given index.
    pub fn set_child_at(&mut self, index: usize, child_mapping: DocumentMapping) {
        if index >= self.child_mappings.len() {
            self.child_mappings.resize_with(index + 1, Default::default);
        }
        self.child_mappings[index] = Some(child_mapping);
    }

    // Tries to find the name from the given index. Returns `Some(name)` if found, `None` otherwise.
    pub fn try_to_find_name_from_index(&self, target_index: usize) -> Option<String> {
        self.indexes_by_name.iter().find_map(|(name, indexes)| {
            if indexes.contains(&target_index) {
                Some(name.clone())
            } else {
                None
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_doc() {
        let doc = Doc::new();
        assert!(!doc.hidden);
        assert_eq!(doc.status, DocumentStatus::Active);
        assert_eq!(doc.fields.len(), DOC_ID_FIELD_INDEX + 1);
        assert!(doc.schema_version_id.is_empty());
    }

    #[test]
    fn test_set_get_id() {
        let mut doc = Doc::new();
        let id = "123".to_string();
        doc.set_id(id.clone());
        assert_eq!(doc.get_id(), Some(&id));
    }

    #[test]
    fn test_clone_doc() {
        let mut doc = Doc::new();
        doc.set_id("123".to_string());
        doc.fields.push(DocField::Bool(true));
        let cloned_doc = doc.clone();
        assert_eq!(doc.hidden, cloned_doc.hidden);
        assert_eq!(doc.status, cloned_doc.status);
        assert_eq!(doc.schema_version_id, cloned_doc.schema_version_id);
        assert_eq!(doc.fields.len(), cloned_doc.fields.len());
    }
}
