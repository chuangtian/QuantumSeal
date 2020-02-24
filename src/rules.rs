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
        needles: &["rsa", "rsassa", "rsa-oaep", "rsa-pss", "pkcs1"],
        exclude_if_line_contains: &[],
        guidance: "Replace RSA key establishment/signatures with NIST PQC (ML-KEM for KEM, ML-DSA/SLH-DSA for signatures) or hybrid modes.",
    },
    Rule {
        id: "ecc",
        name: "Elliptic Curve (ECDSA/ECDH/EdDSA)",
        category: Category::PublicKey,
        risk: QuantumRisk::HighShor,
        needles: &[
            "ecdsa", "ecdh", "eddsa", "ed25519", "ed448", "x25519", "x448",
            "secp256", "secp384", "secp521", "prime256v1", "nistp256", "curve25519",
        ],
        exclude_if_line_contains: &[],
        guidance: "Elliptic-curve schemes are Shor-breakable. Migrate to ML-KEM/ML-DSA; consider hybrid X25519+ML-KEM during transition.",
    },
    Rule {
        id: "dh",
        name: "Diffie-Hellman (finite field)",
        category: Category::KeyExchange,
        risk: QuantumRisk::HighShor,
        needles: &["diffie-hellman", "diffiehellman", "dhparam", "modp", "ffdhe"],
        exclude_if_line_contains: &[],
        guidance: "Finite-field DH is Shor-breakable. Move to ML-KEM or hybrid key establishment.",
    },
    Rule {
        id: "dsa",
        name: "DSA",
        category: Category::Signature,
        risk: QuantumRisk::HighShor,
        needles: &["dsa-sha", "dsawithsha", "dss", "id-dsa", "dsaencryption"],
        // The bare DSA family is a substring of the PQC schemes ML-DSA and
        // SLH-DSA and of ECDSA. Suppress those to avoid mislabeling migrated or
        // elliptic-curve code as finite-field DSA.
        exclude_if_line_contains: &["ml-dsa", "mldsa", "slh-dsa", "slhdsa", "ecdsa", "eddsa"],
        guidance: "DSA is Shor-breakable and largely deprecated. Migrate signatures to ML-DSA or SLH-DSA.",
    },
    // ---- Symmetric ciphers: Grover-affected ----
    Rule {
        id: "aes",
        name: "AES",
        category: Category::Symmetric,
        risk: QuantumRisk::ModerateGrover,
        needles: &["aes-128", "aes128", "aes-192", "aes192", "aes-256", "aes256", "aesgcm", "aes-gcm", "aes-cbc"],
        exclude_if_line_contains: &[],
        guidance: "AES remains viable; ensure >=256-bit keys so Grover's algorithm leaves ~128-bit effective strength.",
    },
    Rule {
        id: "chacha20",
        name: "ChaCha20 / Poly1305",
        category: Category::Symmetric,
        risk: QuantumRisk::ModerateGrover,
        needles: &["chacha20", "chacha", "poly1305", "xchacha20"],
        exclude_if_line_contains: &[],
        guidance: "ChaCha20-Poly1305 (256-bit) is a solid symmetric choice; no PQC replacement needed, retain large keys.",
    },
    Rule {
        id: "3des",
        name: "3DES / DES",
        category: Category::Symmetric,
        risk: QuantumRisk::CriticalDeprecated,
        needles: &["3des", "tripledes", "triple-des", "des-ede", "des-cbc", "desede"],
        exclude_if_line_contains: &[],
        guidance: "DES/3DES are deprecated and weak. Replace immediately with AES-256; this is both a classical and quantum concern.",
    },
    Rule {
        id: "rc4",
        name: "RC4",
        category: Category::Symmetric,
        risk: QuantumRisk::CriticalDeprecated,
        needles: &["rc4", "arcfour"],
        exclude_if_line_contains: &[],
        guidance: "RC4 is broken classically. Remove entirely and replace with an AEAD cipher (AES-GCM or ChaCha20-Poly1305).",
    },
    Rule {
        id: "blowfish",
        name: "Blowfish",
        category: Category::Symmetric,
        risk: QuantumRisk::ModerateGrover,
        needles: &["blowfish", "bf-cbc", "bf-ecb"],
        exclude_if_line_contains: &[],
        guidance: "Blowfish uses a 64-bit block and is dated. Prefer AES-256-GCM.",
    },
    // ---- Hash functions ----
    Rule {
        id: "md5",
        name: "MD5",
        category: Category::Hash,
        risk: QuantumRisk::CriticalDeprecated,
        needles: &["md5"],
        exclude_if_line_contains: &[],
        guidance: "MD5 is collision-broken. Replace with SHA-256/SHA-3; never use for signatures or integrity.",
    },
    Rule {
        id: "sha1",
        name: "SHA-1",
        category: Category::Hash,
        risk: QuantumRisk::CriticalDeprecated,
        needles: &["sha1", "sha-1"],
        exclude_if_line_contains: &[],
        guidance: "SHA-1 is collision-broken. Migrate to SHA-256 or SHA-3.",
    },
    Rule {
        id: "sha2",
        name: "SHA-2 family",
        category: Category::Hash,
        risk: QuantumRisk::ModerateGrover,
        needles: &["sha256", "sha-256", "sha384", "sha-384", "sha512", "sha-512", "sha224"],
        exclude_if_line_contains: &[],
        guidance: "SHA-2 is fine; use >=SHA-384 where Grover margin matters for long-lived integrity.",
    },
    Rule {
        id: "sha3",
        name: "SHA-3 / SHAKE",
        category: Category::Hash,
        risk: QuantumRisk::ModerateGrover,
        needles: &["sha3-", "sha-3", "keccak", "shake128", "shake256"],
        exclude_if_line_contains: &[],
        guidance: "SHA-3/SHAKE are modern and quantum-appropriate at large sizes.",
    },
    // ---- Protocols ----
    Rule {
        id: "tls",
        name: "TLS / SSL",
        category: Category::Protocol,
        risk: QuantumRisk::HighShor,
        needles: &["tlsv1", "tls1", "sslv3", "ssl_ctx", "ssl_context", "starttls", "tls_"],
        exclude_if_line_contains: &[],
        guidance: "TLS key exchange relies on ECDHE/RSA today. Adopt TLS 1.3 and enable hybrid PQC key exchange when your stack supports it.",
    },
    Rule {
        id: "ssh",
        name: "SSH",
        category: Category::Protocol,
        risk: QuantumRisk::HighShor,
        needles: &["ssh-rsa", "ssh-ed25519", "ecdsa-sha2", "openssh"],
        exclude_if_line_contains: &[],
        guidance: "SSH host/user keys use RSA/ECDSA/Ed25519. Track OpenSSH PQC hybrid KEX (e.g. sntrup761x25519) adoption.",
    },
    Rule {
        id: "jwt",
        name: "JWT / JOSE",
        category: Category::Protocol,
        risk: QuantumRisk::HighShor,
        needles: &["jwt", "jws", "jwe", "rs256", "es256", "ps256", "hs256"],
        exclude_if_line_contains: &[],
        guidance: "Asymmetric JWT algs (RS256/ES256/PS256) are Shor-breakable. Plan for PQC-capable token signing; HS256 is symmetric.",
    },
    // ---- Post-quantum (already migrating) ----
    Rule {
        id: "mlkem",
        name: "ML-KEM (Kyber)",
        category: Category::PostQuantum,
        risk: QuantumRisk::LowResistant,
        needles: &["ml-kem", "mlkem", "kyber", "crystals-kyber"],
        exclude_if_line_contains: &[],
        guidance: "ML-KEM (FIPS 203) is a standardized PQ KEM. Good target; verify parameter set (512/768/1024).",
    },
    Rule {
        id: "mldsa",
        name: "ML-DSA (Dilithium)",
        category: Category::PostQuantum,
        risk: QuantumRisk::LowResistant,
        needles: &["ml-dsa", "mldsa", "dilithium", "crystals-dilithium"],
        exclude_if_line_contains: &[],
        guidance: "ML-DSA (FIPS 204) is a standardized PQ signature scheme. Good target.",
    },
    Rule {
        id: "slhdsa",
        name: "SLH-DSA (SPHINCS+)",
        category: Category::PostQuantum,
        risk: QuantumRisk::LowResistant,
        needles: &["slh-dsa", "slhdsa", "sphincs", "sphincs+"],
        exclude_if_line_contains: &[],
        guidance: "SLH-DSA (FIPS 205) is a stateless hash-based PQ signature scheme. Good conservative target.",
    },
    // ---- Randomness / keystores ----
    Rule {
        id: "keystore",
        name: "Key material / keystore",
        category: Category::RandomOrKeystore,
        risk: QuantumRisk::HighShor,
        needles: &["private key", "begin rsa private", "begin ec private", "pkcs12", "keystore", "-----begin"],
        exclude_if_line_contains: &[],
        guidance: "Stored asymmetric key material may need reissuing with PQC algorithms; inventory and rotate as part of migration.",
    },
    Rule {
        id: "weak_random",
        name: "Non-cryptographic RNG",
        category: Category::RandomOrKeystore,
        risk: QuantumRisk::ModerateGrover,
        needles: &["math.random", "mersenne", "rand()", "srand("],
        exclude_if_line_contains: &[],
