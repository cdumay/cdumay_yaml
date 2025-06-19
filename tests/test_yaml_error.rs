use cdumay_core::ErrorConverter;
use cdumay_yaml::YamlErrorConverter;
use serde_value::Value;
use std::collections::BTreeMap;

#[test]
fn test_invalid_yaml_returns_custom_error_with_message() {
    let invalid_yaml = "invalid: yaml: :"; // malformed input
    let parse_result = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml);
    assert!(parse_result.is_err());

    let err = parse_result.unwrap_err();
    let mut context = BTreeMap::new();
    context.insert("file".to_string(), Value::String("config.yaml".to_string()));

    let custom_error = YamlErrorConverter::convert_error(&err, Some("Custom YAML parsing failed".to_string()), context.clone());

    assert_eq!(custom_error.message(), "Custom YAML parsing failed");

    let details = custom_error.details();
    assert!(details.contains_key("file"));
    assert!(details.contains_key("origin"));
}

#[test]
fn test_invalid_yaml_returns_error_with_default_message() {
    let invalid_yaml = "---\ninvalid_yaml: [unterminated"; // bad structure
    let result = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml);
    assert!(result.is_err());

    let err = result.unwrap_err();
    let context = BTreeMap::new();

    let custom_error = YamlErrorConverter::convert_error(&err, None, context.clone());

    assert_eq!(custom_error.message(), err.to_string());
    assert!(custom_error.details().is_empty()); // no context added
}

#[test]
fn test_valid_yaml_does_not_trigger_error() {
    let valid_yaml = "---\nkey: value";
    let result = serde_yaml::from_str::<serde_yaml::Value>(valid_yaml);
    assert!(result.is_ok());
}
