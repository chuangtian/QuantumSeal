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
export type QuantumRisk = "low_resistant" | "moderate_grover" | "high_shor" | "critical_deprecated";
/** Priority band codes. */
export type PriorityBand = "informational" | "low" | "medium" | "high" | "critical";
/** A single location where an indicator was found. */
export interface Occurrence {
    readonly file: string;
    readonly line: number;
    readonly excerpt: string;
    readonly needle: string;
}
/** An aggregated cryptographic component. */
export interface Component {
    readonly id: string;
    readonly name: string;
    readonly category: string;
    readonly quantum_risk: QuantumRisk;
    readonly quantum_risk_label: string;
    readonly priority: number;
    readonly priority_band: PriorityBand;
    readonly file_count: number;
    readonly occurrence_count: number;
    readonly guidance: string;
    readonly occurrences: readonly Occurrence[];
}
/** Summary block. */
export interface Summary {
    readonly files_scanned: number;
    readonly entries_visited: number;
    readonly component_count: number;
    readonly total_occurrences: number;
    readonly priority_bands: Readonly<Record<string, number>>;
}
/** The full CryptoBOM document. */
export interface CryptoBom {
    readonly tool: string;
    readonly tool_version: string;
    readonly schema: string;
    readonly root: string;
    readonly analysis_only: boolean;
    readonly disclaimer: string;
    readonly summary: Summary;
    readonly components: readonly Component[];
}
/** Ordered list of bands from most to least urgent, for sorting/coloring. */
export declare const BAND_ORDER: readonly PriorityBand[];
/** Thrown when a document does not conform to the CryptoBOM schema. */
export declare class InvalidCryptoBomError extends Error {
    constructor(message: string);
}
/**
 * Validate and type an already-parsed JSON value as a {@link CryptoBom}.
 * @throws {InvalidCryptoBomError} if the shape is wrong.
 */
export declare function loadCryptoBom(value: unknown): CryptoBom;
/** Parse CryptoBOM JSON text into a typed document. */
export declare function parseCryptoBom(text: string): CryptoBom;
