use std::clone::Clone;
use std::collections::HashMap;

use super::doc::{Doc, Field};

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
        doc.fields = vec![Field::Int(0); self.next_index];
        doc
    }

    // Set the first field of this name with the given value. Panics if the field does not exist.
    pub fn set_first_of_name(&mut self, doc: &mut Doc, name: &str, value: Field) {
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
    pub fn try_set_first_of_name(&mut self, doc: &mut Doc, name: &str, value: Field) -> bool {
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
    pub fn first_of_name<'a>(&self, doc: &'a Doc, name: &str) -> &'a Field {
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
    pub fn to_map(&self, doc: &Doc) -> HashMap<String, Field> {
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
