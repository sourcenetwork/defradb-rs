use super::cond::*;
use super::*;

use chrono::*;
use core::doc::Field;

fn now_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc
        .with_ymd_and_hms(2024, 4, 8, 12, 34, 56)
        .unwrap()
}

fn now() -> Field {
    Field::DateTime(now_time())
}

fn yesterday_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc
        .with_ymd_and_hms(2024, 4, 7, 12, 34, 56)
        .unwrap()
}

fn yesterday() -> Field {
    Field::DateTime(yesterday_time())
}

fn get_fields() -> Vec<Field> {
    vec![
        Field::Int(5),
        Field::String("str".to_string()),
        Field::Float(1.2),
        Field::Bool(true),
        now(),
    ]
}

fn get_field(index: usize) -> Field {
    get_fields()[index].clone()
}

fn get_doc() -> core::doc::Doc {
    core::doc::Doc {
        hidden: false,
        status: core::doc::Status::Active,
        fields: get_fields(),
        schema_version_id: "".to_string(),
    }
}

fn expect(actual: Result<bool, err::Error>, expect_match: bool, expect_error: bool) {
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

fn test_op(doc_field: Field, op: Op, target_field: Field, expect_match: bool) {
    let cond = Condition::Op(op, target_field);
    expect(match_conditions(&cond, &doc_field), expect_match, false);
}

#[test]
fn test_eq_int() {
    test_op(Field::Int(5), Op::EQ, Field::Int(5), true);
    test_op(Field::Int(4), Op::EQ, Field::Int(5), false);
}

#[test]
fn test_eq_opt_int() {
    test_op(Field::OptionalInt(Some(5)), Op::EQ, Field::Int(5), true);
    test_op(Field::OptionalInt(None), Op::EQ, Field::Int(5), false);
    test_op(Field::OptionalInt(Some(5)), Op::EQ, Field::Int(4), false);
}

#[test]
fn test_eq_str() {
    test_op(
        Field::String("str".to_string()),
        Op::EQ,
        Field::String("str".to_string()),
        true,
    );
    test_op(
        Field::String("str".to_string()),
        Op::EQ,
        Field::String("wrong".to_string()),
        false,
    );
}

#[test]
fn test_eq_opt_str() {
    test_op(
        Field::OptionalString(Some("str".to_string())),
        Op::EQ,
        Field::String("str".to_string()),
        true,
    );
    test_op(
        Field::OptionalString(None),
        Op::EQ,
        Field::String("str".to_string()),
        false,
    );
    test_op(
        Field::OptionalString(Some("str".to_string())),
        Op::EQ,
        Field::String("wrong".to_string()),
        false,
    );
}

#[test]
fn test_eq_float() {
    test_op(Field::Float(5.0), Op::EQ, Field::Float(5.0), true);
    test_op(Field::Float(5.5), Op::EQ, Field::Float(5.0), false);
}

#[test]
fn test_eq_opt_float() {
    test_op(
        Field::OptionalFloat(Some(5.0)),
        Op::EQ,
        Field::Float(5.0),
        true,
    );
    test_op(Field::OptionalFloat(None), Op::EQ, Field::Float(5.0), false);
    test_op(
        Field::OptionalFloat(Some(5.0)),
        Op::EQ,
        Field::Float(4.0),
        false,
    );
}

#[test]
fn test_eq_float_int() {
    test_op(Field::Float(4.0), Op::EQ, Field::Int(4), true);
    test_op(Field::Int(4), Op::EQ, Field::Float(4.0), true);
}

#[test]
fn test_eq_bool() {
    test_op(Field::Bool(true), Op::EQ, Field::Bool(true), true);
    test_op(Field::Bool(false), Op::EQ, Field::Bool(true), false);
}

#[test]
fn test_eq_opt_bool() {
    test_op(
        Field::OptionalBool(Some(true)),
        Op::EQ,
        Field::Bool(true),
        true,
    );
    test_op(Field::OptionalBool(None), Op::EQ, Field::Bool(true), false);
    test_op(
        Field::OptionalBool(Some(true)),
        Op::EQ,
        Field::Bool(false),
        false,
    );
}

#[test]
fn test_eq_time() {
    test_op(now(), Op::EQ, now(), true);
    test_op(yesterday(), Op::EQ, now(), false);
}

#[test]
fn test_eq_opt_time() {
    test_op(
        Field::OptionalDateTime(Some(now_time())),
        Op::EQ,
        now(),
        true,
    );
    test_op(Field::OptionalDateTime(None), Op::EQ, now(), false);
    test_op(
        Field::OptionalDateTime(Some(now_time())),
        Op::EQ,
        yesterday(),
        false,
    );
}

#[test]
fn test_eq_null() {
    test_op(Field::Int(5), Op::EQ, Field::Null, false);
    test_op(Field::Null, Op::EQ, Field::Int(5), false);
    test_op(Field::Null, Op::EQ, Field::Null, true);
}

#[test]
fn test_eq_bool_arr() {
    test_op(
        Field::BoolArray(vec![true, false, true]),
        Op::EQ,
        Field::BoolArray(vec![true, false, true]),
        true,
    );
    test_op(
        Field::BoolArray(vec![true, false, true]),
        Op::EQ,
        Field::BoolArray(vec![false, true, true]),
        false,
    );
}

#[test]
fn test_eq_int_arr() {
    test_op(
        Field::IntArray(vec![2, 3, 4]),
        Op::EQ,
        Field::IntArray(vec![2, 3, 4]),
        true,
    );
    test_op(
        Field::IntArray(vec![2, 4, 3]),
        Op::EQ,
        Field::IntArray(vec![2, 3, 4]),
        false,
    );
    test_op(
        Field::IntArray(vec![2, 3]),
        Op::EQ,
        Field::IntArray(vec![2, 3, 4]),
        false,
    );
}

#[test]
fn test_eq_float_arr() {
    test_op(
        Field::FloatArray(vec![2.0, 3.0, 4.0]),
        Op::EQ,
        Field::FloatArray(vec![2.0, 3.0, 4.0]),
        true,
    );
    test_op(
        Field::FloatArray(vec![2.0, 4.0, 3.0]),
        Op::EQ,
        Field::FloatArray(vec![2.0, 3.0, 4.0]),
        false,
    );
}

#[test]
fn test_eq_string_arr() {
    test_op(
        Field::StringArray(vec!["a".to_string(), "b".to_string()]),
        Op::EQ,
        Field::StringArray(vec!["a".to_string(), "b".to_string()]),
        true,
    );
    test_op(
        Field::StringArray(vec!["a".to_string(), "b".to_string()]),
        Op::EQ,
        Field::StringArray(vec!["b".to_string(), "a".to_string()]),
        false,
    );
}

#[test]
fn test_eq_time_arr() {
    test_op(
        Field::DateTimeArray(vec![now_time(), yesterday_time()]),
        Op::EQ,
        Field::DateTimeArray(vec![now_time(), yesterday_time()]),
        true,
    );
    test_op(
        Field::DateTimeArray(vec![now_time(), yesterday_time()]),
        Op::EQ,
        Field::DateTimeArray(vec![yesterday_time(), now_time()]),
        false,
    );
}

#[test]
fn test_eq_optional_bool_arr() {
    test_op(
        Field::OptionalBoolArray(vec![Some(true), None, Some(false)]),
        Op::EQ,
        Field::OptionalBoolArray(vec![Some(true), None, Some(false)]),
        true,
    );
    test_op(
        Field::OptionalBoolArray(vec![Some(true), Some(false), None]),
        Op::EQ,
        Field::OptionalBoolArray(vec![Some(true), Some(false), Some(false)]),
        false,
    );
}

#[test]
fn test_eq_optional_int_arr() {
    test_op(
        Field::OptionalIntArray(vec![Some(2), None, Some(4)]),
        Op::EQ,
        Field::OptionalIntArray(vec![Some(2), None, Some(4)]),
        true,
    );
    test_op(
        Field::OptionalIntArray(vec![Some(2), Some(4), Some(3)]),
        Op::EQ,
        Field::OptionalIntArray(vec![Some(2), Some(3), Some(4)]),
        false,
    );
    test_op(
        Field::OptionalIntArray(vec![Some(2), Some(3)]),
        Op::EQ,
        Field::OptionalIntArray(vec![Some(2), Some(3), Some(4)]),
        false,
    );
    test_op(
        Field::OptionalIntArray(vec![Some(2), Some(3), None]),
        Op::EQ,
        Field::OptionalIntArray(vec![Some(2), Some(3), Some(0)]),
        false,
    );
}

#[test]
fn test_eq_optional_float_arr() {
    test_op(
        Field::OptionalFloatArray(vec![Some(2.0), None, Some(3.0)]),
        Op::EQ,
        Field::OptionalFloatArray(vec![Some(2.0), None, Some(3.0)]),
        true,
    );
    test_op(
        Field::OptionalFloatArray(vec![Some(2.0), Some(3.0), None]),
        Op::EQ,
        Field::OptionalFloatArray(vec![Some(2.0), Some(3.0), Some(0.0)]),
        false,
    );
}

#[test]
fn test_eq_optional_string_arr() {
    test_op(
        Field::OptionalStringArray(vec![Some("a".to_string()), None, Some("b".to_string())]),
        Op::EQ,
        Field::OptionalStringArray(vec![Some("a".to_string()), None, Some("b".to_string())]),
        true,
    );
    test_op(
        Field::OptionalStringArray(vec![Some("a".to_string()), Some("b".to_string()), None]),
        Op::EQ,
        Field::OptionalStringArray(vec![
            Some("a".to_string()),
            Some("b".to_string()),
            Some("".to_string()),
        ]),
        false,
    );
}

#[test]
fn test_eq_optional_time_arr() {
    test_op(
        Field::OptionalDateTimeArray(vec![Some(now_time()), None, Some(yesterday_time())]),
        Op::EQ,
        Field::OptionalDateTimeArray(vec![Some(now_time()), None, Some(yesterday_time())]),
        true,
    );
    test_op(
        Field::OptionalDateTimeArray(vec![Some(now_time()), Some(yesterday_time()), None]),
        Op::EQ,
        Field::OptionalDateTimeArray(vec![
            Some(now_time()),
            Some(yesterday_time()),
            Some(Utc.timestamp_opt(0, 0).unwrap()),
        ]),
        false,
    );
}

#[test]
fn test_ne() {
    test_op(Field::Int(5), Op::NE, Field::Int(5), false);
    test_op(Field::Int(4), Op::NE, Field::Int(5), true);
}

#[test]
fn test_ne_null() {
    test_op(Field::Null, Op::NE, Field::Int(5), true);
    test_op(Field::Int(5), Op::NE, Field::Null, true);
    test_op(Field::Null, Op::NE, Field::Null, false);
}

#[test]
fn test_gt_int() {
    test_op(Field::Int(5), Op::GT, Field::Int(4), true);
    test_op(Field::Int(5), Op::GT, Field::Int(5), false);
    test_op(Field::Int(4), Op::GT, Field::Int(5), false);
}

#[test]
fn test_gt_float() {
    test_op(Field::Float(5.0), Op::GT, Field::Float(4.0), true);
    test_op(Field::Float(5.0), Op::GT, Field::Float(5.0), false);
    test_op(Field::Float(4.0), Op::GT, Field::Float(5.0), false);
}

#[test]
fn test_gt_float_int() {
    test_op(Field::Float(5.0), Op::GT, Field::Int(4), true);
    test_op(Field::Float(5.5), Op::GT, Field::Int(5), true);
    test_op(Field::Int(5), Op::GT, Field::Float(4.0), true);
    test_op(Field::Int(5), Op::GT, Field::Float(4.5), true);
}

#[test]
fn test_gt_time() {
    test_op(now(), Op::GT, yesterday(), true);
    test_op(now(), Op::GT, now(), false);
    test_op(yesterday(), Op::GT, now(), false);
}

#[test]
fn test_gt_null() {
    test_op(Field::Null, Op::GT, Field::Int(5), false);
    test_op(Field::Int(5), Op::GT, Field::Null, false);
}

#[test]
fn test_ge_int() {
    test_op(Field::Int(6), Op::GE, Field::Int(5), true);
    test_op(Field::Int(5), Op::GE, Field::Int(5), true);
    test_op(Field::Int(5), Op::GE, Field::Int(6), false);
}

#[test]
fn test_ge_float() {
    test_op(Field::Float(6.0), Op::GE, Field::Float(5.0), true);
    test_op(Field::Float(5.0), Op::GE, Field::Float(5.0), true);
    test_op(Field::Float(5.0), Op::GE, Field::Float(6.0), false);
}

#[test]
fn test_ge_float_int() {
    test_op(Field::Float(5.0), Op::GE, Field::Int(5), true);
    test_op(Field::Float(5.5), Op::GE, Field::Int(5), true);
    test_op(Field::Int(5), Op::GE, Field::Float(5.0), true);
    test_op(Field::Int(5), Op::GE, Field::Float(4.5), true);
}

#[test]
fn test_ge_time() {
    test_op(now(), Op::GE, yesterday(), true);
    test_op(now(), Op::GE, now(), true);
    test_op(yesterday(), Op::GE, now(), false);
}

#[test]
fn test_ge_null() {
    test_op(Field::Null, Op::GE, Field::Int(5), false);
    test_op(Field::Int(5), Op::GE, Field::Null, false);
}

#[test]
fn test_lt_int() {
    test_op(Field::Int(4), Op::LT, Field::Int(5), true);
    test_op(Field::Int(5), Op::LT, Field::Int(5), false);
    test_op(Field::Int(5), Op::LT, Field::Int(4), false);
}

#[test]
fn test_lt_float() {
    test_op(Field::Float(4.0), Op::LT, Field::Float(5.0), true);
    test_op(Field::Float(5.0), Op::LT, Field::Float(5.0), false);
    test_op(Field::Float(5.0), Op::LT, Field::Float(4.0), false);
}

#[test]
fn test_lt_float_int() {
    test_op(Field::Float(4.0), Op::LT, Field::Int(5), true);
    test_op(Field::Float(4.5), Op::LT, Field::Int(5), true);
    test_op(Field::Int(4), Op::LT, Field::Float(4.5), true);
}

#[test]
fn test_lt_time() {
    test_op(yesterday(), Op::LT, now(), true);
    test_op(now(), Op::LT, now(), false);
    test_op(now(), Op::LT, yesterday(), false);
}

#[test]
fn test_lt_null() {
    test_op(Field::Null, Op::LT, Field::Int(5), false);
    test_op(Field::Int(5), Op::LT, Field::Null, false);
}

#[test]
fn test_le_int() {
    test_op(Field::Int(5), Op::LE, Field::Int(6), true);
    test_op(Field::Int(5), Op::LE, Field::Int(5), true);
    test_op(Field::Int(6), Op::LE, Field::Int(5), false);
}

#[test]
fn test_le_float() {
    test_op(Field::Float(5.0), Op::LE, Field::Float(5.5), true);
    test_op(Field::Float(5.0), Op::LE, Field::Float(5.0), true);
    test_op(Field::Float(5.5), Op::LE, Field::Float(5.0), false);
}

#[test]
fn test_le_float_int() {
    test_op(Field::Float(5.0), Op::LE, Field::Int(5), true);
    test_op(Field::Float(4.5), Op::LE, Field::Int(5), true);
    test_op(Field::Int(5), Op::LE, Field::Float(5.0), true);
    test_op(Field::Int(5), Op::LE, Field::Float(5.5), true);
}

#[test]
fn test_le_time() {
    test_op(yesterday(), Op::LE, now(), true);
    test_op(now(), Op::LE, now(), true);
    test_op(now(), Op::LE, yesterday(), false);
}

#[test]
fn test_le_null() {
    test_op(Field::Null, Op::LE, Field::Int(5), false);
    test_op(Field::Int(5), Op::LE, Field::Null, false);
}

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

#[test]
fn test_nin() {
    let cond = Condition::Op(Op::NIN, Field::IntArray(vec![5, 6, 7]));
    expect(match_conditions(&cond, &Field::Int(4)), true, false);
    expect(match_conditions(&cond, &Field::Int(6)), false, false);
}

#[test]
fn test_like() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::LIKE, field.clone());
    expect(match_conditions(&cond, &field), true, false);

    // not exact match
    let cond = Condition::Op(Op::LIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match prefix
    let cond = Condition::Op(Op::LIKE, Field::String("Source%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match suffix
    let cond = Condition::Op(Op::LIKE, Field::String("%web3".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match contains
    let cond = Condition::Op(Op::LIKE, Field::String("%glue%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match start and end with
    let cond = Condition::Op(Op::LIKE, Field::String("Source%web3".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // case sensitive no match
    let field = Field::String("SOURCE is the glue of web3".to_string());
    expect(match_conditions(&cond, &field), false, false);

    // case sensitive no prefix match
    let field = Field::String("SOURCE%".to_string());
    expect(match_conditions(&cond, &field), false, false);
}

#[test]
fn test_like_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::LIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), false, false);

    let cond = Condition::Op(Op::LIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), false, false);
}

#[test]
fn test_nlike() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::NLIKE, field.clone());
    expect(match_conditions(&cond, &field), false, false);

    // not exact match
    let cond = Condition::Op(Op::NLIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match prefix
    let cond = Condition::Op(Op::NLIKE, Field::String("Source%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match suffix
    let cond = Condition::Op(Op::NLIKE, Field::String("%web3".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match contains
    let cond = Condition::Op(Op::NLIKE, Field::String("%glue%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match start and end with
    let cond = Condition::Op(Op::NLIKE, Field::String("Source%web3".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // case sensitive no match
    let field = Field::String("SOURCE is the glue of web3".to_string());
    expect(match_conditions(&cond, &field), true, false);

    // case sensitive no prefix match
    let field = Field::String("SOURCE%".to_string());
    expect(match_conditions(&cond, &field), true, false);
}

#[test]
fn test_nlike_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::NLIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), true, false);

    let cond = Condition::Op(Op::NLIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), true, false);
}

#[test]
fn test_ilike() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::ILIKE, field.clone());
    expect(match_conditions(&cond, &field), true, false);

    // not exact match
    let cond = Condition::Op(Op::ILIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match prefix
    let cond = Condition::Op(Op::ILIKE, Field::String("SOURCE%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match suffix
    let cond = Condition::Op(Op::ILIKE, Field::String("%WEB3".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match contains
    let cond = Condition::Op(Op::ILIKE, Field::String("%GLUE%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match start and end with
    let cond = Condition::Op(Op::ILIKE, Field::String("source%WEB3".to_string()));
    expect(match_conditions(&cond, &field), true, false);
}

#[test]
fn test_ilike_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::ILIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), false, false);

    let cond = Condition::Op(Op::ILIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), false, false);
}

#[test]
fn test_nilike() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::NILIKE, field.clone());
    expect(match_conditions(&cond, &field), false, false);

    // not exact match
    let cond = Condition::Op(Op::NILIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match prefix
    let cond = Condition::Op(Op::NILIKE, Field::String("Source%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match prefix case-insensitive
    let cond = Condition::Op(Op::NILIKE, Field::String("SOURCE%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match suffix
    let cond = Condition::Op(Op::NILIKE, Field::String("%WEB3".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match contains
    let cond = Condition::Op(Op::NILIKE, Field::String("%GLUE%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match start and end with
    let cond = Condition::Op(Op::NILIKE, Field::String("source%WEB3".to_string()));
    expect(match_conditions(&cond, &field), false, false);
}

#[test]
fn test_nilike_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::NILIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), true, false);

    let cond = Condition::Op(Op::NILIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), true, false);
}

#[test]
fn test_prop_int() {
    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0))));
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4))));
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );
}

#[test]
fn test_prop_float() {
    let cond = Condition::Prop(2, Box::new(Condition::Op(Op::EQ, get_field(2))));
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::Prop(2, Box::new(Condition::Op(Op::EQ, Field::Float(9.0))));
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );
}

#[test]
fn test_prop_str() {
    let cond = Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1))));
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let str_field = Field::String("wrong".to_string());
    let cond = Condition::Prop(1, Box::new(Condition::Op(Op::EQ, str_field)));
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );
}

#[test]
fn test_prop_bool() {
    let cond = Condition::Prop(3, Box::new(Condition::Op(Op::EQ, get_field(3))));
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::Prop(3, Box::new(Condition::Op(Op::EQ, Field::Bool(false))));
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );
}

#[test]
fn test_prop_time() {
    let cond = Condition::Prop(4, Box::new(Condition::Op(Op::EQ, get_field(4))));
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::Prop(4, Box::new(Condition::Op(Op::EQ, yesterday())));
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );
}

#[test]
fn test_and() {
    let cond = Condition::CompoundOp(
        CompoundOp::AND,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );

    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::AND,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::AND,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(
                1,
                Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
            ),
        ],
    );
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::AND,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), false, true);
}

#[test]
fn test_or() {
    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(
                1,
                Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
            ),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4)))),
            Condition::Prop(
                1,
                Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
            ),
        ],
    );
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::String("4".to_string()))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), false, true);
}

#[test]
fn test_not() {
    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, get_field(0))),
        )],
    );

    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::Int(4))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::String("4".to_string()))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, true);
}

#[test]
fn with_doc_arr_at_least_one_should_match() {
    let mut new_doc = get_doc();
    new_doc.fields[0] = Field::Int(8);

    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0))));
    expect(
        match_conditions(&cond, &Field::DocArray(vec![new_doc.clone(), get_doc()])),
        true,
        false,
    );

    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4))));
    expect(
        match_conditions(&cond, &Field::DocArray(vec![new_doc, get_doc()])),
        false,
        false,
    );
}
