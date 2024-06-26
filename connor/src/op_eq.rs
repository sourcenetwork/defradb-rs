// Copyright 2024 Democratized Data Foundation
//
// Use of this software is governed by the Business Source License
// included in the file licenses/BSL.txt.
//
// As of the Change Date specified in that file, in accordance with
// the Business Source License, use of this software will be governed
// by the Apache License, Version 2.0, included in the file
// licenses/APL.txt.

pub fn handle(target_doc_field: &core::doc::Field, doc_field: &core::doc::Field) -> bool {
    match target_doc_field {
        core::doc::Field::Null => matches!(doc_field, core::doc::Field::Null),
        core::doc::Field::String(str_cond) => {
            if let core::doc::Field::String(str_val) = doc_field {
                return str_val == str_cond;
            }
            false
        }
        core::doc::Field::Int(int_cond) => {
            if let core::doc::Field::Int(int_val) = doc_field {
                return int_val == int_cond;
            }
            if let core::doc::Field::Float(float_val) = doc_field {
                return *float_val == *int_cond as f64;
            }
            false
        }
        core::doc::Field::Float(float_cond) => {
            if let core::doc::Field::Float(float_val) = doc_field {
                return float_val == float_cond;
            }
            if let core::doc::Field::Int(int_val) = doc_field {
                return *int_val as f64 == *float_cond;
            }
            false
        }
        core::doc::Field::Bool(bool_cond) => {
            if let core::doc::Field::Bool(bool_val) = doc_field {
                return bool_val == bool_cond;
            }
            false
        }
        core::doc::Field::DateTime(date_time_cond) => {
            if let core::doc::Field::DateTime(date_time_val) = doc_field {
                return date_time_val == date_time_cond;
            }
            false
        }
        core::doc::Field::BoolArray(arr_cond) => {
            if let core::doc::Field::BoolArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::IntArray(arr_cond) => {
            if let core::doc::Field::IntArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::FloatArray(arr_cond) => {
            if let core::doc::Field::FloatArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::StringArray(arr_cond) => {
            if let core::doc::Field::StringArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::DateTimeArray(arr_cond) => {
            if let core::doc::Field::DateTimeArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::OptionalBoolArray(arr_cond) => {
            if let core::doc::Field::OptionalBoolArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::OptionalIntArray(arr_cond) => {
            if let core::doc::Field::OptionalIntArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::OptionalFloatArray(arr_cond) => {
            if let core::doc::Field::OptionalFloatArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::OptionalStringArray(arr_cond) => {
            if let core::doc::Field::OptionalStringArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        core::doc::Field::OptionalDateTimeArray(arr_cond) => {
            if let core::doc::Field::OptionalDateTimeArray(arr_val) = doc_field {
                return arr_val == arr_cond;
            }
            false
        }
        _ => return false,
    }
}
