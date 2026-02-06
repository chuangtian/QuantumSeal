// Fixture: a service mid-migration to post-quantum cryptography.
// Illustrative text only — not a working implementation.

/**
 * Hybrid key establishment during the transition period.
 * Combines a classical curve with a standardized PQC KEM.
 */
export function negotiateKeys(): string {
  // Hybrid: X25519 + ML-KEM-768 (FIPS 203).
  const kem = "ML-KEM-768";      // a.k.a. Kyber
  const classical = "x25519";    // still present during hybrid rollout
  return `${classical}+${kem}`;
}

// Signatures migrated to ML-DSA (Dilithium, FIPS 204).
export const SIGNATURE_ALG = "ML-DSA-65";

// Long-term firmware signing uses a conservative hash-based scheme.
export const FIRMWARE_SIG = "SLH-DSA-SHA2-128s"; // SPHINCS+

// Symmetric layer already uses AES-256-GCM (Grover-safe at 256-bit).
export const AEAD = "AES-256-GCM";
