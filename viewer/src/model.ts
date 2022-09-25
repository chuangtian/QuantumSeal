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
  | "informational"
  | "low"
  | "medium"
  | "high"
  | "critical";

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
export const BAND_ORDER: readonly PriorityBand[] = [
  "critical",
  "high",
  "medium",
  "low",
  "informational",
];

/** Thrown when a document does not conform to the CryptoBOM schema. */
export class InvalidCryptoBomError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "InvalidCryptoBomError";
  }
}

function isObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function requireString(obj: Record<string, unknown>, key: string): string {
  const v = obj[key];
  if (typeof v !== "string") {
    throw new InvalidCryptoBomError(`expected string field '${key}'`);
  }
  return v;
}

function requireNumber(obj: Record<string, unknown>, key: string): number {
  const v = obj[key];
  if (typeof v !== "number" || !Number.isFinite(v)) {
    throw new InvalidCryptoBomError(`expected numeric field '${key}'`);
  }
  return v;
}

function requireBool(obj: Record<string, unknown>, key: string): boolean {
  const v = obj[key];
  if (typeof v !== "boolean") {
    throw new InvalidCryptoBomError(`expected boolean field '${key}'`);
  }
  return v;
}

function requireArray(obj: Record<string, unknown>, key: string): unknown[] {
  const v = obj[key];
  if (!Array.isArray(v)) {
    throw new InvalidCryptoBomError(`expected array field '${key}'`);
  }
  return v;
}

function parseOccurrence(value: unknown): Occurrence {
  if (!isObject(value)) {
    throw new InvalidCryptoBomError("occurrence must be an object");
  }
  return {
    file: requireString(value, "file"),
    line: requireNumber(value, "line"),
    excerpt: requireString(value, "excerpt"),
    needle: requireString(value, "needle"),
  };
}

function parseComponent(value: unknown): Component {
  if (!isObject(value)) {
    throw new InvalidCryptoBomError("component must be an object");
  }
  const occurrences = requireArray(value, "occurrences").map(parseOccurrence);
  return {
    id: requireString(value, "id"),
    name: requireString(value, "name"),
    category: requireString(value, "category"),
    quantum_risk: requireString(value, "quantum_risk") as QuantumRisk,
    quantum_risk_label: requireString(value, "quantum_risk_label"),
    priority: requireNumber(value, "priority"),
    priority_band: requireString(value, "priority_band") as PriorityBand,
    file_count: requireNumber(value, "file_count"),
    occurrence_count: requireNumber(value, "occurrence_count"),
    guidance: requireString(value, "guidance"),
    occurrences,
  };
}

function parseSummary(value: unknown): Summary {
  if (!isObject(value)) {
    throw new InvalidCryptoBomError("summary must be an object");
  }
  const bandsRaw = value["priority_bands"];
  const bands: Record<string, number> = {};
  if (isObject(bandsRaw)) {
    for (const [k, v] of Object.entries(bandsRaw)) {
      if (typeof v === "number") {
        bands[k] = v;
      }
    }
  }
  return {
    files_scanned: requireNumber(value, "files_scanned"),
    entries_visited: requireNumber(value, "entries_visited"),
    component_count: requireNumber(value, "component_count"),
    total_occurrences: requireNumber(value, "total_occurrences"),
    priority_bands: bands,
  };
}

/**
 * Validate and type an already-parsed JSON value as a {@link CryptoBom}.
 * @throws {InvalidCryptoBomError} if the shape is wrong.
 */
export function loadCryptoBom(value: unknown): CryptoBom {
