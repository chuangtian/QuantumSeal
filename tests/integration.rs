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

    // Expect Shor-breakable and deprecated primitives.
    assert!(ids.contains(&"rsa"), "expected RSA, got {ids:?}");
    assert!(ids.contains(&"ecc"), "expected ECC, got {ids:?}");
    assert!(ids.contains(&"md5"), "expected MD5, got {ids:?}");
    assert!(ids.contains(&"3des"), "expected 3DES, got {ids:?}");
    assert!(ids.contains(&"rc4"), "expected RC4, got {ids:?}");

    // The top band should be Critical (md5/sha1/3des/rc4 present).
    assert_eq!(bom.top_band(), Some(PriorityBand::Critical));

    // Components are sorted by descending priority.
    let priorities: Vec<u32> = bom.components.iter().map(|c| c.priority).collect();
    let mut sorted = priorities.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(priorities, sorted);
}

#[test]
fn migrated_fixture_flags_pqc_components() {
    let bom = scan_fixture("fixtures/migrated_service");
    let ids: Vec<&str> = bom.components.iter().map(|c| c.rule_id).collect();
    assert!(ids.contains(&"mlkem"), "expected ML-KEM, got {ids:?}");
    assert!(ids.contains(&"mldsa"), "expected ML-DSA, got {ids:?}");
    assert!(ids.contains(&"slhdsa"), "expected SLH-DSA, got {ids:?}");
    assert!(ids.contains(&"aes"), "expected AES, got {ids:?}");
}

#[test]
fn clean_fixture_has_no_findings() {
    let bom = scan_fixture("fixtures/clean_service");
    assert!(
        bom.components.is_empty(),
        "clean fixture should produce no findings, got {:?}",
        bom.components.iter().map(|c| c.rule_id).collect::<Vec<_>>()
    );
}

#[test]
fn json_roundtrips_and_is_parseable() {
    let bom = scan_fixture("fixtures/legacy_service");
    let json_text = bom.to_json_string();
    // Ensure it parses back with our own parser.
    let parsed = quantumseal::json::parse(&json_text).expect("emitted JSON must parse");
    let obj = parsed.as_object().unwrap();
    assert_eq!(
        obj.get("analysis_only").unwrap(),
        &quantumseal::json::Json::Bool(true)
    );
    assert!(obj.get("components").unwrap().as_array().unwrap().len() >= 5);
}

#[test]
fn diff_detects_migration_progress() {
    // Baseline = legacy service; current = migrated service.
    let baseline = scan_fixture("fixtures/legacy_service");
    let current = scan_fixture("fixtures/migrated_service");

    let baseline_json = baseline.to_json_string();
    let d = diff::compare(&baseline_json, &current).expect("diff should succeed");

    // RSA/MD5/etc. were removed; PQC algorithms were added.
    let removed_ids: Vec<&str> = d.removed.iter().map(|c| c.id.as_str()).collect();
    assert!(removed_ids.contains(&"rsa"));
    assert!(removed_ids.contains(&"md5"));

    let added_ids: Vec<&str> = d.added.iter().map(|c| c.id.as_str()).collect();
    assert!(added_ids.contains(&"mlkem"));

    // Adding new components counts as "regression" in the neutral sense of new
    // crypto exposure; the migrated code introduces new (PQC) components.
    assert!(!d.is_empty());
}
