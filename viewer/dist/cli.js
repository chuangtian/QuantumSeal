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
                }
                options.out = value;
                break;
            }
            case "--max-occurrences": {
                const value = argv[++i];
                const n = Number(value);
                if (!Number.isInteger(n) || n < 0) {
                    throw new Error("--max-occurrences requires a non-negative integer");
                }
                options.maxOccurrences = n;
                break;
            }
            default:
                if (arg.startsWith("--")) {
                    throw new Error(`unknown option '${arg}'`);
                }
                if (options.input !== undefined) {
                    throw new Error("only one input file may be given");
                }
                options.input = arg;
        }
    }
    return options;
}
function readInput(input) {
    if (input !== undefined) {
        return (0, node_fs_1.readFileSync)(input, "utf8");
    }
    // Read from stdin.
    return (0, node_fs_1.readFileSync)(0, "utf8");
}
function main(argv) {
    let options;
    try {
        options = parseCliArgs(argv);
    }
    catch (err) {
        process.stderr.write(`error: ${err.message}\n\n${USAGE}`);
        return 2;
    }
    if (options.help) {
        process.stdout.write(USAGE);
        return 0;
    }
    let text;
    try {
        text = readInput(options.input);
    }
    catch (err) {
        process.stderr.write(`error: cannot read input: ${err.message}\n`);
        return 2;
    }
    let rendered;
    try {
        const bom = (0, model_1.parseCryptoBom)(text);
        switch (options.format) {
            case "terminal":
                rendered = (0, formatters_1.formatTerminal)(bom, {
                    color: options.color,
                    maxOccurrences: options.maxOccurrences,
                });
                break;
            case "markdown":
                rendered = (0, formatters_1.formatMarkdown)(bom);
                break;
            case "html":
                rendered = (0, formatters_1.formatHtml)(bom);
                break;
        }
    }
    catch (err) {
        if (err instanceof model_1.InvalidCryptoBomError) {
            process.stderr.write(`error: ${err.message}\n`);
            return 1;
        }
        throw err;
    }
    if (options.out !== undefined) {
        (0, node_fs_1.writeFileSync)(options.out, rendered, "utf8");
        process.stderr.write(`wrote report to ${options.out}\n`);
    }
    else {
        process.stdout.write(rendered);
    }
    return 0;
}
process.exit(main(process.argv.slice(2)));
