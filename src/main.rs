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
            }
        } else {
            positionals.push(arg.clone());
        }
        i += 1;
    }
    Ok(Args { positionals, flags })
}

fn cmd_scan(args: &[String]) -> Result<ExitCode, String> {
    let parsed = parse_args(
        args,
        &["format", "output", "max-depth"],
        &["follow-symlinks"],
    )?;

    let path = parsed
        .positionals
        .first()
        .cloned()
        .ok_or("scan requires a PATH argument")?;
    let root = PathBuf::from(&path);
    if !root.exists() {
        return Err(format!("path does not exist: {path}"));
    }

    let format = parsed
        .flags
        .get("format")
        .and_then(|v| v.clone())
        .unwrap_or_else(|| "text".to_string());

    let max_depth = match parsed.flags.get("max-depth").and_then(|v| v.clone()) {
        Some(s) => Some(
            s.parse::<usize>()
                .map_err(|_| "--max-depth must be a non-negative integer")?,
        ),
        None => None,
    };

    let options = ScanOptions {
        follow_symlinks: parsed.flags.contains_key("follow-symlinks"),
        max_depth,
    };

    let scan = scanner::scan(&root, &options).map_err(|e| format!("scan failed: {e}"))?;
    let bom = CryptoBom::from_scan(&path, &scan);

    let rendered = match format.as_str() {
        "json" => bom.to_json_string(),
        "text" => bom.to_text(),
        other => return Err(format!("unknown format '{other}' (use json|text)")),
    };

    if let Some(Some(out)) = parsed.flags.get("output") {
        std::fs::write(out, rendered.as_bytes())
            .map_err(|e| format!("failed to write {out}: {e}"))?;
        eprintln!("wrote CryptoBOM to {out}");
    } else {
        print!("{rendered}");
        if !rendered.ends_with('\n') {
            println!();
        }
    }

    Ok(ExitCode::SUCCESS)
}

fn cmd_diff(args: &[String]) -> Result<ExitCode, String> {
    let parsed = parse_args(args, &["path", "format"], &["fail-on-regression"])?;

    let baseline_path = parsed
        .positionals
        .first()
        .cloned()
        .ok_or("diff requires a BASELINE json file argument")?;
    let baseline_text = std::fs::read_to_string(&baseline_path)
        .map_err(|e| format!("failed to read baseline {baseline_path}: {e}"))?;

    let scan_path = parsed
        .flags
        .get("path")
        .and_then(|v| v.clone())
        .unwrap_or_else(|| ".".to_string());
    let root = Path::new(&scan_path);
    if !root.exists() {
        return Err(format!("scan path does not exist: {scan_path}"));
    }

    let format = parsed
        .flags
        .get("format")
        .and_then(|v| v.clone())
        .unwrap_or_else(|| "text".to_string());

    let scan =
        scanner::scan(root, &ScanOptions::default()).map_err(|e| format!("scan failed: {e}"))?;
    let current = CryptoBom::from_scan(&scan_path, &scan);

    let diff = diff::compare(&baseline_text, &current)?;

    let rendered = match format.as_str() {
        "json" => diff.to_json().to_pretty_string(),
