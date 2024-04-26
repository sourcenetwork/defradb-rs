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
fn test_like() {
    let field = Field::String("Source is the glue of web3".to_string());

    // exact match
    let cond = Condition::Op(Op::LIKE, field.clone());
    expect(match_conditions(&cond, &field), true, false);

    // not exact match
    let cond = Condition::Op(Op::LIKE, Field::String("Source is the glue".to_string()));
    expect(match_conditions(&cond, &field), false, false);

    // match prefix
    let cond = Condition::Op(Op::LIKE, Field::String("Source%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match suffix
    let cond = Condition::Op(Op::LIKE, Field::String("%web3".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match contains
    let cond = Condition::Op(Op::LIKE, Field::String("%glue%".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // match start and end with
    let cond = Condition::Op(Op::LIKE, Field::String("Source%web3".to_string()));
    expect(match_conditions(&cond, &field), true, false);

    // case sensitive no match
    let field = Field::String("SOURCE is the glue of web3".to_string());
    expect(match_conditions(&cond, &field), false, false);

    // case sensitive no prefix match
    let field = Field::String("SOURCE%".to_string());
    expect(match_conditions(&cond, &field), false, false);
}

#[test]
fn test_like_null() {
    let str_field = Field::String("Source is the glue of web3".to_string());

    let cond = Condition::Op(Op::LIKE, Field::Null);
    expect(match_conditions(&cond, &str_field), false, false);

    let cond = Condition::Op(Op::LIKE, str_field);
    expect(match_conditions(&cond, &Field::Null), false, false);
}
