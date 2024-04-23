use core::doc::Field;

mod err;
mod op_eq;
mod op_ge;
mod op_gt;
mod op_ilike;
mod op_in;
mod op_like;
#[cfg(test)]
mod tests;

#[derive(Clone, Debug)]
pub enum Condition {
    Prop(usize, Box<Condition>),
    Op(Op, Field),
    CompoundOp(CompoundOp, Vec<Condition>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Op {
    EQ,
    NE,
    GE,
    GT,
    LE,
    LT,
    IN,
    NIN,
    LIKE,
    NLIKE,
    ILIKE,
    NILIKE,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CompoundOp {
    NOT,
    AND,
    OR,
}

pub fn match_conditions(condition: &Condition, doc_field: &Field) -> Result<bool, err::Error> {
    match doc_field {
        Field::DocArray(arr) => {
            for doc in arr {
                match match_conditions(condition, &Field::Doc(doc.clone())) {
                    Ok(true) => return Result::Ok(true),
                    Ok(false) => continue,
                    Err(e) => return Result::Err(e),
                }
            }
            return Result::Ok(false);
        }
        Field::OptionalInt(opt_int) => {
            if let Some(int_val) = opt_int {
                return match_conditions(condition, &Field::Int(*int_val));
            }
            return Result::Ok(false);
        }
        Field::OptionalFloat(opt_float) => {
            if let Some(float_val) = opt_float {
                return match_conditions(condition, &Field::Float(*float_val));
            }
            return Result::Ok(false);
        }
        Field::OptionalString(opt_str) => {
            if let Some(str_val) = opt_str {
                return match_conditions(condition, &Field::String(str_val.clone()));
            }
            return Result::Ok(false);
        }
        Field::OptionalBool(opt_bool) => {
            if let Some(bool_val) = opt_bool {
                return match_conditions(condition, &Field::Bool(*bool_val));
            }
            return Result::Ok(false);
        }
        Field::OptionalDateTime(opt_date_time) => {
            if let Some(date_time_val) = opt_date_time {
                return match_conditions(condition, &Field::DateTime(date_time_val.clone()));
            }
            return Result::Ok(false);
        }
        _ => {}
    }

    match &condition {
        &Condition::Op(ref op, ref target_doc_field) => {
            match op {
                Op::GT | Op::GE | Op::LT | Op::LE => {
                    match doc_field {
                        core::doc::Field::Null => return Result::Ok(false),
                        _ => {}
                    }
                    match target_doc_field {
                        core::doc::Field::Null => return Result::Ok(false),
                        _ => {}
                    }
                }
                _ => {}
            }
            match op {
                Op::EQ => return Result::Ok(op_eq::handle(target_doc_field, doc_field)),
                Op::NE => return Result::Ok(!op_eq::handle(target_doc_field, doc_field)),
                Op::GT => return Result::Ok(op_gt::handle(target_doc_field, doc_field)),
                Op::LT => return Result::Ok(!op_ge::handle(target_doc_field, doc_field)),
                Op::GE => return Result::Ok(op_ge::handle(target_doc_field, doc_field)),
                Op::LE => return Result::Ok(!op_gt::handle(target_doc_field, doc_field)),
                Op::IN => return Result::Ok(op_in::handle(target_doc_field, doc_field)),
                Op::NIN => return Result::Ok(!op_in::handle(target_doc_field, doc_field)),
                Op::LIKE => return Result::Ok(op_like::handle(target_doc_field, doc_field)),
                Op::NLIKE => return Result::Ok(!op_like::handle(target_doc_field, doc_field)),
                Op::ILIKE => return Result::Ok(op_ilike::handle(target_doc_field, doc_field)),
                Op::NILIKE => return Result::Ok(!op_ilike::handle(target_doc_field, doc_field)),
            };
        }
        &Condition::Prop(index, op) => {
            if let Field::Doc(doc) = doc_field {
                match doc.fields.get(*index) {
                    Some(prop_data) => return match_conditions(op.as_ref(), prop_data),
                    None => return Result::Err(err::Error::new("Index out of bounds".to_string())),
                }
            }
            return Result::Ok(false);
        }
        &Condition::CompoundOp(op, conditions) => match op {
            CompoundOp::AND => {
                for cond in conditions {
                    match match_conditions(&cond, &doc_field) {
                        Ok(false) => return Result::Ok(false),
                        Ok(true) => continue,
                        Err(e) => return Result::Err(e),
                    }
                }
                return Result::Ok(true);
            }
            CompoundOp::OR => {
                for cond in conditions {
                    match match_conditions(&cond, &doc_field) {
                        Ok(true) => return Result::Ok(true),
                        Ok(false) => continue,
                        Err(e) => return Result::Err(e),
                    }
                }
            }
            CompoundOp::NOT => match match_conditions(&conditions[0], &doc_field) {
                Ok(res) => return Result::Ok(!res),
                Err(e) => return Result::Err(e),
            },
        },
    }
    return Result::Ok(false);
}
