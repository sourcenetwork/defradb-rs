pub fn handle(condition: &core::doc::Field, doc_field: &core::doc::Field) -> bool {
    match condition {
        core::doc::Field::BoolArray(arr_cond) => {
            if let core::doc::Field::Bool(val) = doc_field {
                return arr_cond.contains(&val);
            }
            false
        }
        core::doc::Field::IntArray(arr_cond) => {
            if let core::doc::Field::Int(val) = doc_field {
                return arr_cond.contains(&val);
            }
            false
        }
        core::doc::Field::FloatArray(arr_cond) => {
            if let core::doc::Field::Float(val) = doc_field {
                return arr_cond.contains(&val);
            }
            false
        }
        core::doc::Field::StringArray(arr_cond) => {
            if let core::doc::Field::String(val) = doc_field {
                return arr_cond.contains(&val);
            }
            false
        }
        core::doc::Field::DateTimeArray(arr_cond) => {
            if let core::doc::Field::DateTime(val) = doc_field {
                return arr_cond.contains(&val);
            }
            false
        }
        core::doc::Field::OptionalBoolArray(arr_cond) => match doc_field {
            core::doc::Field::OptionalBool(val) => arr_cond.contains(&val),
            core::doc::Field::Bool(val) => arr_cond.contains(&Some(*val)),
            core::doc::Field::Null => arr_cond.contains(&None),
            _ => false,
        },
        core::doc::Field::OptionalIntArray(arr_cond) => match doc_field {
            core::doc::Field::OptionalInt(val) => arr_cond.contains(&val),
            core::doc::Field::Int(val) => arr_cond.contains(&Some(*val)),
            core::doc::Field::Null => arr_cond.contains(&None),
            _ => false,
        },
        core::doc::Field::OptionalFloatArray(arr_cond) => match doc_field {
            core::doc::Field::OptionalFloat(val) => arr_cond.contains(&val),
            core::doc::Field::Float(val) => arr_cond.contains(&Some(*val)),
            core::doc::Field::Null => arr_cond.contains(&None),
            _ => false,
        },
        core::doc::Field::OptionalStringArray(arr_cond) => match doc_field {
            core::doc::Field::OptionalString(val) => arr_cond.contains(&val),
            core::doc::Field::String(val) => arr_cond.contains(&Some(val.clone())),
            core::doc::Field::Null => arr_cond.contains(&None),
            _ => false,
        },
        core::doc::Field::OptionalDateTimeArray(arr_cond) => match doc_field {
            core::doc::Field::OptionalDateTime(val) => arr_cond.contains(&val),
            core::doc::Field::DateTime(val) => arr_cond.contains(&Some(val.clone())),
            core::doc::Field::Null => arr_cond.contains(&None),
            _ => false,
        },
        _ => return false,
    }
}
