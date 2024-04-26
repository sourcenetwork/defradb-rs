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
