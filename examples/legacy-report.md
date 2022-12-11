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

