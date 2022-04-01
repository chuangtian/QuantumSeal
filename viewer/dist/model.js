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
