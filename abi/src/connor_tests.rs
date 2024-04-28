// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

/*
We tried to move this tests into `tests` folder but the new test files just wouldn't see abi
crate or nested modules. So we leave it like this for now until we figure out how to make it work.
*/
extern crate libc;

use crate::connor::*;
use crate::util;
use connor::*;
use libc::c_char;

const SIMPLE_EQ_COND_JSON: &str = r#"
{
    "PROP": {
        "index": 4,
        "condition": {
            "EQ": {
                "Int": 10
            }
        }
    }
}"#;

#[test]
fn deserialize_simple_cond() {
    let result = deserialize_condition(SIMPLE_EQ_COND_JSON);

    assert!(result.is_ok());
    let cond = result.unwrap();

    match cond {
        cond::Condition::Prop(index, condition) => {
            assert_eq!(index, 4);
            match &*condition {
                cond::Condition::Op(op, field) => {
                    assert_eq!(*op, cond::Op::EQ);
                    if let core::doc::Field::Int(int_val) = field {
                        assert_eq!(*int_val, 10);
                    } else {
                        panic!("Expected Int");
                    }
                }
                _ => panic!("Expected Op"),
            }
        }
        _ => panic!("Expected CompoundOp"),
    }
}

const NESTED_PROPS_JSON: &str = r#"
{
    "AND": [
        {
            "PROP": {
                "index": 0,
                "condition": {
                    "EQ": {
                        "Int": 10
                    }
                }
            }
        },
        {
            "PROP": {
                "index": 1,
                "condition": {
                    "PROP": {
                        "index": 0,
                        "condition": {
                            "IN": {
                                "IntArray": [1, 2, 3]
                            }
                        }
                    }
                }
            }
        }
    ]
}"#;

#[test]
fn deserialize_cond_with_nested_props() {
    let result = deserialize_condition(NESTED_PROPS_JSON);

    assert!(result.is_ok());
    let cond = result.unwrap();

    match cond {
        cond::Condition::CompoundOp(op, conditions) => {
            assert_eq!(op, cond::CompoundOp::AND);
            assert_eq!(conditions.len(), 2);
            match &conditions[0] {
                cond::Condition::Prop(index, condition) => {
                    assert_eq!(*index, 0);
                    match &**condition {
                        cond::Condition::Op(op, field) => {
                            assert_eq!(*op, cond::Op::EQ);
                            if let core::doc::Field::Int(int_val) = field {
                                assert_eq!(*int_val, 10);
                            } else {
                                panic!("Expected Int");
                            }
                        }
                        _ => panic!("Expected Op"),
                    }
                }
                _ => panic!("Expected Prop"),
            }
            match &conditions[1] {
                cond::Condition::Prop(index, condition) => {
                    assert_eq!(*index, 1);
                    match &**condition {
                        cond::Condition::Prop(index, condition) => {
                            assert_eq!(*index, 0);
                            match &**condition {
                                cond::Condition::Op(op, field) => {
                                    assert_eq!(*op, cond::Op::IN);
                                    if let core::doc::Field::IntArray(int_arr) = field {
                                        assert_eq!(int_arr, &[1, 2, 3]);
                                    } else {
                                        panic!("Expected IntArray");
                                    }
                                }
                                _ => panic!("Expected Op"),
                            }
                        }
                        _ => panic!("Expected Prop"),
                    }
                }
                _ => panic!("Expected Prop"),
            }
        }
        _ => panic!("Expected CompoundOp"),
    }
}

const AND_COND_JSON: &str = r#"
{
    "AND":[
       {
          "PROP":{
             "condition":{
                "NE":{
                   "String":"bae-3bfe0092-e31f-5ebe-a3ba-fa18fac448a6"
                }
             },
             "index":0
          }
       },
       {
          "PROP":{
             "condition":{
                "EQ":{
                   "String":"bae-fd541c25-229e-5280-b44b-e5c2af3e374d"
                }
             },
             "index":4
          }
       }
    ]
 }"#;

const AUTHOR_DOC_WITH_PROPS_JSON: &str = r#"{
    "fields":[
       {
          "String":"bae-3bfe0092-e31f-5ebe-a3ba-fa18fac448a6"
       },
       {
          "Int":65
       },
       {
          "String":"John Grisham"
       },
       null,
       null,
       null,
       null,
       null,
       null
    ]
 }"#;

#[test]
fn match_doc_against_and_cond() {
    let result = match_conditions_with(AND_COND_JSON, AUTHOR_DOC_WITH_PROPS_JSON);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

fn assert_no_error(data: *const c_char) {
    let mut is_modified = false;
    for i in 0..256 {
        if unsafe { *data.add(i) } != 0 {
            is_modified = true;
            break;
        }
    }
    if is_modified {
        let c_str = unsafe { std::ffi::CStr::from_ptr(data) };
        let output_str = c_str.to_string_lossy().to_string();
        panic!(
            "Expected buffer with no error, but found \"{}\"",
            output_str
        );
    }
}

#[test]
fn match_conditions_through_abi_no_match() {
    let mut out = util::StringBuffer::new();

    let cond_str = SIMPLE_EQ_COND_JSON.to_string();
    let doc_str = AUTHOR_DOC_WITH_PROPS_JSON.to_string();

    let result = match_conditions(
        cond_str.as_ptr() as *const c_char,
        doc_str.as_ptr() as *const c_char,
        &mut out,
    );

    assert_no_error(out.data());

    assert!(!result);
}

const EQ_AND_NE_COND_JSON: &str = r#" {
"AND":[
   {
      "PROP":{
         "condition":{
            "NE":{
               "String":"bae-edeade01-2d21-5d6d-aadf-efc5a5279de5"
            }
         },
         "index":0
      }
   },
   {
      "PROP":{
         "condition":{
            "EQ":{
               "String":"bae-807ea028-6c13-5f86-a72b-46e8b715a162"
            }
         },
         "index":2
      }
   }
]}"#;

const DOC_WITH_WITH_2_STRINGS_JSON: &str = r#"{
    "fields":[
       {
          "String":"bae-78a40f28-a4b8-5dca-be44-392b0f96d0ff"
       },
       null,
       {
          "String":"bae-807ea028-6c13-5f86-a72b-46e8b715a162"
       },
       null,
       null,
       null,
       null
    ]
 }"#;

#[test]
fn match_conditions_through_abi_match() {
    let mut out = util::StringBuffer::new();

    let cond_str = EQ_AND_NE_COND_JSON.to_string();
    let doc_str = DOC_WITH_WITH_2_STRINGS_JSON.to_string();

    let result = match_conditions(
        cond_str.as_ptr() as *const c_char,
        doc_str.as_ptr() as *const c_char,
        &mut out,
    );

    assert_no_error(out.data());

    assert!(result);
}

const COND_1_JSON: &str = r#"{"GT":{"Int":0},"NE":null}"#;
const FIELD_1_JSON: &str = r#"{"Int":2}"#;

#[test]
fn match_conditions_against_field_match() {
    let result = match_conditions_with(COND_1_JSON, FIELD_1_JSON);

    assert!(result.is_ok());
    assert!(result.unwrap());
}

const COND_2_JSON: &str = r#"{"LT":{"Float":9}}"#;
const FIELD_2_JSON: &str = r#"null"#;

#[test]
fn match_conditions_against_field_match_no_match() {
    let result = match_conditions_with(COND_2_JSON, FIELD_2_JSON);

    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn match_conditions_with_null_op() {
    for op in ["NOT", "AND", "OR"] {
        let cond_str = format!("{{\"{}\":null}}", op);
        let result = match_conditions_with(&cond_str, FIELD_1_JSON);

        assert!(result.is_err(), "Expected error for op: {}", op);
    }
}
