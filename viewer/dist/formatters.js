"use strict";
/**
 * Report formatters for quantumseal CryptoBOM documents.
 *
 * Three output formats, all produced with the standard library only:
 *   - `formatTerminal`  ANSI-colored text for a console.
 *   - `formatMarkdown`  GitHub-flavored Markdown with a summary table.
 *   - `formatHtml`      a self-contained, dependency-free HTML page.
 */
Object.defineProperty(exports, "__esModule", { value: true });
exports.formatTerminal = formatTerminal;
exports.formatMarkdown = formatMarkdown;
exports.formatHtml = formatHtml;
exports.groupByBand = groupByBand;
exports.sortComponents = sortComponents;
const model_1 = require("./model");
const DISCLAIMER = "This report presents static analysis of cryptographic indicators only. " +
    "It is not a cryptographic implementation and not a security audit; every " +
    "finding warrants human review.";
function palette(enabled) {
    if (!enabled) {
        return {
            reset: "",
            bold: "",
            dim: "",
            red: "",
            yellow: "",
            blue: "",
            green: "",
            magenta: "",
        };
    }
    return {
        reset: "\u001b[0m",
        bold: "\u001b[1m",
        dim: "\u001b[2m",
        red: "\u001b[31m",
        yellow: "\u001b[33m",
        blue: "\u001b[34m",
        green: "\u001b[32m",
        magenta: "\u001b[35m",
