//! Recursive file inventory and indicator matching.
//!
//! The scanner walks a directory tree (standard library only), reads text-like
//! files, lowercases each line once, and tests every rule's needles against it.
//! Matches are grouped into [`Finding`]s that are later assembled into the
//! CryptoBOM.

use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

use crate::rules::{Rule, RULES};

/// File extensions considered "scannable" text. Everything else is skipped so
/// binaries do not pollute the inventory.
const TEXT_EXTENSIONS: &[&str] = &[
    "rs",
    "ts",
    "tsx",
    "js",
    "jsx",
    "mjs",
    "cjs",
    "py",
    "go",
    "java",
    "kt",
    "c",
    "h",
    "cc",
    "cpp",
    "hpp",
    "cs",
    "rb",
    "php",
    "swift",
    "scala",
    "sh",
    "bash",
    "zsh",
    "ps1",
    "toml",
    "yaml",
    "yml",
    "json",
    "ini",
    "cfg",
    "conf",
    "env",
    "properties",
    "xml",
    "gradle",
    "tf",
    "hcl",
    "pem",
    "crt",
    "key",
    "txt",
    "md",
    "dockerfile",
    "makefile",
    "sql",
];

/// Directory names skipped during the walk.
const SKIP_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "target",
    "dist",
    "build",
    ".venv",
    "venv",
    "__pycache__",
    ".idea",
    ".vscode",
    "vendor",
    ".next",
    "out",
];

/// Maximum bytes read per file (guards against huge/binary files).
const MAX_FILE_BYTES: usize = 4 * 1024 * 1024;

/// One matched indicator occurrence within a file.
#[derive(Debug, Clone, PartialEq)]
pub struct Match {
    pub rule_id: &'static str,
    pub line_number: usize,
    /// The trimmed, length-capped source line, for context.
    pub excerpt: String,
    /// The specific needle that fired.
    pub needle: String,
}

/// All matches for a single file.
#[derive(Debug, Clone)]
pub struct FileFindings {
    pub path: PathBuf,
    pub matches: Vec<Match>,
}

/// Aggregated scan result.
#[derive(Debug, Clone, Default)]
pub struct ScanResult {
    pub files: Vec<FileFindings>,
    /// Count of files inspected (text files opened).
    pub files_scanned: usize,
    /// Count of filesystem entries visited overall.
    pub entries_visited: usize,
}

/// Options controlling a scan.
///
/// The [`Default`] is a non-following, unlimited-depth walk.
#[derive(Debug, Clone, Default)]
pub struct ScanOptions {
    /// Follow symbolic links. Off by default to avoid cycles.
    pub follow_symlinks: bool,
    /// Maximum recursion depth (`None` = unlimited).
    pub max_depth: Option<usize>,
}

/// Scan `root` recursively and return all findings.
pub fn scan(root: &Path, options: &ScanOptions) -> io::Result<ScanResult> {
    let mut result = ScanResult::default();
    walk(root, 0, options, &mut result)?;
    // Deterministic ordering for reproducible output.
    result.files.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(result)
}

fn walk(
    dir: &Path,
    depth: usize,
    options: &ScanOptions,
    result: &mut ScanResult,
) -> io::Result<()> {
    if let Some(max) = options.max_depth {
        if depth > max {
            return Ok(());
        }
    }

    // A single file target is also valid input.
    let meta = fs::symlink_metadata(dir)?;
    if meta.file_type().is_file() {
        result.entries_visited += 1;
        if let Some(ff) = scan_file(dir)? {
            result.files_scanned += 1;
            if !ff.matches.is_empty() {
                result.files.push(ff);
            }
        } else {
            result.files_scanned += 0;
        }
        return Ok(());
    }

    if !meta.file_type().is_dir() {
        return Ok(());
