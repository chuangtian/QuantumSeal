# quantumseal CryptoBOM — `fixtures/legacy_service`

> This report presents static analysis of cryptographic indicators only. It is not a cryptographic implementation and not a security audit; every finding warrants human review.

**Tool:** quantumseal v0.1.0 · **Files scanned:** 2 · **Components:** 13 · **Occurrences:** 30

## Priority summary

| Band | Components |
| --- | ---: |
| critical | 4 |
| high | 6 |
| low | 3 |

## Components

| Priority | Band | Component | Category | Quantum risk | Files | Occurrences |
| ---: | --- | --- | --- | --- | ---: | ---: |
| 97 | critical | MD5 | hash | critical_deprecated | 2 | 3 |
| 93 | critical | SHA-1 | hash | critical_deprecated | 1 | 2 |
| 90 | critical | 3DES / DES | symmetric | critical_deprecated | 1 | 1 |
| 90 | critical | RC4 | symmetric | critical_deprecated | 1 | 1 |
| 80 | high | RSA | public_key | high_shor | 2 | 6 |
| 77 | high | Elliptic Curve (ECDSA/ECDH/EdDSA) | public_key | high_shor | 2 | 4 |
| 77 | high | SSH | protocol | high_shor | 2 | 2 |
| 77 | high | TLS / SSL | protocol | high_shor | 2 | 2 |
| 73 | high | JWT / JOSE | protocol | high_shor | 1 | 2 |
| 73 | high | Key material / keystore | random_or_keystore | high_shor | 1 | 2 |
| 28 | low | AES | symmetric | moderate_grover | 1 | 2 |
| 28 | low | SHA-2 family | hash | moderate_grover | 1 | 2 |
| 25 | low | ChaCha20 / Poly1305 | symmetric | moderate_grover | 1 | 1 |

## Details

### MD5 (priority 97, critical)

- **Quantum risk:** Critical (deprecated + quantum-relevant)
- **Guidance:** MD5 is collision-broken. Replace with SHA-256/SHA-3; never use for signatures or integrity.

  - `fixtures/legacy_service/config/tls.toml:17` — `legacy_checksum = "md5"`
  - `fixtures/legacy_service/src/crypto.rs:25` — `// MD5 and SHA-1 are collision-broken.`
  - `fixtures/legacy_service/src/crypto.rs:26` — `let _md5 = "md5";`

### SHA-1 (priority 93, critical)

- **Quantum risk:** Critical (deprecated + quantum-relevant)
- **Guidance:** SHA-1 is collision-broken. Migrate to SHA-256 or SHA-3.

  - `fixtures/legacy_service/src/crypto.rs:25` — `// MD5 and SHA-1 are collision-broken.`
  - `fixtures/legacy_service/src/crypto.rs:27` — `let _sha1 = "sha1";`

### 3DES / DES (priority 90, critical)

- **Quantum risk:** Critical (deprecated + quantum-relevant)
- **Guidance:** DES/3DES are deprecated and weak. Replace immediately with AES-256; this is both a classical and quantum concern.

  - `fixtures/legacy_service/src/crypto.rs:28` — `let _cipher = "3DES-CBC"; // Triple-DES, deprecated`

### RC4 (priority 90, critical)

- **Quantum risk:** Critical (deprecated + quantum-relevant)
- **Guidance:** RC4 is broken classically. Remove entirely and replace with an AEAD cipher (AES-GCM or ChaCha20-Poly1305).

  - `fixtures/legacy_service/src/crypto.rs:29` — `let _rc4 = "RC4";         // broken stream cipher`

### RSA (priority 80, high)

- **Quantum risk:** High (Shor-breakable)
- **Guidance:** Replace RSA key establishment/signatures with NIST PQC (ML-KEM for KEM, ML-DSA/SLH-DSA for signatures) or hybrid modes.

  - `fixtures/legacy_service/config/tls.toml:7` — `"ECDHE-RSA-AES256-GCM-SHA384",`
  - `fixtures/legacy_service/config/tls.toml:13` — `host_key_algorithms = "ssh-rsa,ecdsa-sha2-nistp256,ssh-ed25519"`
  - `fixtures/legacy_service/src/crypto.rs:7` — `/// Generates an RSA-2048 keypair for TLS termination.`
  - `fixtures/legacy_service/src/crypto.rs:9` — `// ssh-rsa host keys are also rotated here.`
  - `fixtures/legacy_service/src/crypto.rs:10` — `let _alg = "RSA-OAEP";`
  - `fixtures/legacy_service/src/crypto.rs:11` — `let _sig = "RSASSA-PSS";`

### Elliptic Curve (ECDSA/ECDH/EdDSA) (priority 77, high)

- **Quantum risk:** High (Shor-breakable)
- **Guidance:** Elliptic-curve schemes are Shor-breakable. Migrate to ML-KEM/ML-DSA; consider hybrid X25519+ML-KEM during transition.

  - `fixtures/legacy_service/config/tls.toml:8` — `"ECDHE-ECDSA-CHACHA20-POLY1305",`
  - `fixtures/legacy_service/config/tls.toml:13` — `host_key_algorithms = "ssh-rsa,ecdsa-sha2-nistp256,ssh-ed25519"`
  - `fixtures/legacy_service/src/crypto.rs:12` — `let _curve = "secp256r1"; // ECDSA on the NIST P-256 curve`
  - `fixtures/legacy_service/src/crypto.rs:13` — `let _kex = "ECDH with X25519";`

### SSH (priority 77, high)

- **Quantum risk:** High (Shor-breakable)
- **Guidance:** SSH host/user keys use RSA/ECDSA/Ed25519. Track OpenSSH PQC hybrid KEX (e.g. sntrup761x25519) adoption.

  - `fixtures/legacy_service/config/tls.toml:13` — `host_key_algorithms = "ssh-rsa,ecdsa-sha2-nistp256,ssh-ed25519"`
  - `fixtures/legacy_service/src/crypto.rs:9` — `// ssh-rsa host keys are also rotated here.`

### TLS / SSL (priority 77, high)

- **Quantum risk:** High (Shor-breakable)
- **Guidance:** TLS key exchange relies on ECDHE/RSA today. Adopt TLS 1.3 and enable hybrid PQC key exchange when your stack supports it.

  - `fixtures/legacy_service/config/tls.toml:5` — `min_version = "TLSv1.2"`
  - `fixtures/legacy_service/src/crypto.rs:8` — `pub fn provision_tls_keys() -> Result<(), Box<dyn Error>> {`

### JWT / JOSE (priority 73, high)

- **Quantum risk:** High (Shor-breakable)
- **Guidance:** Asymmetric JWT algs (RS256/ES256/PS256) are Shor-breakable. Plan for PQC-capable token signing; HS256 is symmetric.

  - `fixtures/legacy_service/src/crypto.rs:17` — `/// Legacy token signing still uses RS256 (asymmetric JWT).`
