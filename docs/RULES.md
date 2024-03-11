# quantumseal indicator rules & scoring

This document describes **what quantumseal looks for** and **how it prioritizes**
findings. It is reference material for reviewers who want to understand — or
extend — the built-in catalog.

> **Scope reminder.** quantumseal performs *static text analysis only*. A rule
> match means a recognizable cryptographic name appears in a file; it is **not**
> proof of insecure usage, and the absence of a match is **not** proof of
> safety. Treat every finding as a starting point for human review, not a
> security verdict. quantumseal does not implement, execute, break, or evaluate
> any cryptography.

## Quantum-risk model

Each rule is assigned one of four quantum-risk classes, grounded in the widely
understood impact of quantum algorithms on cryptography:

| Class | Code | Meaning | Base weight |
| --- | --- | --- | ---: |
| Low (quantum-resistant) | `low_resistant` | Post-quantum schemes and large symmetric keys believed safe against known quantum attacks. | 5 |
| Moderate (Grover-affected) | `moderate_grover` | Symmetric ciphers / hashes whose effective strength is halved by Grover's algorithm but not broken; usually mitigated by larger sizes. | 25 |
| High (Shor-breakable) | `high_shor` | Public-key primitives (RSA, ECC, finite-field DH, DSA) broken by Shor's algorithm — the primary migration targets. | 70 |
| Critical (deprecated + quantum-relevant) | `critical_deprecated` | Primitives already weak classically (MD5, SHA-1, DES/3DES, RC4). Highest urgency. | 90 |

## Priority scoring

Each detected component receives a **0–100 priority score**:

```
score = min(100, base_weight(risk) + occurrence_bonus + file_bonus)
```

- `base_weight(risk)` — dominant term from the table above.
- `occurrence_bonus` — saturating bonus for how many times the indicator
  appears (0 → 0, 2–4 → 3, 5–9 → 6, 10–24 → 9, 25+ → 12).
- `file_bonus` — saturating bonus for how many distinct files contain it
  (1 → 0, 2–4 → 4, 5–9 → 8, 10+ → 12).

Scores map to bands:

| Band | Score range |
| --- | --- |
| `informational` | 0–9 |
| `low` | 10–34 |
| `medium` | 35–59 |
| `high` | 60–84 |
| `critical` | 85–100 |

Risk always dominates prevalence: a single isolated Shor-breakable primitive
still outranks a heavily-used quantum-resistant one.

## Matching semantics

- Each source line is lowercased once, then tested against every rule's
  **needles** (lowercase substrings).
- **Class-aware boundaries** reduce false positives: a needle is rejected as an
  embedded fragment when the adjacent character shares its boundary class —
  letter-next-to-letter or digit-next-to-digit. This is why `sha3-` does not
  fire on `sha384`, while `Dilithium3` and `generate_rsa_2048` still match.
- **Line-level exclusions** (`exclude_if_line_contains`) disambiguate names that
  are substrings of unrelated primitives. For example, the bare `dsa` family is
  suppressed on any line containing `ml-dsa`, `slh-dsa`, or `ecdsa`.
- Only **text-like files** are scanned (by extension); binaries and files
  containing NUL bytes are skipped, as are common vendor directories
  (`node_modules`, `target`, `.git`, `dist`, `build`, …).

## Built-in catalog

| Rule id | Name | Category | Risk |
| --- | --- | --- | --- |
| `rsa` | RSA | public_key | high_shor |
| `ecc` | Elliptic Curve (ECDSA/ECDH/EdDSA) | public_key | high_shor |
| `dh` | Diffie-Hellman (finite field) | key_exchange | high_shor |
| `dsa` | DSA | signature | high_shor |
| `aes` | AES | symmetric | moderate_grover |
| `chacha20` | ChaCha20 / Poly1305 | symmetric | moderate_grover |
| `3des` | 3DES / DES | symmetric | critical_deprecated |
| `rc4` | RC4 | symmetric | critical_deprecated |
