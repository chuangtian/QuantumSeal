/**
 * Type definitions and a validating loader for quantumseal CryptoBOM documents.
 *
 * The viewer is dependency-free: it relies only on the Node.js standard library
 * and the JSON built into the language. This module turns an untyped parsed
 * JSON value into a strongly-typed {@link CryptoBom}, rejecting malformed input
 * with clear errors.
 *
 * Reminder: a CryptoBOM is the output of *static analysis* of cryptographic
 * indicators. It is not a security verdict or a cryptographic implementation.
 */

/** Quantum-risk classification codes emitted by the Rust CLI. */
export type QuantumRisk =
  | "low_resistant"
  | "moderate_grover"
  | "high_shor"
  | "critical_deprecated";

/** Priority band codes. */
export type PriorityBand =
