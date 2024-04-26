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

use common::*;
use connor::cond::*;
use core::doc::Field;

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
