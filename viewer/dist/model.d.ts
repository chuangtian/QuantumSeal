/**
 * Type definitions and a validating loader for quantumseal CryptoBOM documents.
 *
 * The viewer is dependency-free: it relies only on the Node.js standard library
 * and the JSON built into the language. This module turns an untyped parsed
 * JSON value into a strongly-typed {@link CryptoBom}, rejecting malformed input
