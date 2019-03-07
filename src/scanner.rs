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
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(()), // unreadable dir: skip gracefully
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        result.entries_visited += 1;

        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };

        if file_type.is_symlink() && !options.follow_symlinks {
            continue;
        }

        let effective_type = if file_type.is_symlink() {
            match fs::metadata(&path) {
                Ok(m) => m.file_type(),
                Err(_) => continue,
            }
        } else {
            file_type
        };

        if effective_type.is_dir() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if SKIP_DIRS.contains(&name) {
                    continue;
                }
            }
            walk(&path, depth + 1, options, result)?;
        } else if effective_type.is_file() {
            if let Some(ff) = scan_file(&path)? {
                result.files_scanned += 1;
                if !ff.matches.is_empty() {
                    result.files.push(ff);
                }
            }
        }
    }
    Ok(())
}

/// Decide whether a path is a text file we should scan.
fn is_scannable(path: &Path) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_lowercase();
        if TEXT_EXTENSIONS.contains(&ext.as_str()) {
            return true;
        }
    }
    // Extensionless well-known files.
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        let lower = name.to_lowercase();
        if matches!(
            lower.as_str(),
            "dockerfile" | "makefile" | ".env" | ".gitignore" | ".gitattributes"
        ) {
            return true;
        }
    }
    false
}

/// Read and scan a single file. Returns `Ok(None)` if the file is skipped.
fn scan_file(path: &Path) -> io::Result<Option<FileFindings>> {
    if !is_scannable(path) {
        return Ok(None);
    }

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return Ok(None),
    };

    let mut buf = Vec::new();
    let mut limited = (&mut file).take(MAX_FILE_BYTES as u64);
    if limited.read_to_end(&mut buf).is_err() {
        return Ok(None);
    }

    // Reject files that look binary (contain NUL in the first chunk).
    let probe_len = buf.len().min(8000);
    if buf[..probe_len].contains(&0) {
        return Ok(None);
    }

    let text = match String::from_utf8(buf) {
        Ok(t) => t,
        Err(e) => String::from_utf8_lossy(e.as_bytes()).into_owned(),
    };

    let matches = match_lines(&text);
    Ok(Some(FileFindings {
        path: path.to_path_buf(),
        matches,
    }))
}

/// Match all rules against every line of `text`.
pub fn match_lines(text: &str) -> Vec<Match> {
    let mut matches = Vec::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let lower = raw_line.to_lowercase();
        for rule in RULES {
            // Skip this rule entirely if the line contains a disambiguating
            // token (e.g. don't flag "dsa" on a line that says "ml-dsa").
            if rule
                .exclude_if_line_contains
                .iter()
                .any(|token| lower.contains(token))
            {
                continue;
            }
            for needle in rule.needles {
                if contains_indicator(&lower, needle) {
                    matches.push(Match {
                        rule_id: rule.id,
                        line_number: idx + 1,
                        excerpt: excerpt(raw_line),
                        needle: (*needle).to_string(),
                    });
                    // One match per rule per line is enough signal.
                    break;
                }
            }
        }
    }
    matches
}

/// Substring search with class-aware boundary checks to reduce false positives.
///
/// The needle is rejected as an embedded fragment when the character adjacent
/// to it belongs to the *same character class* as the needle's own boundary
/// character:
///   * an alphabetic boundary must not be flanked by another letter
///     (so "dss" does not match inside "oddssue"), and
///   * a digit boundary must not be flanked by another digit
///     (so "sha3" does not match inside "sha384").
///
/// Underscores and separators (`-`, space) are always acceptable boundaries, so
/// identifiers like `generate_rsa_2048`, `ML-KEM-768`, and `Dilithium3` match.
/// Needles that contain a separator are specific enough that only the class of
/// their first/last character is checked at each end.
fn contains_indicator(haystack: &str, needle: &str) -> bool {
    if needle.is_empty() {
        return false;
    }

    let hay = haystack.as_bytes();
    let ndl = needle.as_bytes();
    let first = ndl[0];
    let last = ndl[ndl.len() - 1];

    let mut start = 0;
    while let Some(rel) = find_subslice(&hay[start..], ndl) {
        let pos = start + rel;
        let after_idx = pos + ndl.len();

        let before_ok = pos == 0 || !same_class_break(hay[pos - 1], first);
        let after_ok = after_idx >= hay.len() || !same_class_break(hay[after_idx], last);

        if before_ok && after_ok {
            return true;
        }
        start = pos + 1;
    }
    false
}

/// Returns true when `adjacent` would incorrectly extend a needle whose
/// boundary character is `boundary` — i.e. both are letters, or both are
/// digits. Underscores and separators never break a match.
fn same_class_break(adjacent: u8, boundary: u8) -> bool {
    (boundary.is_ascii_alphabetic() && adjacent.is_ascii_alphabetic())
        || (boundary.is_ascii_digit() && adjacent.is_ascii_digit())
}

/// Naive substring search over byte slices (stdlib has no slice::find for this).
fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.len() > haystack.len() {
        return None;
    }
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

/// Trim and cap a line for use as report context.
fn excerpt(line: &str) -> String {
    let trimmed = line.trim();
    const MAX: usize = 160;
    if trimmed.chars().count() <= MAX {
        trimmed.to_string()
    } else {
        let capped: String = trimmed.chars().take(MAX).collect();
        format!("{capped}…")
    }
}

/// Convenience: count matches per rule id across a scan result.
pub fn matches_per_rule(result: &ScanResult) -> BTreeMap<&'static str, usize> {
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    for file in &result.files {
        for m in &file.matches {
            *counts.entry(m.rule_id).or_insert(0) += 1;
        }
    }
    counts
}

/// Look up the rule struct for a match's rule id (always succeeds for matches
/// produced by [`match_lines`]).
pub fn rule_for(m: &Match) -> &'static Rule {
    RULES
        .iter()
        .find(|r| r.id == m.rule_id)
        .expect("match carries a valid rule id")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_rsa_and_ecc() {
        let text = "let key = generate_rsa_2048();\nuse ECDSA for signing";
        let matches = match_lines(text);
        let ids: Vec<&str> = matches.iter().map(|m| m.rule_id).collect();
        assert!(ids.contains(&"rsa"));
        assert!(ids.contains(&"ecc"));
    }

    #[test]
    fn word_boundaries_reduce_false_positives() {
        // "medsafe" should not match the "dsa" needle... but note our dsa rule
        // uses multi-char needles like "dss"; verify "dss" doesn't match inside
        // a larger word.
        let text = "address the oddssue";
        let matches = match_lines(text);
        assert!(matches.iter().all(|m| m.rule_id != "dsa"));
    }

    #[test]
    fn detects_post_quantum_algorithms() {
        let text = "cipher = ML-KEM-768\nsig = Dilithium3";
        let matches = match_lines(text);
        let ids: Vec<&str> = matches.iter().map(|m| m.rule_id).collect();
        assert!(ids.contains(&"mlkem"));
        assert!(ids.contains(&"mldsa"));
    }

    #[test]
    fn excerpt_is_trimmed_and_capped() {
        let long = "x".repeat(500);
        let e = excerpt(&format!("   {long}   "));
        assert!(e.chars().count() <= 161);
    }

    #[test]
    fn subslice_search_works() {
        assert_eq!(find_subslice(b"hello world", b"world"), Some(6));
        assert_eq!(find_subslice(b"abc", b"xyz"), None);
    }

    #[test]
    fn sha3_does_not_match_inside_sha384() {
        // "sha384" must not trigger the SHA-3 rule via the "sha3" needle.
        let matches = match_lines("cipher = ECDHE-RSA-AES256-GCM-SHA384");
        assert!(
            matches.iter().all(|m| m.rule_id != "sha3"),
            "sha3 should not match sha384: {matches:?}"
        );
        // But an explicit SHA-3 usage still matches.
        let matches = match_lines("hash = SHA3-256");
        assert!(matches.iter().any(|m| m.rule_id == "sha3"));
    }

    #[test]
    fn dsa_does_not_match_inside_ecdsa() {
        // "ecdsa-sha2" must not trigger the DSA rule.
        let matches = match_lines("algs = ecdsa-sha2-nistp256");
        assert!(
            matches.iter().all(|m| m.rule_id != "dsa"),
            "dsa should not match ecdsa: {matches:?}"
        );
        // ECDSA itself is still detected as ECC.
        assert!(matches.iter().any(|m| m.rule_id == "ecc"));
    }

    #[test]
    fn dsa_suppressed_on_pqc_signature_lines() {
        // ML-DSA / SLH-DSA names must not be mislabeled as finite-field DSA.
        let m1 = match_lines("const SIG = \"SLH-DSA-SHA2-128s\";");
        assert!(m1.iter().all(|m| m.rule_id != "dsa"), "{m1:?}");
        assert!(m1.iter().any(|m| m.rule_id == "slhdsa"));

        let m2 = match_lines("sig = ML-DSA-65");
        assert!(m2.iter().all(|m| m.rule_id != "dsa"), "{m2:?}");
        assert!(m2.iter().any(|m| m.rule_id == "mldsa"));

        // But a genuine finite-field DSA line is still flagged.
        let m3 = match_lines("cert_sig = dsaWithSHA1");
        assert!(m3.iter().any(|m| m.rule_id == "dsa"), "{m3:?}");
    }
