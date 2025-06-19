use cdumay_core::ErrorConverter;
use cdumay_yaml::convert_yaml_result;
use std::collections::BTreeMap;

#[test]
fn test_convert_result_with_context() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));

    let converted = convert_yaml_result!(result, context, "Test error");
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.message().contains("Test error"));
}

#[test]
fn test_convert_result_without_text() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let mut context = BTreeMap::new();
    context.insert("test".to_string(), serde_value::Value::String("value".to_string()));
    let converted = convert_yaml_result!(result, context);
    assert!(converted.is_err());

    let err = converted.unwrap_err();
    assert!(err.details().contains_key("test"));
}

#[test]
fn test_convert_result_minimal() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("invalid: yaml: :");
    let converted = convert_yaml_result!(result);
    assert!(converted.is_err());
}

#[test]
fn test_convert_result_success() {
    let result = serde_yaml::from_str::<serde_yaml::Value>("valid: yaml");
    let converted = convert_yaml_result!(result);
    assert!(converted.is_ok());
}
