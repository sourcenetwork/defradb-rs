// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

extern crate libc;

use crate::core as core_abi;
use crate::util;
use connor::cond;
use libc::c_char;
use serde_json;
use serde_json::Value;
use std::convert::TryFrom;

#[no_mangle]
pub extern "C" fn match_conditions(
    cond_json: *const c_char,
    doc_json: *const c_char,
    error: *mut util::StringBuffer,
) -> bool {
    let cond_str = unsafe { util::const_char_to_str(cond_json) };
    let doc_str = unsafe { util::const_char_to_str(doc_json) };
    match match_conditions_with(cond_str, doc_str) {
        Ok(result) => result,
        Err(e) => {
            unsafe {
                (*error).fill(&e);
            }
            false
        }
    }
}

pub fn match_conditions_with(cond_str: &str, doc_str: &str) -> Result<bool, String> {
    let cond = deserialize_condition(cond_str).map_err(|e| e.to_string())?;
    match core_abi::deserialize_doc(doc_str).map_err(|e| e.to_string()) {
        Ok(doc) => {
            cond::match_conditions(&cond, &core::doc::Field::Doc(doc)).map_err(|e| e.to_string())
        }
        Err(_) => match core_abi::deserialize_field(doc_str).map_err(|e| e.to_string()) {
            Ok(field) => cond::match_conditions(&cond, &field).map_err(|e| e.to_string()),
            Err(e) => Err(e),
        },
    }
}

pub fn deserialize_condition(json_str: &str) -> Result<cond::Condition, String> {
    let v: Value = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    let cond = ConditionWrapper::try_from(v).map_err(|e| e.to_string())?;
    Ok(cond.0)
}

#[derive(Debug)]
struct ConditionWrapper(cond::Condition);

impl TryFrom<Value> for ConditionWrapper {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                for (key, value) in map.iter() {
                    match key.as_str() {
                        "EQ" | "NE" | "GT" | "GE" | "LT" | "LE" | "IN" | "NIN" | "LIKE"
                        | "NLIKE" | "ILIKE" | "NILIKE" => {
                            let field = core_abi::FieldWrapper::try_from(value.clone())?;
                            return Ok(ConditionWrapper(cond::Condition::Op(
                                match key.as_str() {
                                    "EQ" => cond::Op::EQ,
                                    "NE" => cond::Op::NE,
                                    "GT" => cond::Op::GT,
                                    "GE" => cond::Op::GE,
                                    "LT" => cond::Op::LT,
                                    "LE" => cond::Op::LE,
                                    "IN" => cond::Op::IN,
                                    "NIN" => cond::Op::NIN,
                                    "LIKE" => cond::Op::LIKE,
                                    "NLIKE" => cond::Op::NLIKE,
                                    "ILIKE" => cond::Op::ILIKE,
                                    "NILIKE" => cond::Op::NILIKE,
                                    _ => unreachable!(),
                                },
                                field.0,
                            )));
                        }
                        "PROP" => {
                            if let Value::Object(prop_map) = value {
                                let index = prop_map
                                    .get("index")
                                    .and_then(Value::as_u64)
                                    .ok_or("Index missing or not a number in Prop")?
                                    as usize;
                                let condition_val = prop_map
                                    .get("condition")
                                    .ok_or("Condition missing in Prop")?;
                                let condition = ConditionWrapper::try_from(condition_val.clone())?;
                                return Ok(ConditionWrapper(cond::Condition::Prop(
                                    index,
                                    Box::new(condition.0),
                                )));
                            }
                        }
                        "AND" | "OR" => {
                            let conditions = value
                                .as_array()
                                .ok_or("Expected array for CompoundOp")?
                                .iter()
                                .map(|item| ConditionWrapper::try_from(item.clone()))
                                .collect::<Result<Vec<_>, String>>()?;
                            return Ok(ConditionWrapper(cond::Condition::CompoundOp(
                                match key.as_str() {
                                    "AND" => cond::CompoundOp::AND,
                                    "OR" => cond::CompoundOp::OR,
                                    _ => unreachable!(),
                                },
                                conditions.into_iter().map(|wrapper| wrapper.0).collect(),
                            )));
                        }
                        "NOT" => {
                            let condition = ConditionWrapper::try_from(value.clone())?;
                            return Ok(ConditionWrapper(cond::Condition::CompoundOp(
                                cond::CompoundOp::NOT,
                                vec![condition.0],
                            )));
                        }
                        _ => {
                            return Err(format!("Invalid key in Condition: {}", key));
                        }
                    }
                }
                Err("Invalid JSON object for Condition".to_string())
            }
            _ => Err("Expected a JSON object for Condition".to_string()),
        }
    }
}
