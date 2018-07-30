<div align="center">

<img src="docs/assets/banner.svg" alt="QuantumSeal — post-quantum migration observatory: legacy RSA/ECC signals re-plotted as migration paths" width="100%">

# QuantumSeal · migration observatory

**A crypto-inventory instrument for the post-quantum transition.** Point it at a
tree of source and config, and it charts every recognizable cryptographic
name — RSA, ECC, MD5, ML-KEM, TLS — as a scored **CryptoBOM** (Cryptographic
Bill of Materials). A companion viewer renders that dossier as a terminal,
Markdown, or HTML briefing. Diff two scans to watch a migration close.

[![CI](https://github.com/chuangtian/QuantumSeal/actions/workflows/ci.yml/badge.svg)](https://github.com/chuangtian/QuantumSeal/actions/workflows/ci.yml)
![MIT](https://img.shields.io/badge/license-MIT-34d399)
![Rust](https://img.shields.io/badge/rust-stable-22d3ee)
![Viewer](https://img.shields.io/badge/viewer-node%2020-34d399)

<em>Rust CLI (stdlib only, zero deps) · TypeScript viewer (Node stdlib only)</em>

</div>

> [!CAUTION]
> **quantumseal reads text. It never runs cryptography.**
> It does not implement, execute, break, benchmark, or evaluate any cipher,
> and it is **not** a security audit or a post-quantum crypto library. A finding
> means a known cryptographic *name* appears on a line of a file — a coordinate
> for human review, not a verdict on whether that code is safe. Absence of a
> finding is **not** evidence of safety.

---

## Flight plan (contents)

- [What the instrument charts](#what-the-instrument-charts)
- [Signal-to-orbit: how it works](#signal-to-orbit-how-it-works)
- [Getting the instrument running](#getting-the-instrument-running)
- [Console transcripts](#console-transcripts)
  - [Quick scan](#quick-scan)
  - [Baseline diff](#baseline-diff)
  - [Rendering a dossier](#rendering-a-dossier)
- [Risk taxonomy](#risk-taxonomy)
- [Priority scoring, explained](#priority-scoring-explained)
- [The CryptoBOM dossier](#the-cryptobom-dossier)
- [Indicator rules by example](#indicator-rules-by-example)
- [Migration workflow](#migration-workflow)
