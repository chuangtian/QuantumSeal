//! CryptoBOM: the Cryptographic Bill of Materials.
//!
//! This module turns raw scanner findings into a structured, prioritized
//! inventory and serializes it to JSON or a human-readable text report.

use std::collections::BTreeMap;

use crate::json::Json;
use crate::rules::{Category, QuantumRisk, RULES};
use crate::scanner::ScanResult;

/// The schema version embedded in emitted documents.
pub const SCHEMA_VERSION: &str = "quantumseal-cbom/1";

/// A per-file occurrence of a component.
#[derive(Debug, Clone, PartialEq)]
pub struct Occurrence {
    pub file: String,
    pub line: usize,
    pub excerpt: String,
    pub needle: String,
}

/// One cryptographic component aggregated across the whole scan.
#[derive(Debug, Clone)]
pub struct Component {
    pub rule_id: &'static str,
    pub name: &'static str,
    pub category: Category,
    pub risk: QuantumRisk,
    pub guidance: &'static str,
    pub occurrences: Vec<Occurrence>,
    /// Number of distinct files this component appears in.
    pub file_count: usize,
    /// Computed migration priority score (0-100).
    pub priority: u32,
}

