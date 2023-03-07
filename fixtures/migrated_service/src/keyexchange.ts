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

