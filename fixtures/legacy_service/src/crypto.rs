// Fixture: a service that (textually) uses legacy, quantum-vulnerable crypto.
// This file exists so quantumseal has realistic indicators to inventory.
// It is illustrative text — NOT a working or secure implementation.

use std::error::Error;

/// Generates an RSA-2048 keypair for TLS termination.
