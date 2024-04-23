extern crate libc;

use super::core::*;

#[test]
fn test_deserialize_doc() {
    let json_str =
        r#"{"fields":[{"Int":42},{"Doc":{"fields":[{"IntArray":[1,2,3]},{"OptionalInt":null}]}}]}"#;
    let result = deserialize_doc(json_str);

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.fields.len(), 2);

    if let core::doc::Field::Int(value) = &doc.fields[0] {
        assert_eq!(*value, 42);
    } else {
        panic!("Expected Int variant");
    }

    if let core::doc::Field::Doc(doc_box) = &doc.fields[1] {
        assert_eq!(doc_box.fields.len(), 2);

        if let core::doc::Field::IntArray(int_vec) = &doc_box.fields[0] {
            assert_eq!(int_vec.len(), 3);
            assert_eq!(int_vec[0], 1);
            assert_eq!(int_vec[1], 2);
            assert_eq!(int_vec[2], 3);
        } else {
            panic!("Expected IntArray variant");
        }

        if let core::doc::Field::OptionalInt(maybe_int) = &doc_box.fields[1] {
            assert!(maybe_int.is_none());
        } else {
            panic!("Expected OptionalInt variant");
        }
    } else {
        panic!("Expected Doc variant");
    }
}

#[test]
fn deserialize_doc_with_null_fields() {
    let json_str = r#"{"fields":[{"String":"bae-3bfe0092-e31f-5ebe-a3ba-fa18fac448a6"},null,{"Int":65},null]}"#;
    let result = deserialize_doc(json_str);

    assert!(result.is_ok());
    let doc = result.unwrap();
    assert_eq!(doc.fields.len(), 4);

    if let core::doc::Field::String(value) = &doc.fields[0] {
        assert_eq!(*value, "bae-3bfe0092-e31f-5ebe-a3ba-fa18fac448a6");
    } else {
        panic!("Expected String variant");
    }

    if let core::doc::Field::Null = &doc.fields[1] {
    } else {
        panic!("Expected Null variant");
    }

    if let core::doc::Field::Int(value) = &doc.fields[2] {
        assert_eq!(*value, 65);
    } else {
        panic!("Expected Int variant");
    }

    if let core::doc::Field::Null = &doc.fields[3] {
    } else {
        panic!("Expected Null variant");
    }
}
