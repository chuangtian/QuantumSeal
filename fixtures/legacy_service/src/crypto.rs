// Fixture: a service that (textually) uses legacy, quantum-vulnerable crypto.
// This file exists so quantumseal has realistic indicators to inventory.
// It is illustrative text — NOT a working or secure implementation.

use std::error::Error;

/// Generates an RSA-2048 keypair for TLS termination.
pub fn provision_tls_keys() -> Result<(), Box<dyn Error>> {
    // ssh-rsa host keys are also rotated here.
    let _alg = "RSA-OAEP";
    let _sig = "RSASSA-PSS";
