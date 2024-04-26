mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

#[test]
fn with_doc_arr_at_least_one_should_match() {
    let mut new_doc = get_doc();
    new_doc.fields[0] = Field::Int(8);

    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0))));
    expect(
        match_conditions(&cond, &Field::DocArray(vec![new_doc.clone(), get_doc()])),
        true,
        false,
    );

    let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4))));
    expect(
        match_conditions(&cond, &Field::DocArray(vec![new_doc, get_doc()])),
        false,
        false,
    );
}
