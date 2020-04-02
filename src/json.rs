//! Minimal, dependency-free JSON support.
//!
//! `quantumseal` deliberately relies on the Rust standard library only, so this
//! module provides just enough JSON to serialize the CryptoBOM and to parse a
//! previously emitted baseline back in for comparison.
//!
//! It is not a general-purpose JSON library: it targets the specific document
//! shapes this tool produces and consumes, but the parser is a correct,
//! spec-compliant recursive-descent parser for the JSON grammar it accepts.

use std::collections::BTreeMap;
use std::fmt::Write as _;

/// An owned JSON value.
#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    /// All numbers are stored as `f64`; integers are formatted without a
    /// fractional part when serialized.
    Number(f64),
    String(String),
    Array(Vec<Json>),
    /// Object keys are kept sorted for deterministic output.
    Object(BTreeMap<String, Json>),
}

impl Json {
    /// Convenience constructor for an integer-valued number.
    pub fn int(value: i64) -> Json {
        Json::Number(value as f64)
    }

    /// Borrow the value as a string slice, if it is a string.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Json::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    /// Borrow the value as an object map, if it is an object.
    pub fn as_object(&self) -> Option<&BTreeMap<String, Json>> {
        match self {
            Json::Object(m) => Some(m),
            _ => None,
        }
    }
