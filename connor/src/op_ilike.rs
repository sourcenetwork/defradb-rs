// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

fn ilike(condition: &str, data: &str) -> bool {
    let mut has_prefix = false;
    let mut has_suffix = false;
    let mut cn = condition.to_lowercase();

    if cn.starts_with('%') {
        has_prefix = true;
        cn.remove(0);
    }
    if cn.ends_with('%') {
        has_suffix = true;
        cn.pop();
    }

    let data = data.to_lowercase();

    match (has_prefix, has_suffix) {
        (true, true) => data.contains(&cn),
        (true, false) => data.ends_with(&cn),
        (false, true) => data.starts_with(&cn),
        (false, false) => {
            let start_and_end: Vec<&str> = cn.split('%').collect();
            if start_and_end.len() == 2 {
                data.starts_with(start_and_end[0]) && data.ends_with(start_and_end[1])
            } else {
                data == cn
            }
        }
    }
}

pub fn handle(condition: &core::doc::Field, doc_field: &core::doc::Field) -> bool {
    match (condition, doc_field) {
        (core::doc::Field::Null, core::doc::Field::Null) => true,
        (core::doc::Field::Null, _) | (_, core::doc::Field::Null) => false,
        (core::doc::Field::String(str_cond), core::doc::Field::String(str_val)) => {
            ilike(str_cond, str_val)
        }
        _ => false,
    }
}
