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
