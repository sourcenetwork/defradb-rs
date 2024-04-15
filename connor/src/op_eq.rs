pub fn handle(target_doc_field: &core::DocField, doc_field: &core::DocField) -> bool {
    match target_doc_field {
        core::DocField::String(str_cond) => {
            if let core::DocField::String(str_val) = doc_field {
                return str_val == str_cond;
            }
            false
        }
        core::DocField::Int(int_cond) => {
            if let core::DocField::Int(int_val) = doc_field {
                return int_val == int_cond;
            }
            if let core::DocField::Float(float_val) = doc_field {
                return *float_val == *int_cond as f64;
            }
            false
        }
        core::DocField::Float(float_cond) => {
            if let core::DocField::Float(float_val) = doc_field {
                return float_val == float_cond;
            }
            if let core::DocField::Int(int_val) = doc_field {
                return *int_val as f64 == *float_cond;
            }
            false
        }
        core::DocField::Bool(bool_cond) => {
            if let core::DocField::Bool(bool_val) = doc_field {
                return bool_val == bool_cond;
            }
            false
        }
        core::DocField::DateTime(date_time_cond) => {
            if let core::DocField::DateTime(date_time_val) = doc_field {
                return date_time_val == date_time_cond;
            }
            false
        }
        _ => return false,
    }
}
