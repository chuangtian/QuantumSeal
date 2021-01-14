//! End-to-end integration tests that scan the bundled fixtures.

use std::path::Path;

use quantumseal::cbom::{CryptoBom, PriorityBand};
use quantumseal::diff;
use quantumseal::scanner::{self, ScanOptions};

fn scan_fixture(rel: &str) -> CryptoBom {
