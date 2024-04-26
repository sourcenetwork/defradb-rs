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
