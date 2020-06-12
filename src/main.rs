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

COMMANDS:
    scan <PATH>          Inventory PATH recursively and emit a CryptoBOM.
    diff <BASELINE>      Scan and compare against a saved baseline CryptoBOM (JSON).
    rules                Print the built-in crypto indicator catalog.
    help                 Show this help.

SCAN OPTIONS:
    --format <json|text>     Output format (default: text).
    --output <FILE>          Write output to FILE instead of stdout.
    --max-depth <N>          Limit recursion depth.
    --follow-symlinks        Follow symbolic links (off by default).

DIFF OPTIONS:
    --path <PATH>            Directory to scan for the current state (default: .).
    --format <json|text>     Output format (default: text).
    --fail-on-regression     Exit non-zero if new/increased crypto exposure is found.

EXAMPLES:
    quantumseal scan ./src --format json --output cbom.json
    quantumseal diff cbom.json --path ./src --fail-on-regression
    quantumseal rules

NOTE: quantumseal analyzes textual indicators of cryptography to help plan a
post-quantum migration. It is NOT a cryptographic implementation or a security
audit; every finding warrants human review.
"#;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &[String]) -> Result<ExitCode, String> {
    let Some(command) = args.first() else {
        print!("{USAGE}");
        return Ok(ExitCode::from(1));
    };

    match command.as_str() {
        "scan" => cmd_scan(&args[1..]),
        "diff" => cmd_diff(&args[1..]),
        "rules" => cmd_rules(),
        "help" | "-h" | "--help" => {
            print!("{USAGE}");
            Ok(ExitCode::SUCCESS)
        }
        "--version" | "-V" => {
            println!("quantumseal {}", env!("CARGO_PKG_VERSION"));
            Ok(ExitCode::SUCCESS)
        }
        other => Err(format!(
            "unknown command '{other}'. Run 'quantumseal help'."
        )),
    }
}

/// Simple flag/positional splitter.
struct Args {
    positionals: Vec<String>,
    flags: std::collections::BTreeMap<String, Option<String>>,
}

fn parse_args(args: &[String], valued: &[&str], boolean: &[&str]) -> Result<Args, String> {
    let mut positionals = Vec::new();
    let mut flags = std::collections::BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        if let Some(name) = arg.strip_prefix("--") {
            if boolean.contains(&name) {
                flags.insert(name.to_string(), None);
            } else if valued.contains(&name) {
                let value = args
                    .get(i + 1)
                    .ok_or_else(|| format!("flag --{name} requires a value"))?;
                flags.insert(name.to_string(), Some(value.clone()));
                i += 1;
            } else {
                return Err(format!("unknown flag --{name}"));
