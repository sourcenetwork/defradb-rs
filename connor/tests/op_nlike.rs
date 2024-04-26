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
fn test_nlike() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::NLIKE, field.clone());
    expect(match_conditions(&cond, &field), false, false);

    // not exact match
    let cond = Condition::Op(Op::NLIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match prefix
    let cond = Condition::Op(Op::NLIKE, Field::String("Source%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match suffix
    let cond = Condition::Op(Op::NLIKE, Field::String("%web3".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match contains
    let cond = Condition::Op(Op::NLIKE, Field::String("%glue%".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match start and end with
    let cond = Condition::Op(Op::NLIKE, Field::String("Source%web3".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // case sensitive no match
    let field = Field::String("SOURCE is the glue of web3".to_string());
    expect(match_conditions(&cond, &field), true, false);

    // case sensitive no prefix match
    let field = Field::String("SOURCE%".to_string());
    expect(match_conditions(&cond, &field), true, false);
}

#[test]
fn test_nlike_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::NLIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), true, false);

    let cond = Condition::Op(Op::NLIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), true, false);
}
