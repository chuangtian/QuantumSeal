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
