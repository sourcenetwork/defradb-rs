mod common;

use common::*;
use connor::cond::*;
use core::doc::Field;

#[test]
fn test_or() {
    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4)))),
            Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
            Condition::Prop(
                1,
                Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
            ),
        ],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![
            Condition::Prop(0, Box::new(Condition::Op(Op::EQ, Field::Int(4)))),
            Condition::Prop(
                1,
                Box::new(Condition::Op(Op::EQ, Field::String("wrong".to_string()))),
            ),
        ],
    );
    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::OR,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::String("4".to_string()))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), false, true);
}
