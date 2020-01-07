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
