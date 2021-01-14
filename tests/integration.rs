//! End-to-end integration tests that scan the bundled fixtures.

use std::path::Path;

use quantumseal::cbom::{CryptoBom, PriorityBand};
use quantumseal::diff;
use quantumseal::scanner::{self, ScanOptions};

fn scan_fixture(rel: &str) -> CryptoBom {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(rel);
    let scan = scanner::scan(&root, &ScanOptions::default()).expect("scan should succeed");
    CryptoBom::from_scan(rel, &scan)
}

#[test]
fn legacy_fixture_flags_high_risk_components() {
    let bom = scan_fixture("fixtures/legacy_service");
    let ids: Vec<&str> = bom.components.iter().map(|c| c.rule_id).collect();
