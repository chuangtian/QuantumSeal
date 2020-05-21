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
