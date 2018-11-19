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
                });
            }
        }

        // Sort by descending priority, then by name for stability.
        components.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| a.name.cmp(b.name)));

        CryptoBom {
            tool: "quantumseal".to_string(),
            tool_version: env!("CARGO_PKG_VERSION").to_string(),
            schema: SCHEMA_VERSION.to_string(),
            root: root.replace('\\', "/"),
            files_scanned: scan.files_scanned,
            entries_visited: scan.entries_visited,
            components,
        }
    }

    /// Total occurrences across all components.
    pub fn total_occurrences(&self) -> usize {
        self.components.iter().map(|c| c.occurrences.len()).sum()
    }

    /// Highest priority band present, if any.
    pub fn top_band(&self) -> Option<PriorityBand> {
        self.components
            .iter()
            .map(|c| PriorityBand::from_score(c.priority))
            .max_by_key(|b| *b as u8)
    }

    /// Serialize to the internal [`Json`] value.
    pub fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert("tool".to_string(), Json::String(self.tool.clone()));
        root.insert(
            "tool_version".to_string(),
            Json::String(self.tool_version.clone()),
        );
        root.insert("schema".to_string(), Json::String(self.schema.clone()));
        root.insert("root".to_string(), Json::String(self.root.clone()));
        root.insert("analysis_only".to_string(), Json::Bool(true));
        root.insert(
            "disclaimer".to_string(),
            Json::String(
                "Static analysis of cryptographic indicators only. Not a cryptographic implementation and not a security audit."
                    .to_string(),
            ),
        );

        let mut summary = BTreeMap::new();
        summary.insert(
            "files_scanned".to_string(),
            Json::int(self.files_scanned as i64),
        );
        summary.insert(
            "entries_visited".to_string(),
            Json::int(self.entries_visited as i64),
        );
        summary.insert(
            "component_count".to_string(),
            Json::int(self.components.len() as i64),
        );
        summary.insert(
            "total_occurrences".to_string(),
            Json::int(self.total_occurrences() as i64),
        );

        // Band histogram.
        let mut bands: BTreeMap<String, i64> = BTreeMap::new();
        for c in &self.components {
            *bands
                .entry(PriorityBand::from_score(c.priority).code().to_string())
                .or_insert(0) += 1;
        }
        let mut band_obj = BTreeMap::new();
        for (k, v) in bands {
            band_obj.insert(k, Json::int(v));
        }
        summary.insert("priority_bands".to_string(), Json::Object(band_obj));
        root.insert("summary".to_string(), Json::Object(summary));

        let mut comps = Vec::new();
        for c in &self.components {
            comps.push(component_to_json(c));
        }
        root.insert("components".to_string(), Json::Array(comps));

        Json::Object(root)
    }

    /// Serialize to a pretty JSON string.
    pub fn to_json_string(&self) -> String {
        self.to_json().to_pretty_string()
    }

    /// Render a human-readable text report.
    pub fn to_text(&self) -> String {
        let mut out = String::new();
        out.push_str("quantumseal — CryptoBOM (post-quantum migration inventory)\n");
        out.push_str("NOTE: static analysis of crypto indicators only; not a security audit.\n");
        out.push_str(&"=".repeat(70));
        out.push('\n');
        out.push_str(&format!("Root:            {}\n", self.root));
        out.push_str(&format!(
            "Tool:            {} v{}\n",
            self.tool, self.tool_version
        ));
        out.push_str(&format!("Files scanned:   {}\n", self.files_scanned));
        out.push_str(&format!("Entries visited: {}\n", self.entries_visited));
        out.push_str(&format!("Components:      {}\n", self.components.len()));
        out.push_str(&format!("Occurrences:     {}\n", self.total_occurrences()));
        out.push('\n');

        if self.components.is_empty() {
            out.push_str("No cryptographic indicators detected.\n");
            return out;
        }

        for c in &self.components {
            let band = PriorityBand::from_score(c.priority);
            out.push_str(&"-".repeat(70));
            out.push('\n');
            out.push_str(&format!(
                "[{:>3}] {} — {}\n",
                c.priority,
                c.name,
                band.code().to_uppercase()
            ));
            out.push_str(&format!(
                "      risk={}  category={}  files={}  occurrences={}\n",
                c.risk.code(),
                c.category.code(),
                c.file_count,
                c.occurrences.len()
            ));
