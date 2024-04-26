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
