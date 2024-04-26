mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

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
