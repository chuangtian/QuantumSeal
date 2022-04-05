"use strict";
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
Object.defineProperty(exports, "__esModule", { value: true });
exports.InvalidCryptoBomError = exports.BAND_ORDER = void 0;
exports.loadCryptoBom = loadCryptoBom;
exports.parseCryptoBom = parseCryptoBom;
/** Ordered list of bands from most to least urgent, for sorting/coloring. */
exports.BAND_ORDER = [
    "critical",
    "high",
    "medium",
    "low",
    "informational",
];
/** Thrown when a document does not conform to the CryptoBOM schema. */
class InvalidCryptoBomError extends Error {
    constructor(message) {
        super(message);
        this.name = "InvalidCryptoBomError";
    }
}
exports.InvalidCryptoBomError = InvalidCryptoBomError;
function isObject(value) {
    return typeof value === "object" && value !== null && !Array.isArray(value);
}
function requireString(obj, key) {
    const v = obj[key];
    if (typeof v !== "string") {
        throw new InvalidCryptoBomError(`expected string field '${key}'`);
    }
    return v;
}
function requireNumber(obj, key) {
    const v = obj[key];
    if (typeof v !== "number" || !Number.isFinite(v)) {
        throw new InvalidCryptoBomError(`expected numeric field '${key}'`);
    }
    return v;
}
function requireBool(obj, key) {
    const v = obj[key];
    if (typeof v !== "boolean") {
        throw new InvalidCryptoBomError(`expected boolean field '${key}'`);
    }
    return v;
}
function requireArray(obj, key) {
    const v = obj[key];
    if (!Array.isArray(v)) {
        throw new InvalidCryptoBomError(`expected array field '${key}'`);
    }
    return v;
}
function parseOccurrence(value) {
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
function parseComponent(value) {
    if (!isObject(value)) {
        throw new InvalidCryptoBomError("component must be an object");
    }
    const occurrences = requireArray(value, "occurrences").map(parseOccurrence);
    return {
        id: requireString(value, "id"),
        name: requireString(value, "name"),
        category: requireString(value, "category"),
        quantum_risk: requireString(value, "quantum_risk"),
        quantum_risk_label: requireString(value, "quantum_risk_label"),
        priority: requireNumber(value, "priority"),
        priority_band: requireString(value, "priority_band"),
        file_count: requireNumber(value, "file_count"),
        occurrence_count: requireNumber(value, "occurrence_count"),
        guidance: requireString(value, "guidance"),
        occurrences,
    };
}
function parseSummary(value) {
    if (!isObject(value)) {
        throw new InvalidCryptoBomError("summary must be an object");
    }
    const bandsRaw = value["priority_bands"];
    const bands = {};
    if (isObject(bandsRaw)) {
