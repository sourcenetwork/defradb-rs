// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

use core::doc;

#[test]
fn test_new_doc() {
    let doc = doc::Doc::new();
    assert!(!doc.hidden);
    assert_eq!(doc.status, doc::Status::Active);
    assert_eq!(doc.fields.len(), doc::DOC_ID_FIELD_INDEX + 1);
    assert!(doc.schema_version_id.is_empty());
}

#[test]
fn test_set_get_id() {
    let mut doc = doc::Doc::new();
    let id = "123".to_string();
    doc.set_id(id.clone());
    assert_eq!(doc.get_id(), Some(&id));
}

#[test]
fn test_clone_doc() {
    let mut doc = doc::Doc::new();
    doc.set_id("123".to_string());
    doc.fields.push(doc::Field::Bool(true));
    let cloned_doc = doc.clone();
    assert_eq!(doc.hidden, cloned_doc.hidden);
    assert_eq!(doc.status, cloned_doc.status);
    assert_eq!(doc.schema_version_id, cloned_doc.schema_version_id);
    assert_eq!(doc.fields.len(), cloned_doc.fields.len());
}
