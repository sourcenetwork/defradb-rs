mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

#[test]
fn test_nin() {
    let cond = Condition::Op(Op::NIN, Field::IntArray(vec![5, 6, 7]));
    expect(match_conditions(&cond, &Field::Int(4)), true, false);
    expect(match_conditions(&cond, &Field::Int(6)), false, false);
}
