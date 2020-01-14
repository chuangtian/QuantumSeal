//! Cryptographic indicator rules.
//!
//! This module encodes a curated catalog of *textual indicators* that suggest
//! the presence of a cryptographic algorithm, protocol, or primitive in source
//! and configuration files.
//!
//! IMPORTANT: quantumseal performs **static text analysis only**. Matching an
//! indicator does not prove a file uses cryptography insecurely, nor does the
//! absence of a match prove a codebase is safe. The tool exists to help teams
//! *inventory and prioritize* migration work toward post-quantum cryptography
//! (PQC); it does not implement, break, or evaluate any cryptography.

/// Quantum risk classification for a cryptographic family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum QuantumRisk {
    /// Believed resistant to known quantum attacks (e.g. PQC KEMs/signatures,
    /// large symmetric keys). Lowest migration urgency.
    LowResistant,
    /// Symmetric/hash primitives whose effective security is reduced by
    /// Grover's algorithm but not broken; usually mitigated by larger sizes.
    ModerateGrover,
    /// Public-key primitives broken by Shor's algorithm (RSA, ECC, DH). These
    /// are the primary migration targets.
    HighShor,
    /// Legacy/deprecated primitives that are already weak classically and are
    /// also quantum-relevant; highest urgency.
    CriticalDeprecated,
}

impl QuantumRisk {
    /// A short stable identifier used in serialized output.
    pub fn code(self) -> &'static str {
        match self {
            QuantumRisk::LowResistant => "low_resistant",
            QuantumRisk::ModerateGrover => "moderate_grover",
            QuantumRisk::HighShor => "high_shor",
            QuantumRisk::CriticalDeprecated => "critical_deprecated",
        }
    }

    /// Human-readable label.
    pub fn label(self) -> &'static str {
        match self {
            QuantumRisk::LowResistant => "Low (quantum-resistant)",
            QuantumRisk::ModerateGrover => "Moderate (Grover-affected)",
            QuantumRisk::HighShor => "High (Shor-breakable)",
            QuantumRisk::CriticalDeprecated => "Critical (deprecated + quantum-relevant)",
        }
    }

    /// Base priority weight contributed by the risk class.
    pub fn base_weight(self) -> u32 {
        match self {
            QuantumRisk::LowResistant => 5,
            QuantumRisk::ModerateGrover => 25,
            QuantumRisk::HighShor => 70,
            QuantumRisk::CriticalDeprecated => 90,
        }
    }
}

/// The broad category of a cryptographic indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    PublicKey,
    KeyExchange,
    Signature,
    Symmetric,
    Hash,
    Protocol,
    PostQuantum,
    RandomOrKeystore,
}

impl Category {
    pub fn code(self) -> &'static str {
        match self {
            Category::PublicKey => "public_key",
            Category::KeyExchange => "key_exchange",
            Category::Signature => "signature",
            Category::Symmetric => "symmetric",
            Category::Hash => "hash",
            Category::Protocol => "protocol",
            Category::PostQuantum => "post_quantum",
            Category::RandomOrKeystore => "random_or_keystore",
        }
    }
}

/// A single detection rule.
///
/// `needles` are lowercase substrings; a rule matches a line when any needle is
/// present (word-ish boundary aware — see [`crate::scanner`]). Keeping needles
/// lowercase lets the scanner do a single case-fold per line.
#[derive(Debug, Clone)]
pub struct Rule {
    /// Stable rule identifier, e.g. `rsa`.
    pub id: &'static str,
    /// Display name, e.g. `RSA`.
    pub name: &'static str,
    pub category: Category,
    pub risk: QuantumRisk,
    /// Lowercase substrings that trigger this rule.
    pub needles: &'static [&'static str],
    /// Lowercase substrings that, if present anywhere on the same line, suppress
    /// a would-be match. Used to disambiguate names that are substrings of
    /// unrelated primitives (e.g. the bare `dsa` family versus `ml-dsa`).
    pub exclude_if_line_contains: &'static [&'static str],
    /// Short guidance shown in reports.
    pub guidance: &'static str,
}

/// The full built-in rule catalog.
///
/// The list is intentionally conservative and well-documented rather than
/// exhaustive; teams can extend it. Each entry maps a recognizable name to a
/// quantum-risk classification grounded in the NIST PQC transition guidance
/// (Shor breaks RSA/ECC/DH; Grover halves symmetric strength).
pub const RULES: &[Rule] = &[
    // ---- Public-key / key-exchange / signatures: Shor-breakable ----
    Rule {
        id: "rsa",
        name: "RSA",
        category: Category::PublicKey,
        risk: QuantumRisk::HighShor,
