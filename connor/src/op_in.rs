pub fn handle(condition: &core::DocField, doc_field: &core::DocField) -> bool {
    match condition {
        core::DocField::IntArray(int_arr_cond) => {
            if let core::DocField::Int(int_val) = doc_field {
                return int_arr_cond.contains(&int_val);
            }
            false
        }
        core::DocField::FloatArray(float_arr_cond) => {
            if let core::DocField::Float(float_val) = doc_field {
                return float_arr_cond.contains(&float_val);
            }
            false
        }
        core::DocField::StringArray(str_arr_cond) => {
            if let core::DocField::String(str_val) = doc_field {
                return str_arr_cond.contains(&str_val);
            }
            false
        }
        core::DocField::DateTimeArray(date_time_arr_cond) => {
            if let core::DocField::DateTime(date_time_val) = doc_field {
                return date_time_arr_cond.contains(&date_time_val);
            }
            false
        }
        _ => return false,
    }
}
