// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

mod common;

use connor::cond::*;

use chrono::*;
use common::*;
use core::doc::Field;

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
