#!/usr/bin/env node
"use strict";
/**
 * quantumseal-view — CLI for rendering a CryptoBOM.
 *
 * Usage:
 *   quantumseal-view <cbom.json> [--format terminal|markdown|html] [--out FILE]
 *                                [--no-color] [--max-occurrences N]
 *   quantumseal-view --help
 *
 * Reads a quantumseal CryptoBOM (JSON) and prints a formatted report. When no
 * file is given, reads JSON from stdin. Dependency-free: Node standard library
 * only.
 */
Object.defineProperty(exports, "__esModule", { value: true });
const node_fs_1 = require("node:fs");
const formatters_1 = require("./formatters");
const model_1 = require("./model");
const USAGE = `quantumseal-view — render a quantumseal CryptoBOM (static analysis only)

USAGE:
  quantumseal-view <cbom.json> [OPTIONS]
  cat cbom.json | quantumseal-view [OPTIONS]

OPTIONS:
  --format <terminal|markdown|html>   Output format (default: terminal).
  --out <FILE>                        Write to FILE instead of stdout.
  --no-color                          Disable ANSI color (terminal format).
  --max-occurrences <N>               Limit occurrences per component (0 = all; default 5).
  -h, --help                          Show this help.

EXAMPLES:
  quantumseal-view cbom.json --format markdown --out report.md
  quantumseal-view cbom.json --format html --out report.html
  cat cbom.json | quantumseal-view --no-color
`;
function parseCliArgs(argv) {
    const options = {
        format: "terminal",
        color: true,
        maxOccurrences: 5,
        help: false,
    };
    for (let i = 0; i < argv.length; i++) {
        const arg = argv[i];
        switch (arg) {
            case "-h":
            case "--help":
                options.help = true;
                break;
            case "--no-color":
                options.color = false;
                break;
            case "--format": {
                const value = argv[++i];
                if (value !== "terminal" && value !== "markdown" && value !== "html") {
                    throw new Error(`invalid --format '${value ?? ""}' (terminal|markdown|html)`);
                }
                options.format = value;
                break;
            }
            case "--out": {
                const value = argv[++i];
                if (value === undefined) {
                    throw new Error("--out requires a file path");
