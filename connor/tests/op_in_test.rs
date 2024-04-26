mod common;

use chrono::*;
use common::*;
use connor::cond::*;
use core::doc::Field;

#[test]
fn test_in_bool() {
    let cond = Condition::Op(Op::IN, Field::BoolArray(vec![true]));
    expect(match_conditions(&cond, &Field::Bool(true)), true, false);
    expect(match_conditions(&cond, &Field::Bool(false)), false, false);
}

#[test]
fn test_in_int() {
    let cond = Condition::Op(Op::IN, Field::IntArray(vec![5, 6, 7]));
    expect(match_conditions(&cond, &Field::Int(6)), true, false);
    expect(match_conditions(&cond, &Field::Int(4)), false, false);
}

#[test]
fn test_in_float() {
    let cond = Condition::Op(Op::IN, Field::FloatArray(vec![5.0, 6.0, 7.0]));
    expect(match_conditions(&cond, &Field::Float(6.0)), true, false);
    expect(match_conditions(&cond, &Field::Float(4.0)), false, false);
}

#[test]
fn test_in_str() {
    let in_field = Field::StringArray(vec![
        "str1".to_string(),
        "str2".to_string(),
        "str3".to_string(),
    ]);
    let cond = Condition::Op(Op::IN, in_field);
    expect(
        match_conditions(&cond, &Field::String("str2".to_string())),
        true,
        false,
    );
    expect(
        match_conditions(&cond, &Field::String("str4".to_string())),
        false,
        false,
    );
}

#[test]
fn test_in_time() {
    let cond = Condition::Op(
        Op::IN,
        Field::DateTimeArray(vec![now_time(), yesterday_time()]),
    );
    expect(match_conditions(&cond, &yesterday()), true, false);

    let tomorrow = Field::DateTime(now_time() + Duration::days(1));
    expect(match_conditions(&cond, &tomorrow), false, false);
}

#[test]
fn test_in_optional_bool() {
    let cond = Condition::Op(Op::IN, Field::OptionalBoolArray(vec![Some(true), None]));
    expect(match_conditions(&cond, &Field::Bool(true)), true, false);
    expect(match_conditions(&cond, &Field::Null), true, false);
    expect(match_conditions(&cond, &Field::Bool(false)), false, false);
}

#[test]
fn test_in_optional_int() {
    let cond = Condition::Op(
        Op::IN,
        Field::OptionalIntArray(vec![Some(5), None, Some(6)]),
    );
    expect(match_conditions(&cond, &Field::Int(6)), true, false);
    expect(match_conditions(&cond, &Field::Null), true, false);
    expect(match_conditions(&cond, &Field::Int(4)), false, false);
}

#[test]
fn test_in_optional_float() {
    let cond = Condition::Op(
        Op::IN,
        Field::OptionalFloatArray(vec![Some(5.0), None, Some(6.0)]),
    );
    expect(match_conditions(&cond, &Field::Float(6.0)), true, false);
    expect(match_conditions(&cond, &Field::Null), true, false);
    expect(match_conditions(&cond, &Field::Float(4.0)), false, false);
}

#[test]
fn test_in_optional_str() {
    let cond = Condition::Op(
        Op::IN,
        Field::OptionalStringArray(vec![
            Some("str1".to_string()),
            None,
            Some("str2".to_string()),
        ]),
    );
    expect(
        match_conditions(&cond, &Field::String("str2".to_string())),
        true,
        false,
    );
    expect(match_conditions(&cond, &Field::Null), true, false);
    expect(
        match_conditions(&cond, &Field::String("str4".to_string())),
        false,
        false,
    );
}

#[test]
fn test_in_optional_time() {
    let cond = Condition::Op(
        Op::IN,
        Field::OptionalDateTimeArray(vec![Some(now_time()), None]),
    );
    expect(match_conditions(&cond, &now()), true, false);
    expect(match_conditions(&cond, &Field::Null), true, false);
    expect(match_conditions(&cond, &yesterday()), false, false);
}
