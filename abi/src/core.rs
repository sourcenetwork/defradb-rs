extern crate libc;

use serde_json;
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct FieldWrapper(pub core::doc::Field);

#[derive(Debug)]
pub struct DocWrapper(core::doc::Doc);

impl TryFrom<Value> for FieldWrapper {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(mut map) => {
                if let Some(_) = map.remove("Null") {
                    return Ok(FieldWrapper(core::doc::Field::Null));
                } else if let Some(v) = map.remove("Bool") {
                    if let Value::Bool(bool_val) = v {
                        return Ok(FieldWrapper(core::doc::Field::Bool(bool_val)));
                    }
                } else if let Some(v) = map.remove("Int") {
                    if let Value::Number(num) = v {
                        if let Some(int_val) = num.as_i64() {
                            return Ok(FieldWrapper(core::doc::Field::Int(int_val)));
                        }
                    }
                } else if let Some(v) = map.remove("Float") {
                    if let Value::Number(num) = v {
                        if let Some(float_val) = num.as_f64() {
                            return Ok(FieldWrapper(core::doc::Field::Float(float_val)));
                        }
                    }
                } else if let Some(v) = map.remove("String") {
                    if let Value::String(str_val) = v {
                        return Ok(FieldWrapper(core::doc::Field::String(str_val)));
                    }
                } else if let Some(v) = map.remove("DateTime") {
                    if let Value::String(str_val) = v {
                        let date_time = chrono::DateTime::parse_from_rfc3339(&str_val)
                            .map_err(|e| e.to_string())?;
                        return Ok(FieldWrapper(core::doc::Field::DateTime(date_time.into())));
                    }
                } else if let Some(doc_val) = map.remove("Doc") {
                    if let Value::Object(_) = doc_val {
                        let doc = DocWrapper::try_from(doc_val)?;
                        return Ok(FieldWrapper(core::doc::Field::Doc(doc.0)));
                    }
                } else if let Some(v) = map.remove("BoolArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for bool_val in arr {
                            if let Value::Bool(b) = bool_val {
                                res_vec.push(b);
                            } else {
                                return Err("Expected a bool in BoolArray".to_string());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::BoolArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("IntArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for int_val in arr {
                            if let Value::Number(num) = int_val {
                                if let Some(int_val) = num.as_i64() {
                                    res_vec.push(int_val);
                                } else {
                                    return Err("Expected an integer in IntArray".to_string());
                                }
                            } else {
                                return Err("Expected an integer in IntArray".to_string());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::IntArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("FloatArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for float_val in arr {
                            if let Value::Number(num) = float_val {
                                if let Some(float_val) = num.as_f64() {
                                    res_vec.push(float_val);
                                } else {
                                    return Err("Expected a float in FloatArray".to_string());
                                }
                            } else {
                                return Err("Expected a float in FloatArray".to_string());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::FloatArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("StringArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for str_val in arr {
                            if let Value::String(str) = str_val {
                                res_vec.push(str);
                            } else {
                                return Err("Expected a string in StringArray".to_string());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::StringArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("DateTimeArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for str_val in arr {
                            if let Value::String(str) = str_val {
                                let date_time = chrono::DateTime::parse_from_rfc3339(&str)
                                    .map_err(|e| e.to_string())?;
                                res_vec.push(date_time.into());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::DateTimeArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("DocArray") {
                    if let Value::Array(arr) = v {
                        let mut res_vec = Vec::new();
                        for doc_val in arr {
                            if let Value::Object(_) = doc_val {
                                if let Value::Object(_) = doc_val {
                                    res_vec.push(DocWrapper::try_from(doc_val)?.0);
                                }
                            } else {
                                return Err("Expected an object in DocArray".to_string());
                            }
                        }
                        return Ok(FieldWrapper(core::doc::Field::DocArray(res_vec)));
                    }
                } else if let Some(v) = map.remove("OptionalBool") {
                    if v.is_null() {
                        return Ok(FieldWrapper(core::doc::Field::OptionalBool(None)));
                    } else if let Value::Bool(b) = v {
                        return Ok(FieldWrapper(core::doc::Field::OptionalBool(Some(b))));
                    } else {
                        return Err("Expected a bool or null in OptionalBool".to_string());
                    }
                } else if let Some(v) = map.remove("OptionalInt") {
                    if v.is_null() {
                        return Ok(FieldWrapper(core::doc::Field::OptionalInt(None)));
                    } else if let Value::Number(num) = v {
                        if let Some(int_val) = num.as_i64() {
                            return Ok(FieldWrapper(core::doc::Field::OptionalInt(Some(int_val))));
                        } else {
                            return Err("Expected an integer in OptionalInt".to_string());
                        }
                    } else {
                        return Err("Expected an integer or null in OptionalInt".to_string());
                    }
                } else if let Some(v) = map.remove("OptionalFloat") {
                    if v.is_null() {
                        return Ok(FieldWrapper(core::doc::Field::OptionalFloat(None)));
                    } else if let Value::Number(num) = v {
                        if let Some(float_val) = num.as_f64() {
                            return Ok(FieldWrapper(core::doc::Field::OptionalFloat(Some(
                                float_val,
                            ))));
                        } else {
                            return Err("Expected a float in OptionalFloat".to_string());
                        }
                    } else {
                        return Err("Expected a float or null in OptionalFloat".to_string());
                    }
                } else if let Some(v) = map.remove("OptionalString") {
                    if v.is_null() {
                        return Ok(FieldWrapper(core::doc::Field::OptionalString(None)));
                    } else if let Value::String(str_val) = v {
                        return Ok(FieldWrapper(core::doc::Field::OptionalString(Some(
                            str_val,
                        ))));
                    } else {
                        return Err("Expected a string or null in OptionalString".to_string());
                    }
                } else if let Some(v) = map.remove("OptionalDateTime") {
                    if v.is_null() {
                        return Ok(FieldWrapper(core::doc::Field::OptionalDateTime(None)));
                    } else if let Value::String(str_val) = v {
                        let date_time = chrono::DateTime::parse_from_rfc3339(&str_val)
                            .map_err(|e| e.to_string())?;
                        return Ok(FieldWrapper(core::doc::Field::OptionalDateTime(Some(
                            date_time.into(),
                        ))));
                    } else {
                        return Err("Expected a string or null in OptionalDateTime".to_string());
                    }
                }
                Err("Invalid DocField JSON structure".to_string())
            }
            Value::Null => Ok(FieldWrapper(core::doc::Field::Null)),
            _ => Err("Expected a JSON object for DocField".to_string()),
        }
    }
}

impl TryFrom<Value> for DocWrapper {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let mut doc = core::doc::Doc::new();
                doc.fields.clear();
                if let Some(fields_val) = map.get("fields") {
                    if let Value::Array(fields_array) = fields_val {
                        doc.fields.reserve(fields_array.len());
                        for field_val in fields_array {
                            let field = FieldWrapper::try_from(field_val.clone())?;
                            doc.fields.push(field.0);
                        }
                    } else {
                        return Err("Expected an array of fields".to_string());
                    }
                } else {
                    return Err("'fields' key not found in Doc JSON".to_string());
                }
                Ok(DocWrapper(doc))
            }
            _ => Err("Expected a JSON object for Doc".to_string()),
        }
    }
}

pub fn deserialize_doc(json_str: &str) -> Result<core::doc::Doc, String> {
    let v: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let doc = DocWrapper::try_from(v).map_err(|e| e.to_string())?;
    Ok(doc.0)
}

pub fn deserialize_field(json_str: &str) -> Result<core::doc::Field, String> {
    let v: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let field = FieldWrapper::try_from(v).map_err(|e| e.to_string())?;
    Ok(field.0)
}
