pub fn handle(condition: &core::doc::Field, doc_field: &core::doc::Field) -> bool {
    match condition {
        core::doc::Field::IntArray(int_arr_cond) => {
            if let core::doc::Field::Int(int_val) = doc_field {
                return int_arr_cond.contains(&int_val);
            }
            false
        }
        core::doc::Field::FloatArray(float_arr_cond) => {
            if let core::doc::Field::Float(float_val) = doc_field {
                return float_arr_cond.contains(&float_val);
            }
            false
        }
        core::doc::Field::StringArray(str_arr_cond) => {
            if let core::doc::Field::String(str_val) = doc_field {
                return str_arr_cond.contains(&str_val);
            }
            false
        }
        core::doc::Field::DateTimeArray(date_time_arr_cond) => {
            if let core::doc::Field::DateTime(date_time_val) = doc_field {
                return date_time_arr_cond.contains(&date_time_val);
            }
            false
        }
        _ => return false,
    }
}
