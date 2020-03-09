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
pub struct ChangedComponent {
    pub id: String,
    pub name: String,
    pub baseline: ComponentSnapshot,
    pub current: ComponentSnapshot,
}

impl Diff {
    /// Whether anything changed at all.
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.changed.is_empty()
    }

    /// True when the current BOM is strictly "better or equal": nothing new was
    /// added and no occurrence counts increased. Useful for CI gates.
    pub fn is_regression(&self) -> bool {
        if !self.added.is_empty() {
            return true;
        }
        self.changed
            .iter()
            .any(|c| c.current.occurrence_count > c.baseline.occurrence_count)
    }

    /// Render a text summary.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("quantumseal — CryptoBOM baseline comparison\n");
        out.push_str(&"=".repeat(70));
        out.push('\n');

        if self.is_empty() {
            out.push_str("No changes: current inventory matches the baseline.\n");
            return out;
        }

        if !self.added.is_empty() {
            out.push_str(&format!("\nADDED ({}):\n", self.added.len()));
            for c in &self.added {
                out.push_str(&format!(
                    "  + {} ({})  priority={} occurrences={}\n",
                    c.name, c.id, c.priority, c.occurrence_count
                ));
            }
        }
        if !self.removed.is_empty() {
            out.push_str(&format!("\nREMOVED ({}):\n", self.removed.len()));
            for c in &self.removed {
                out.push_str(&format!(
                    "  - {} ({})  was priority={} occurrences={}\n",
                    c.name, c.id, c.priority, c.occurrence_count
                ));
            }
        }
        if !self.changed.is_empty() {
            out.push_str(&format!("\nCHANGED ({}):\n", self.changed.len()));
            for c in &self.changed {
                let occ_delta = c.current.occurrence_count - c.baseline.occurrence_count;
                let arrow = if occ_delta > 0 { "▲" } else { "▼" };
                out.push_str(&format!(
                    "  ~ {} ({}) {} occurrences {}→{} ({:+}), priority {}→{}\n",
                    c.name,
                    c.id,
                    arrow,
                    c.baseline.occurrence_count,
                    c.current.occurrence_count,
                    occ_delta,
                    c.baseline.priority,
                    c.current.priority,
                ));
            }
        }
        out.push('\n');
        out.push_str(&"=".repeat(70));
        out.push('\n');
