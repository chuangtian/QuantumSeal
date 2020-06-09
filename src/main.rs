//! quantumseal command-line interface.
//!
//! Subcommands:
//!   scan      Inventory a directory and emit a CryptoBOM (JSON or text).
//!   diff      Compare a saved baseline CryptoBOM against a fresh scan.
//!   rules     List the built-in crypto indicator catalog.
//!   help      Show usage.
//!
//! The CLI is written with the standard library only (no clap): a small,
//! explicit argument parser keeps the dependency footprint at zero.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use quantumseal::cbom::CryptoBom;
use quantumseal::diff;
use quantumseal::rules::RULES;
use quantumseal::scanner::{self, ScanOptions};

const USAGE: &str = r#"quantumseal — post-quantum migration workbench (static analysis only)

USAGE:
    quantumseal <COMMAND> [OPTIONS]

