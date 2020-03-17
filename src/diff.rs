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
        out.push_str(&format!(
            "Regression (new or increased crypto exposure): {}\n",
            if self.is_regression() { "YES" } else { "no" }
        ));
        out
    }

    /// Render the diff as a [`Json`] value.
    pub fn to_json(&self) -> Json {
        let mut root = BTreeMap::new();
        root.insert(
            "kind".to_string(),
            Json::String("quantumseal-diff/1".to_string()),
        );
        root.insert("regression".to_string(), Json::Bool(self.is_regression()));

        root.insert("added".to_string(), snapshots_to_json(&self.added));
        root.insert("removed".to_string(), snapshots_to_json(&self.removed));

        let mut changed = Vec::new();
        for c in &self.changed {
            let mut obj = BTreeMap::new();
            obj.insert("id".to_string(), Json::String(c.id.clone()));
            obj.insert("name".to_string(), Json::String(c.name.clone()));
            obj.insert("baseline".to_string(), snapshot_to_json(&c.baseline));
            obj.insert("current".to_string(), snapshot_to_json(&c.current));
            obj.insert(
                "occurrence_delta".to_string(),
                Json::int(c.current.occurrence_count - c.baseline.occurrence_count),
            );
            changed.push(Json::Object(obj));
        }
        root.insert("changed".to_string(), Json::Array(changed));

        let unchanged: Vec<Json> = self
            .unchanged
            .iter()
            .map(|id| Json::String(id.clone()))
            .collect();
        root.insert("unchanged".to_string(), Json::Array(unchanged));

        Json::Object(root)
    }
}

fn snapshots_to_json(items: &[ComponentSnapshot]) -> Json {
    Json::Array(items.iter().map(snapshot_to_json).collect())
}

fn snapshot_to_json(s: &ComponentSnapshot) -> Json {
    let mut obj = BTreeMap::new();
    obj.insert("id".to_string(), Json::String(s.id.clone()));
    obj.insert("name".to_string(), Json::String(s.name.clone()));
    obj.insert("priority".to_string(), Json::int(s.priority));
    obj.insert(
        "occurrence_count".to_string(),
        Json::int(s.occurrence_count),
    );
    obj.insert("file_count".to_string(), Json::int(s.file_count));
    Json::Object(obj)
}

/// Extract component snapshots from a parsed CryptoBOM JSON document.
pub fn snapshots_from_json(doc: &Json) -> Result<BTreeMap<String, ComponentSnapshot>, String> {
    let obj = doc.as_object().ok_or("baseline root is not an object")?;
    let components = obj
        .get("components")
        .and_then(|c| c.as_array())
        .ok_or("baseline has no 'components' array")?;

    let mut map = BTreeMap::new();
    for comp in components {
        let co = comp.as_object().ok_or("component is not an object")?;
        let id = co
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or("component missing 'id'")?
            .to_string();
        let name = co
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(&id)
            .to_string();
        let priority = co.get("priority").and_then(|v| v.as_i64()).unwrap_or(0);
        let occurrence_count = co
            .get("occurrence_count")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let file_count = co.get("file_count").and_then(|v| v.as_i64()).unwrap_or(0);
        map.insert(
            id.clone(),
            ComponentSnapshot {
                id,
                name,
                priority,
                occurrence_count,
                file_count,
            },
        );
    }
    Ok(map)
}

/// Build snapshots directly from an in-memory CryptoBOM.
pub fn snapshots_from_bom(bom: &CryptoBom) -> BTreeMap<String, ComponentSnapshot> {
    let mut map = BTreeMap::new();
    for c in &bom.components {
        map.insert(
            c.rule_id.to_string(),
            ComponentSnapshot {
                id: c.rule_id.to_string(),
                name: c.name.to_string(),
                priority: c.priority as i64,
                occurrence_count: c.occurrences.len() as i64,
                file_count: c.file_count as i64,
            },
        );
    }
    map
}

