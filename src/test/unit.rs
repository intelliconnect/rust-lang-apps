use rusoto_dynamodb::AttributeValue;

use crate::model::dbmethods::{attr_to_string, string_to_attr};

#[test]
fn attr_test() {
    let sample_string = String::from("hello");

    let actual_attribute = string_to_attr(sample_string);

    let expected_attribute = AttributeValue {
        s: Some(String::from("hello")),
        ..Default::default()
    };

    assert_eq!(actual_attribute, expected_attribute)
}

#[test]
#[should_panic]
fn attr_test2() {
    let sample_attribute = AttributeValue {
        ..Default::default()
    };
    let actual_string = attr_to_string(&sample_attribute).unwrap();
    let expected_string = String::from("hello");

    assert_eq!(actual_string, expected_string)
}

#[test]
fn attr_test3() {
    let sample_string = String::from("world");

    let actual_attribute = string_to_attr(sample_string);

    let expected_attribute = AttributeValue {
        s: Some(String::from("hello")),
        ..Default::default()
    };

    assert_ne!(actual_attribute, expected_attribute)
}
