mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

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
