use core::DocField;
//use core::{Doc, DocField};

#[derive(Clone)]
pub enum FilterKey {
    Prop(core::Doc, usize),
    Op(Operator, DocField),
}

#[derive(Clone)]
pub enum Operator {
    AND,
    EQ,
    GE,
    GT,
    IN,
    LE,
    LT,
    NE,
    NIN,
    OR,
    LIKE,
    NLIKE,
    ILIKE,
    NILIKE,
    NOT,
}

pub fn match_conditions(condition: FilterKey, data: DocField) -> bool {
    //return eq(condition, data);
    let cond_copy = condition.clone();
    match condition {
        FilterKey::Prop(ref doc, index) => {
            //let propData = propIndex.0.fields[propIndex.1];
            let data = doc.fields[index].clone();
            true
            //return eq(condition, data);
        }
        FilterKey::Op(op, doc_field) => {
            match op {
                Operator::EQ => return eq(doc_field.clone(), data.clone()),
                Operator::GT => return gt(doc_field.clone(), data.clone()),
                _ => false,
            };
            match doc_field {
                DocField::String(str_cond) => {
                    if let DocField::String(str_val) = data {
                        return str_val == str_cond;
                    }
                }
                DocField::Int(int_cond) => {
                    if let DocField::Int(int_val) = data {
                        return int_val == int_cond;
                    }
                }
                DocField::Float(float_cond) => {
                    if let DocField::Float(float_val) = data {
                        return float_val == float_cond;
                    }
                }
                _ => return false,
            }
            return process_operator(op, cond_copy, data);
        }
    }
}

fn process_operator(op: Operator, condition: FilterKey, data: core::DocField) -> bool {
    match op {
        Operator::EQ => {
            //eq(condition, data)
            true
        }
        _ => false,
    }
}

fn eq(condition: core::DocField, data: core::DocField) -> bool {
    match condition {
        core::DocField::String(str_cond) => {
            if let core::DocField::String(str_val) = data {
                return str_val == str_cond;
            }
            false
        }
        core::DocField::Int(int_cond) => {
            if let core::DocField::Int(int_val) = data {
                return int_val == int_cond;
            }
            false
        }
        core::DocField::Float(float_cond) => {
            if let core::DocField::Float(float_val) = data {
                return float_val == float_cond;
            }
            false
        }
        _ => return false,
    }
}

fn gt(condition: core::DocField, data: core::DocField) -> bool {
    match condition {
        core::DocField::Int(int_cond) => {
            if let core::DocField::Int(int_val) = data {
                return int_val > int_cond;
            }
            false
        }
        _ => return false,
    }
}

fn eq2(condition: FilterKey, data: core::DocField) -> bool {
    //fn eq(condition: FilterKey, data: core::DocField) -> bool {
    /*match data {
        core::DocField::DocArray(docArray) => {
            for doc in docArray {
                if eq(condition, doc) {
                    true
                }
            }
            false
        }
        core::DocField::MaybeString(strVal) => {
            if let Data::Value(core::DocField::String(strCond)) = condition {
                return strVal == strCond;
            }
        }
        core::DocField::MaybeInt(intVal) => {
            if let Data::Value(core::DocField::Int(intCond)) = condition {
                return intVal == intCond;
            }
        }
        core::DocField::MaybeFloat(floatVal) => {
            if let Data::Value(core::DocField::Float(floatCond)) = condition {
                return floatVal == floatCond;
            }
        }
        core::DocField::MaybeBool(boolVal) => {
            if let Data::Value(core::DocField::Bool(boolCond)) = condition {
                return boolVal == boolCond;
            }
        }
        core::DocField::MaybeTime(timeVal) => {
            if let Data::Value(core::DocField::Time(timeCond)) = condition {
                return timeVal == timeCond;
            }
        }
    }*/

    let cond_copy = condition.clone();
    match condition {
        FilterKey::Prop(ref doc, index) => {
            //let propData = propIndex.0.fields[propIndex.1];
            let data = doc.fields[index].clone();
            false
            //return eq(condition, data);
        }
        FilterKey::Op(op, doc_field) => {
            match doc_field {
                DocField::String(str_cond) => {
                    if let DocField::String(str_val) = data {
                        return str_val == str_cond;
                    }
                }
                DocField::Int(int_cond) => {
                    if let DocField::Int(int_val) = data {
                        return int_val == int_cond;
                    }
                }
                DocField::Float(float_cond) => {
                    if let DocField::Float(float_val) = data {
                        return float_val == float_cond;
                    }
                }
                _ => return false,
            }
            return process_operator(op, cond_copy, data);
        }
    }
    /*match condition {
        core::DocField::String(strCond)) => {
            if let Data::Value(core::DocField::String(strVal)) = data {
                return strVal == strCond;
            }
        }
        core::DocField::Int(intCond)) => {
            if let Data::Value(core::DocField::Int(intVal)) = data {
                return intVal == intCond;
            }
        }
        core::DocField::Float(floatCond)) => {
            if let Data::Value(core::DocField::Float(floatVal)) = data {
                return floatVal == floatCond;
            }
        }
        Data::Map(mapCond) => {
            for (prop, cond) in mapCond {
                match prop {
                    FilterKey::PropertyIndex(propIndex) => {
                        let propData = propIndex.0.fields[propIndex.1];
                        if !eq(cond, propData) {
                            return false;
                        }
                    }
                    FilterKey::Operator(op) => {
                        return match_conditions(op, data);
                    }
                }
                /*if let Data::Value(condData) = cond {

                }
                if let Some(dataValue) = data.get(key) {
                    if !eq(value, dataValue) {
                        return false;
                    }
                } else {
                    return false;
                }*/
            }
            return true;
        }
    }*/
    /*
    switch cn := condition.(type) {
    case string:
        if d, ok := data.(string); ok {
            return d == cn, nil
        }
        return false, nil
    case int64:
        return numbers.Equal(cn, data), nil
    case int32:
        return numbers.Equal(cn, data), nil
    case float64:
        return numbers.Equal(cn, data), nil
    case map[FilterKey]any:
        m := true
        for prop, cond := range cn {
            var err error
            m, err = matchWith(prop.GetOperatorOrDefault("_eq"), cond, prop.GetProp(data))
            if err != nil {
                return false, err
            }

            if !m {
                // No need to evaluate after we fail
                break
            }
        }

        return m, nil
    case time.Time:
        return ctime.Equal(cn, data), nil
    default:
        return reflect.DeepEqual(condition, data), nil
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq() {
        let cond = FilterKey::Op(Operator::EQ, core::DocField::String("hello".to_string()));
        let data = "hello".to_string();
        let match_result = match_conditions(cond.clone(), core::DocField::String(data));
        assert!(match_result, "Expected match");

        let data = "other".to_string();
        let match_result = match_conditions(cond, core::DocField::String(data));
        assert!(!match_result, "Expected no match");
    }

    #[test]
    fn test_gt() {
        let cond = FilterKey::Op(Operator::GT, core::DocField::Int(5));
        let match_result = match_conditions(cond.clone(), core::DocField::Int(6));
        assert!(match_result, "Expected match");

        let match_result = match_conditions(cond, core::DocField::Int(5));
        assert!(!match_result, "Expected no match");
    }
}
