pub fn handle(condition: &core::doc::Field, doc_field: &core::doc::Field) -> bool {
    match condition {
        core::doc::Field::Int(int_cond) => {
            if let core::doc::Field::Int(int_val) = doc_field {
                return int_val > int_cond;
            }
            if let core::doc::Field::Float(float_val) = doc_field {
                return *float_val > *int_cond as f64;
            }
            false
        }
        core::doc::Field::Float(float_cond) => {
            if let core::doc::Field::Float(float_val) = doc_field {
                return float_val > float_cond;
            }
            if let core::doc::Field::Int(int_val) = doc_field {
                return *int_val as f64 > *float_cond;
            }
            false
        }
        core::doc::Field::DateTime(date_time_cond) => {
            if let core::doc::Field::DateTime(date_time_val) = doc_field {
                return date_time_val > date_time_cond;
            }
            false
        }
        _ => return false,
    }
}
