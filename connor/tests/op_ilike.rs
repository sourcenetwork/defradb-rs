mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

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
