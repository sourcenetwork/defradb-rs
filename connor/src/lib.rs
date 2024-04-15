use core::DocField;
use std::error;
use std::fmt;

mod op_eq;
mod op_ge;
mod op_gt;
mod op_ilike;
mod op_in;
mod op_like;

#[derive(Clone, Debug)]
pub enum Condition {
    Prop(usize, Box<Condition>),
    Op(Op, DocField),
    CompOp(CompOp, Vec<Condition>),
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub enum CompOp {
    NOT,
    AND,
    OR,
}

#[derive(Clone, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Self { message: message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl error::Error for Error {}

pub fn match_conditions(condition: &Condition, doc_field: &DocField) -> Result<bool, Error> {
    match doc_field {
        DocField::DocArray(arr) => {
            for doc in arr {
                match match_conditions(condition, &DocField::Doc(doc.clone())) {
                    Ok(true) => return Result::Ok(true),
                    Ok(false) => continue,
                    Err(e) => return Result::Err(e),
                }
            }
            return Result::Ok(false);
        }
        DocField::MaybeInt(opt_int) => {
            if let Some(int_val) = opt_int {
                return match_conditions(condition, &DocField::Int(*int_val));
            }
            return Result::Ok(false);
        }
        DocField::MaybeFloat(opt_float) => {
            if let Some(float_val) = opt_float {
                return match_conditions(condition, &DocField::Float(*float_val));
            }
            return Result::Ok(false);
        }
        DocField::MaybeString(opt_str) => {
            if let Some(str_val) = opt_str {
                return match_conditions(condition, &DocField::String(str_val.clone()));
            }
            return Result::Ok(false);
        }
        DocField::MaybeBool(opt_bool) => {
            if let Some(bool_val) = opt_bool {
                return match_conditions(condition, &DocField::Bool(*bool_val));
            }
            return Result::Ok(false);
        }
        DocField::MaybeDateTime(opt_date_time) => {
            if let Some(date_time_val) = opt_date_time {
                return match_conditions(condition, &DocField::DateTime(date_time_val.clone()));
            }
            return Result::Ok(false);
        }
        _ => {}
    }

    match &condition {
        &Condition::Op(ref op, ref target_doc_field) => {
            match op {
                Op::EQ => return Result::Ok(op_eq::handle(target_doc_field, doc_field)),
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
                _ => false,
            };
        }
        &Condition::Prop(index, op) => {
            if let DocField::Doc(doc) = doc_field {
                match doc.fields.get(*index) {
                    Some(prop_data) => return match_conditions(op.as_ref(), prop_data),
                    None => return Result::Err(Error::new("Index out of bounds".to_string())),
                }
            }
            return Result::Ok(false);
        }
        &Condition::CompOp(op, conditions) => match op {
            CompOp::AND => {
                for cond in conditions {
                    match match_conditions(&cond, &doc_field) {
                        Ok(false) => return Result::Ok(false),
                        Ok(true) => continue,
                        Err(e) => return Result::Err(e),
                    }
                }
                return Result::Ok(true);
            }
            CompOp::OR => {
                for cond in conditions {
                    match match_conditions(&cond, &doc_field) {
                        Ok(true) => return Result::Ok(true),
                        Ok(false) => continue,
                        Err(e) => return Result::Err(e),
                    }
                }
            }
            CompOp::NOT => match match_conditions(&conditions[0], &doc_field) {
                Ok(res) => return Result::Ok(!res),
                Err(e) => return Result::Err(e),
            },
        },
    }
    return Result::Ok(false);
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::*;
    use core::DocField;

    fn now_time() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc
            .with_ymd_and_hms(2024, 4, 8, 12, 34, 56)
            .unwrap()
    }

    fn now() -> DocField {
        DocField::DateTime(now_time())
    }

    fn yesterday_time() -> chrono::DateTime<chrono::Utc> {
        chrono::Utc
            .with_ymd_and_hms(2024, 4, 7, 12, 34, 56)
            .unwrap()
    }

    fn yesterday() -> DocField {
        DocField::DateTime(yesterday_time())
    }

    fn get_fields() -> Vec<DocField> {
        vec![
            DocField::Int(5),
            DocField::String("str".to_string()),
            DocField::Float(1.2),
            DocField::Bool(true),
            now(),
        ]
    }

    fn get_field(index: usize) -> DocField {
        get_fields()[index].clone()
    }

    fn get_doc() -> core::Doc {
        core::Doc {
            hidden: false,
            status: core::DocumentStatus::Active,
            fields: get_fields(),
            schema_version_id: "".to_string(),
        }
    }

    fn expect(actual: Result<bool, Error>, expect_match: bool, expect_error: bool) {
        match actual {
            Ok(true) => assert!(expect_match, "Expected match"),
            Ok(false) => assert!(!expect_match, "Expected no match"),
            _ => {
                if !expect_error {
                    panic!("Expected no error")
                }
            }
        }
    }

    fn test_op(doc_field: DocField, op: Op, target_field: DocField, expect_match: bool) {
        let cond = Condition::Op(op, target_field);
        expect(match_conditions(&cond, &doc_field), expect_match, false);
    }

    #[test]
    fn test_eq_int() {
        test_op(DocField::Int(5), Op::EQ, DocField::Int(5), true);
        test_op(DocField::Int(4), Op::EQ, DocField::Int(5), false);
    }

    #[test]
    fn test_eq_opt_int() {
        test_op(DocField::MaybeInt(Some(5)), Op::EQ, DocField::Int(5), true);
        test_op(DocField::MaybeInt(None), Op::EQ, DocField::Int(5), false);
        test_op(DocField::MaybeInt(Some(5)), Op::EQ, DocField::Int(4), false);
    }

    #[test]
    fn test_eq_str() {
        test_op(
            DocField::String("str".to_string()),
            Op::EQ,
            DocField::String("str".to_string()),
            true,
        );
        test_op(
            DocField::String("str".to_string()),
            Op::EQ,
            DocField::String("wrong".to_string()),
            false,
        );
    }

    #[test]
    fn test_eq_opt_str() {
        test_op(
            DocField::MaybeString(Some("str".to_string())),
            Op::EQ,
            DocField::String("str".to_string()),
            true,
        );
        test_op(
            DocField::MaybeString(None),
            Op::EQ,
            DocField::String("str".to_string()),
            false,
        );
        test_op(
            DocField::MaybeString(Some("str".to_string())),
            Op::EQ,
            DocField::String("wrong".to_string()),
            false,
        );
    }

    #[test]
    fn test_eq_float() {
        test_op(DocField::Float(5.0), Op::EQ, DocField::Float(5.0), true);
        test_op(DocField::Float(5.5), Op::EQ, DocField::Float(5.0), false);
    }

    #[test]
    fn test_eq_opt_float() {
        test_op(
            DocField::MaybeFloat(Some(5.0)),
            Op::EQ,
            DocField::Float(5.0),
            true,
        );
        test_op(
            DocField::MaybeFloat(None),
            Op::EQ,
            DocField::Float(5.0),
            false,
        );
        test_op(
            DocField::MaybeFloat(Some(5.0)),
            Op::EQ,
            DocField::Float(4.0),
            false,
        );
    }

    #[test]
    fn test_eq_float_int() {
        test_op(DocField::Float(4.0), Op::EQ, DocField::Int(4), true);
        test_op(DocField::Int(4), Op::EQ, DocField::Float(4.0), true);
    }

    #[test]
    fn test_eq_bool() {
        test_op(DocField::Bool(true), Op::EQ, DocField::Bool(true), true);
        test_op(DocField::Bool(false), Op::EQ, DocField::Bool(true), false);
    }

    #[test]
    fn test_eq_opt_bool() {
        test_op(
            DocField::MaybeBool(Some(true)),
            Op::EQ,
            DocField::Bool(true),
            true,
        );
        test_op(
            DocField::MaybeBool(None),
            Op::EQ,
            DocField::Bool(true),
            false,
        );
        test_op(
            DocField::MaybeBool(Some(true)),
            Op::EQ,
            DocField::Bool(false),
            false,
        );
    }

    #[test]
    fn test_eq_time() {
        test_op(now(), Op::EQ, now(), true);
        test_op(yesterday(), Op::EQ, now(), false);
    }

    #[test]
    fn test_eq_opt_time() {
        test_op(
            DocField::MaybeDateTime(Some(now_time())),
            Op::EQ,
            now(),
            true,
        );
        test_op(DocField::MaybeDateTime(None), Op::EQ, now(), false);
        test_op(
            DocField::MaybeDateTime(Some(now_time())),
            Op::EQ,
            yesterday(),
            false,
        );
    }

    #[test]
    fn test_gt_int() {
        test_op(DocField::Int(5), Op::GT, DocField::Int(4), true);
        test_op(DocField::Int(5), Op::GT, DocField::Int(5), false);
        test_op(DocField::Int(4), Op::GT, DocField::Int(5), false);
    }

    #[test]
    fn test_gt_float() {
        test_op(DocField::Float(5.0), Op::GT, DocField::Float(4.0), true);
        test_op(DocField::Float(5.0), Op::GT, DocField::Float(5.0), false);
        test_op(DocField::Float(4.0), Op::GT, DocField::Float(5.0), false);
    }

    #[test]
    fn test_gt_float_int() {
        test_op(DocField::Float(5.0), Op::GT, DocField::Int(4), true);
        test_op(DocField::Float(5.5), Op::GT, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::GT, DocField::Float(4.0), true);
        test_op(DocField::Int(5), Op::GT, DocField::Float(4.5), true);
    }

    #[test]
    fn test_gt_time() {
        test_op(now(), Op::GT, yesterday(), true);
        test_op(now(), Op::GT, now(), false);
        test_op(yesterday(), Op::GT, now(), false);
    }

    #[test]
    fn test_ge_int() {
        test_op(DocField::Int(6), Op::GE, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::GE, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::GE, DocField::Int(6), false);
    }

    #[test]
    fn test_ge_float() {
        test_op(DocField::Float(6.0), Op::GE, DocField::Float(5.0), true);
        test_op(DocField::Float(5.0), Op::GE, DocField::Float(5.0), true);
        test_op(DocField::Float(5.0), Op::GE, DocField::Float(6.0), false);
    }

    #[test]
    fn test_ge_float_int() {
        test_op(DocField::Float(5.0), Op::GE, DocField::Int(5), true);
        test_op(DocField::Float(5.5), Op::GE, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::GE, DocField::Float(5.0), true);
        test_op(DocField::Int(5), Op::GE, DocField::Float(4.5), true);
    }

    #[test]
    fn test_ge_time() {
        test_op(now(), Op::GE, yesterday(), true);
        test_op(now(), Op::GE, now(), true);
        test_op(yesterday(), Op::GE, now(), false);
    }

    #[test]
    fn test_lt_int() {
        test_op(DocField::Int(4), Op::LT, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::LT, DocField::Int(5), false);
        test_op(DocField::Int(5), Op::LT, DocField::Int(4), false);
    }

    #[test]
    fn test_lt_float() {
        test_op(DocField::Float(4.0), Op::LT, DocField::Float(5.0), true);
        test_op(DocField::Float(5.0), Op::LT, DocField::Float(5.0), false);
        test_op(DocField::Float(5.0), Op::LT, DocField::Float(4.0), false);
    }

    #[test]
    fn test_lt_float_int() {
        test_op(DocField::Float(4.0), Op::LT, DocField::Int(5), true);
        test_op(DocField::Float(4.5), Op::LT, DocField::Int(5), true);
        test_op(DocField::Int(4), Op::LT, DocField::Float(4.5), true);
    }

    #[test]
    fn test_lt_time() {
        test_op(yesterday(), Op::LT, now(), true);
        test_op(now(), Op::LT, now(), false);
        test_op(now(), Op::LT, yesterday(), false);
    }

    #[test]
    fn test_le_int() {
        test_op(DocField::Int(5), Op::LE, DocField::Int(6), true);
        test_op(DocField::Int(5), Op::LE, DocField::Int(5), true);
        test_op(DocField::Int(6), Op::LE, DocField::Int(5), false);
    }

    #[test]
    fn test_le_float() {
        test_op(DocField::Float(5.0), Op::LE, DocField::Float(5.5), true);
        test_op(DocField::Float(5.0), Op::LE, DocField::Float(5.0), true);
        test_op(DocField::Float(5.5), Op::LE, DocField::Float(5.0), false);
    }

    #[test]
    fn test_le_float_int() {
        test_op(DocField::Float(5.0), Op::LE, DocField::Int(5), true);
        test_op(DocField::Float(4.5), Op::LE, DocField::Int(5), true);
        test_op(DocField::Int(5), Op::LE, DocField::Float(5.0), true);
        test_op(DocField::Int(5), Op::LE, DocField::Float(5.5), true);
    }

    #[test]
    fn test_le_time() {
        test_op(yesterday(), Op::LE, now(), true);
        test_op(now(), Op::LE, now(), true);
        test_op(now(), Op::LE, yesterday(), false);
    }

    #[test]
    fn test_in_int() {
        let cond = Condition::Op(Op::IN, DocField::IntArray(vec![5, 6, 7]));
        expect(match_conditions(&cond, &DocField::Int(6)), true, false);
        expect(match_conditions(&cond, &DocField::Int(4)), false, false);
    }

    #[test]
    fn test_in_float() {
        let cond = Condition::Op(Op::IN, DocField::FloatArray(vec![5.0, 6.0, 7.0]));
        expect(match_conditions(&cond, &DocField::Float(6.0)), true, false);
        expect(match_conditions(&cond, &DocField::Float(4.0)), false, false);
    }

    #[test]
    fn test_in_str() {
        let in_field = DocField::StringArray(vec![
            "str1".to_string(),
            "str2".to_string(),
            "str3".to_string(),
        ]);
        let cond = Condition::Op(Op::IN, in_field);
        expect(
            match_conditions(&cond, &DocField::String("str2".to_string())),
            true,
            false,
        );
        expect(
            match_conditions(&cond, &DocField::String("str4".to_string())),
            false,
            false,
        );
    }

    #[test]
    fn test_in_time() {
        let cond = Condition::Op(
            Op::IN,
            DocField::DateTimeArray(vec![now_time(), yesterday_time()]),
        );
        expect(match_conditions(&cond, &yesterday()), true, false);

        let tomorrow = DocField::DateTime(now_time() + Duration::days(1));
        expect(match_conditions(&cond, &tomorrow), false, false);
    }

    #[test]
    fn test_nin() {
        let cond = Condition::Op(Op::NIN, DocField::IntArray(vec![5, 6, 7]));
        expect(match_conditions(&cond, &DocField::Int(4)), true, false);
        expect(match_conditions(&cond, &DocField::Int(6)), false, false);
    }

    #[test]
    fn test_like() {
        let field = DocField::String("Source is the glue of web3".to_string());

        // exact match
        let cond = Condition::Op(Op::LIKE, field.clone());
        expect(match_conditions(&cond, &field), true, false);

        // not exact match
        let cond = Condition::Op(Op::LIKE, DocField::String("Source is the glue".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match prefix
        let cond = Condition::Op(Op::LIKE, DocField::String("Source%".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match suffix
        let cond = Condition::Op(Op::LIKE, DocField::String("%web3".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match contains
        let cond = Condition::Op(Op::LIKE, DocField::String("%glue%".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match start and end with
        let cond = Condition::Op(Op::LIKE, DocField::String("Source%web3".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // case sensitive no match
        let field = DocField::String("SOURCE is the glue of web3".to_string());
        expect(match_conditions(&cond, &field), false, false);

        // case sensitive no prefix match
        let field = DocField::String("SOURCE%".to_string());
        expect(match_conditions(&cond, &field), false, false);
    }

    #[test]
    fn test_nlike() {
        let field = DocField::String("Source is the glue of web3".to_string());

        // exact match
        let cond = Condition::Op(Op::NLIKE, field.clone());
        expect(match_conditions(&cond, &field), false, false);

        // not exact match
        let cond = Condition::Op(
            Op::NLIKE,
            DocField::String("Source is the glue".to_string()),
        );
        expect(match_conditions(&cond, &field), true, false);

        // match prefix
        let cond = Condition::Op(Op::NLIKE, DocField::String("Source%".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match suffix
        let cond = Condition::Op(Op::NLIKE, DocField::String("%web3".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match contains
        let cond = Condition::Op(Op::NLIKE, DocField::String("%glue%".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match start and end with
        let cond = Condition::Op(Op::NLIKE, DocField::String("Source%web3".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // case sensitive no match
        let field = DocField::String("SOURCE is the glue of web3".to_string());
        expect(match_conditions(&cond, &field), true, false);

        // case sensitive no prefix match
        let field = DocField::String("SOURCE%".to_string());
        expect(match_conditions(&cond, &field), true, false);
    }

    #[test]
    fn test_ilike() {
        let field = DocField::String("Source is the glue of web3".to_string());

        // exact match
        let cond = Condition::Op(Op::ILIKE, field.clone());
        expect(match_conditions(&cond, &field), true, false);

        // not exact match
        let cond = Condition::Op(
            Op::ILIKE,
            DocField::String("Source is the glue".to_string()),
        );
        expect(match_conditions(&cond, &field), false, false);

        // match prefix
        let cond = Condition::Op(Op::ILIKE, DocField::String("SOURCE%".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match suffix
        let cond = Condition::Op(Op::ILIKE, DocField::String("%WEB3".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match contains
        let cond = Condition::Op(Op::ILIKE, DocField::String("%GLUE%".to_string()));
        expect(match_conditions(&cond, &field), true, false);

        // match start and end with
        let cond = Condition::Op(Op::ILIKE, DocField::String("source%WEB3".to_string()));
        expect(match_conditions(&cond, &field), true, false);
    }

    #[test]
    fn test_nilike() {
        let field = DocField::String("Source is the glue of web3".to_string());

        // exact match
        let cond = Condition::Op(Op::NILIKE, field.clone());
        expect(match_conditions(&cond, &field), false, false);

        // not exact match
        let cond = Condition::Op(
            Op::NILIKE,
            DocField::String("Source is the glue".to_string()),
        );
        expect(match_conditions(&cond, &field), true, false);

        // match prefix
        let cond = Condition::Op(Op::NILIKE, DocField::String("Source%".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match prefix case-insensitive
        let cond = Condition::Op(Op::NILIKE, DocField::String("SOURCE%".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match suffix
        let cond = Condition::Op(Op::NILIKE, DocField::String("%WEB3".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match contains
        let cond = Condition::Op(Op::NILIKE, DocField::String("%GLUE%".to_string()));
        expect(match_conditions(&cond, &field), false, false);

        // match start and end with
        let cond = Condition::Op(Op::NILIKE, DocField::String("source%WEB3".to_string()));
        expect(match_conditions(&cond, &field), false, false);
    }

    #[test]
    fn test_prop_int() {
        let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, DocField::Int(4))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );
    }

    #[test]
    fn test_prop_float() {
        let cond = Condition::Prop(2, Box::new(Condition::Op(Op::EQ, get_field(2))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::Prop(2, Box::new(Condition::Op(Op::EQ, DocField::Float(9.0))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );
    }

    #[test]
    fn test_prop_str() {
        let cond = Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let str_field = DocField::String("wrong".to_string());
        let cond = Condition::Prop(1, Box::new(Condition::Op(Op::EQ, str_field)));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );
    }

    #[test]
    fn test_prop_bool() {
        let cond = Condition::Prop(3, Box::new(Condition::Op(Op::EQ, get_field(3))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::Prop(3, Box::new(Condition::Op(Op::EQ, DocField::Bool(false))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );
    }

    #[test]
    fn test_prop_time() {
        let cond = Condition::Prop(4, Box::new(Condition::Op(Op::EQ, get_field(4))));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::Prop(4, Box::new(Condition::Op(Op::EQ, yesterday())));
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );
    }

    #[test]
    fn test_and() {
        let cond = Condition::CompOp(
            CompOp::AND,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
                Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
            ],
        );

        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::AND,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, DocField::Int(4)))),
                Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::AND,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
                Condition::Prop(
                    1,
                    Box::new(Condition::Op(Op::EQ, DocField::String("wrong".to_string()))),
                ),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::AND,
            vec![Condition::Prop(
                0,
                Box::new(Condition::Op(Op::EQ, DocField::String("wrong".to_string()))),
            )],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            true,
        );
    }

    #[test]
    fn test_or() {
        let cond = Condition::CompOp(
            CompOp::OR,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
                Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::OR,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, DocField::Int(4)))),
                Condition::Prop(1, Box::new(Condition::Op(Op::EQ, get_field(1)))),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::OR,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0)))),
                Condition::Prop(
                    1,
                    Box::new(Condition::Op(Op::EQ, DocField::String("wrong".to_string()))),
                ),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::OR,
            vec![
                Condition::Prop(0, Box::new(Condition::Op(Op::EQ, DocField::Int(4)))),
                Condition::Prop(
                    1,
                    Box::new(Condition::Op(Op::EQ, DocField::String("wrong".to_string()))),
                ),
            ],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::OR,
            vec![Condition::Prop(
                0,
                Box::new(Condition::Op(Op::EQ, DocField::String("4".to_string()))),
            )],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            true,
        );
    }

    #[test]
    fn test_not() {
        let cond = Condition::CompOp(
            CompOp::NOT,
            vec![Condition::Prop(
                0,
                Box::new(Condition::Op(Op::EQ, get_field(0))),
            )],
        );

        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            false,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::NOT,
            vec![Condition::Prop(
                0,
                Box::new(Condition::Op(Op::EQ, DocField::Int(4))),
            )],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            false,
        );

        let cond = Condition::CompOp(
            CompOp::NOT,
            vec![Condition::Prop(
                0,
                Box::new(Condition::Op(Op::EQ, DocField::String("4".to_string()))),
            )],
        );
        expect(
            match_conditions(&cond, &DocField::Doc(get_doc())),
            true,
            true,
        );
    }

    #[test]
    fn with_doc_arr_at_least_one_should_match() {
        let mut new_doc = get_doc();
        new_doc.fields[0] = DocField::Int(8);

        let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, get_field(0))));
        expect(
            match_conditions(&cond, &DocField::DocArray(vec![new_doc.clone(), get_doc()])),
            true,
            false,
        );

        let cond = Condition::Prop(0, Box::new(Condition::Op(Op::EQ, DocField::Int(4))));
        expect(
            match_conditions(&cond, &DocField::DocArray(vec![new_doc, get_doc()])),
            false,
            false,
        );
    }
}
