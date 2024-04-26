#![allow(dead_code)]

use chrono::*;
use connor::*;
use core::doc::Field;

pub fn now_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc
        .with_ymd_and_hms(2024, 4, 8, 12, 34, 56)
        .unwrap()
}

pub fn now() -> Field {
    Field::DateTime(now_time())
}

pub fn yesterday_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc
        .with_ymd_and_hms(2024, 4, 7, 12, 34, 56)
        .unwrap()
}

pub fn yesterday() -> Field {
    Field::DateTime(yesterday_time())
}

pub fn get_fields() -> Vec<Field> {
    vec![
        Field::Int(5),
        Field::String("str".to_string()),
        Field::Float(1.2),
        Field::Bool(true),
        now(),
    ]
}

pub fn get_field(index: usize) -> Field {
    get_fields()[index].clone()
}

pub fn get_doc() -> core::doc::Doc {
    core::doc::Doc {
        hidden: false,
        status: core::doc::Status::Active,
        fields: get_fields(),
        schema_version_id: "".to_string(),
    }
}

pub fn expect(actual: Result<bool, error::Error>, expect_match: bool, expect_error: bool) {
    match actual {
        Ok(true) => assert!(expect_match, "Expected match"),
        Ok(false) => assert!(!expect_match, "Expected no match"),
        _ => {
            if !expect_error {
                panic!("Expected no error")
            }
        }
    }
}

pub fn test_op(doc_field: Field, op: cond::Op, target_field: Field, expect_match: bool) {
    let cond = cond::Condition::Op(op, target_field);
    expect(
        cond::match_conditions(&cond, &doc_field),
        expect_match,
        false,
    );
}
