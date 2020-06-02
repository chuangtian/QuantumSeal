//! # quantumseal
//!
//! A **post-quantum migration workbench**. `quantumseal` recursively inventories
//! source and configuration files for textual indicators of cryptographic
//! algorithms and protocols, classifies each by quantum risk, assigns a
//! migration priority, and emits a Cryptographic Bill of Materials (CryptoBOM)
//! as JSON or human-readable text. It can also compare a fresh scan against a
//! previously saved baseline to track migration progress.
//!
//! ## Scope and disclaimer
//!
//! quantumseal performs **static text analysis only**. It does not implement,
//! execute, break, or evaluate any cryptography. A match indicates that a name
//! associated with a cryptographic primitive appears in a file — it is a
//! *starting point for human review*, not a vulnerability finding or a security
//! audit. The Rust implementation uses the **standard library only**.
//!
//! ## Modules
//! - [`json`]: minimal dependency-free JSON reader/writer.
