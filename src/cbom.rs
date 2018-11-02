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

/// Migration priority band derived from the score.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityBand {
    Informational,
    Low,
    Medium,
    High,
    Critical,
}

impl PriorityBand {
    pub fn from_score(score: u32) -> PriorityBand {
        match score {
            0..=9 => PriorityBand::Informational,
            10..=34 => PriorityBand::Low,
            35..=59 => PriorityBand::Medium,
            60..=84 => PriorityBand::High,
            _ => PriorityBand::Critical,
        }
    }

    pub fn code(self) -> &'static str {
        match self {
            PriorityBand::Informational => "informational",
            PriorityBand::Low => "low",
            PriorityBand::Medium => "medium",
            PriorityBand::High => "high",
            PriorityBand::Critical => "critical",
        }
    }
}

/// The complete CryptoBOM.
#[derive(Debug, Clone)]
pub struct CryptoBom {
    pub tool: String,
    pub tool_version: String,
    pub schema: String,
    pub root: String,
    pub files_scanned: usize,
    pub entries_visited: usize,
    pub components: Vec<Component>,
}

impl CryptoBom {
    /// Build a CryptoBOM from a scan result.
    pub fn from_scan(root: &str, scan: &ScanResult) -> CryptoBom {
        // Group matches by rule id.
        let mut by_rule: BTreeMap<&'static str, Vec<Occurrence>> = BTreeMap::new();
        let mut files_by_rule: BTreeMap<&'static str, std::collections::BTreeSet<String>> =
            BTreeMap::new();

        for file in &scan.files {
            let path = file.path.to_string_lossy().replace('\\', "/");
            for m in &file.matches {
                by_rule.entry(m.rule_id).or_default().push(Occurrence {
                    file: path.clone(),
                    line: m.line_number,
                    excerpt: m.excerpt.clone(),
                    needle: m.needle.clone(),
                });
                files_by_rule
                    .entry(m.rule_id)
                    .or_default()
                    .insert(path.clone());
            }
        }

        let mut components = Vec::new();
        for rule in RULES {
            if let Some(occurrences) = by_rule.get(rule.id) {
                let file_count = files_by_rule.get(rule.id).map(|s| s.len()).unwrap_or(0);
                let priority = compute_priority(rule.risk, occurrences.len(), file_count);
                components.push(Component {
                    rule_id: rule.id,
                    name: rule.name,
                    category: rule.category,
                    risk: rule.risk,
                    guidance: rule.guidance,
                    occurrences: occurrences.clone(),
                    file_count,
                    priority,
