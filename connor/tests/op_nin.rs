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
fn test_nin() {
    let cond = Condition::Op(Op::NIN, Field::IntArray(vec![5, 6, 7]));
    expect(match_conditions(&cond, &Field::Int(4)), true, false);
    expect(match_conditions(&cond, &Field::Int(6)), false, false);
}
