//! A lightweight module that converts YAML serialization and deserialization errors (`serde_yaml::Error`) into structured, typed errors using the [`cdumay_core`](https://!docs.rs/cdumay_core/) framework.
//!
//! This helps standardize error handling for Rust applications that deal with YAML configuration or data files, while enriching error details with structured context.
//!
//! ## Features
//!
//! - Converts YAML-related errors into a standardized error format
//! - Provides unique error codes, HTTP status codes, and descriptions
//! - Supports rich contextual error metadata via `BTreeMap`
//! - Integrates easily with the `cdumay_core::ErrorConverter` trait
//! - Provides a convenient `convert_yaml_result!` macro for error conversion
//!
//! ## Usage Example
//!
//! ### Dependencies
//!
//! ```toml
//! [dependencies]
//! cdumay_core = "1.0"
//! serde = { version = "1.0", features = ["derive"] }
//! serde-value = "0.7"
//! serde_yaml = "0.8"
//! ```
//!
//! ### Code sample
//!
//! Using the `YamlErrorConverter` directly:
//! ```rust
//! use cdumay_core::ErrorConverter;
//! use std::collections::BTreeMap;
//! use serde::{Deserialize, Serialize};
//! use cdumay_yaml::YamlErrorConverter;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     debug: bool,
//! }
//!
//! fn serialize_config(config: &Config) -> cdumay_core::Result<String> {
//!     serde_yaml::to_string(config).map_err(|e| {
//!         let mut ctx = BTreeMap::new();
//!         ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
//!         YamlErrorConverter::convert(&e, "Failed to serialize YAML config".into(), ctx)
//!     })
//! }
//!
//! fn deserialize_config(input: &str) -> cdumay_core::Result<Config> {
//!     serde_yaml::from_str::<Config>(input).map_err(|e| {
//!         let mut ctx = BTreeMap::new();
//!         ctx.insert("input".into(), serde_value::Value::String(input.to_string()));
//!         YamlErrorConverter::convert(&e, "Failed to deserialize YAML config".into(), ctx)
//!     })
//! }
//! ```
//!
//! ## Example Output
//!
//! ```json
//! {
//!   "code": "YAML-00001",
//!   "status": 400,
//!   "kind": "Invalid YAML data",
//!   "message": "Failed to deserialize YAML config",
//!   "context": {
//!     "input": "invalid: yaml"
//!   }
//! }
//! ```
//!
//! Using the `convert_yaml_result!` macro:
//!
//! ```rust
//! use cdumay_core::ErrorConverter;
//! use std::collections::BTreeMap;
//! use serde::{Deserialize, Serialize};
//! use cdumay_yaml::convert_yaml_result;
//!
//! #[derive(Serialize, Deserialize)]
//! struct Config {
//!     name: String,
//!     debug: bool,
//! }
//!
//! fn serialize_config(config: &Config) -> cdumay_core::Result<String> {
//!     let mut ctx = BTreeMap::new();
//!     ctx.insert("config_name".into(), serde_value::Value::String(config.name.clone()));
//!     convert_yaml_result!(serde_yaml::to_string(config), ctx, "Failed to serialize YAML config")
//! }
//!
//! fn deserialize_config(input: &str) -> cdumay_core::Result<Config> {
//!     convert_yaml_result!(serde_yaml::from_str::<Config>(input))
//! }
//! ```
use cdumay_core::{Error, ErrorConverter, define_errors, define_kinds};
use std::collections::BTreeMap;

define_kinds! {
    YamlData = (400, "Invalid YAML data")
}

define_errors! {
    DataError = YamlData,
}

/// Struct providing helper functions to convert `serde_yaml::Error`
/// into typed application errors.
pub struct YamlErrorConverter;

impl ErrorConverter for YamlErrorConverter {
    type Error = serde_yaml::Error;
    /// Converts a `serde_yaml::Error` into a structured application `Error`.
    ///
    /// # Parameters
    /// - `err`: The original `serde_yaml::Error` returned from a YAML operation.
    /// - `text`: Custom error message you wish to associate with the failure.
    /// - `context`: A context to enrich the error with metadata.
    ///
    /// # Returns
    /// A typed `Error` with metadata and details included.
    fn convert(_: &serde_yaml::Error, text: String, context: BTreeMap<String, serde_value::Value>) -> Error {
        DataError::new().with_message(text).with_details(context).into()
    }
}
