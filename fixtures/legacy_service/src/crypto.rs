// Fixture: a service that (textually) uses legacy, quantum-vulnerable crypto.
// This file exists so quantumseal has realistic indicators to inventory.
// It is illustrative text — NOT a working or secure implementation.

use std::error::Error;

/// Generates an RSA-2048 keypair for TLS termination.
pub fn provision_tls_keys() -> Result<(), Box<dyn Error>> {
    // ssh-rsa host keys are also rotated here.
    let _alg = "RSA-OAEP";
    let _sig = "RSASSA-PSS";
    let _curve = "secp256r1"; // ECDSA on the NIST P-256 curve
    let _kex = "ECDH with X25519";
    Ok(())
}

/// Legacy token signing still uses RS256 (asymmetric JWT).
pub fn sign_token() -> String {
    // JWT alg header: RS256
    "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9".to_string()
}

/// Deprecated primitives that should be removed entirely.
pub fn legacy_digest(input: &[u8]) -> Vec<u8> {
