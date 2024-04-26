// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

use chrono;
use std::clone::Clone;

pub const DOC_ID_FIELD_INDEX: usize = 0;

#[derive(Clone, Debug)]
pub enum Field {
    Null,
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
    OptionalBool(Option<bool>),
    OptionalInt(Option<i64>),
    OptionalFloat(Option<f64>),
    OptionalString(Option<String>),
    OptionalDateTime(Option<chrono::DateTime<chrono::Utc>>),
    OptionalBoolArray(Vec<Option<bool>>),
    OptionalIntArray(Vec<Option<i64>>),
    OptionalFloatArray(Vec<Option<f64>>),
    OptionalStringArray(Vec<Option<String>>),
    OptionalDateTimeArray(Vec<Option<chrono::DateTime<chrono::Utc>>>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Status {
    Active,
    Deleted,
}

#[derive(Clone, Debug)]
pub struct Doc {
    pub hidden: bool,
    pub fields: Vec<Field>,
    pub status: Status,
    pub schema_version_id: String,
}

impl Doc {
    pub fn new() -> Self {
        let mut fields = Vec::new();
        fields.resize(DOC_ID_FIELD_INDEX + 1, Field::String("".to_string()));

        Self {
            hidden: false,
            fields: fields,
            status: Status::Active,
            schema_version_id: String::new(),
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        if let Field::String(id) = &self.fields[DOC_ID_FIELD_INDEX] {
            Some(id)
        } else {
            None
        }
    }

    pub fn set_id(&mut self, doc_id: String) {
        self.fields[DOC_ID_FIELD_INDEX] = Field::String(doc_id);
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
