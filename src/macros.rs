/// Macro to convert a [`Result<T, serde_yaml::Error>`] into a [`cdumay_core::Result<T>`]
#[macro_export]
macro_rules! convert_yaml_result {
    ($result:expr, $context:expr, $text:expr) => {
        $result.map_err(|err| {
            cdumay_yaml::YamlErrorConverter::convert_error(&err, Some($text.to_string()), $context)
        })
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|err| {
            cdumay_yaml::YamlErrorConverter::convert_error(&err, None, $context)
        })
    };
    ($result:expr) => {
        $result.map_err(|err| {
            cdumay_yaml::YamlErrorConverter::convert_error(&err, None, std::collections::BTreeMap::new())
        })
    };
}