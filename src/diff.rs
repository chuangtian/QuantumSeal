//! Baseline comparison.
//!
//! Given a previously emitted CryptoBOM (JSON) and a freshly computed one, this
//! module reports which cryptographic components were added, removed, or had
//! their priority/occurrence counts change. This lets teams track migration
//! progress over time (e.g. "RSA occurrences dropped from 40 to 12").

use std::collections::BTreeMap;

use crate::cbom::CryptoBom;
use crate::json::{self, Json};

/// A single component's state used for comparison.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentSnapshot {
    pub id: String,
    pub name: String,
    pub priority: i64,
    pub occurrence_count: i64,
    pub file_count: i64,
}

/// The result of comparing a baseline to a current BOM.
#[derive(Debug, Clone, Default)]
pub struct Diff {
    pub added: Vec<ComponentSnapshot>,
    pub removed: Vec<ComponentSnapshot>,
    pub changed: Vec<ChangedComponent>,
    pub unchanged: Vec<String>,
}

/// A component present in both, with differing metrics.
#[derive(Debug, Clone, PartialEq)]
