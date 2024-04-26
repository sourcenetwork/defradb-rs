mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

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
