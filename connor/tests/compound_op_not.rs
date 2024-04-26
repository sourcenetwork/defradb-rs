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
fn test_not() {
    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, get_field(0))),
        )],
    );

    expect(
        match_conditions(&cond, &Field::Doc(get_doc())),
        false,
        false,
    );

    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::Int(4))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, false);

    let cond = Condition::CompoundOp(
        CompoundOp::NOT,
        vec![Condition::Prop(
            0,
            Box::new(Condition::Op(Op::EQ, Field::String("4".to_string()))),
        )],
    );
    expect(match_conditions(&cond, &Field::Doc(get_doc())), true, true);
}
