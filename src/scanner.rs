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

