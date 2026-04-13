#!/usr/bin/env node
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

import { readFileSync, writeFileSync } from "node:fs";

import {
  formatHtml,
  formatMarkdown,
  formatTerminal,
} from "./formatters";
import { InvalidCryptoBomError, parseCryptoBom } from "./model";

type Format = "terminal" | "markdown" | "html";

interface CliOptions {
  input?: string;
  format: Format;
  out?: string;
  color: boolean;
  maxOccurrences: number;
  help: boolean;
}

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

function parseCliArgs(argv: readonly string[]): CliOptions {
  const options: CliOptions = {
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

function readInput(input: string | undefined): string {
  if (input !== undefined) {
    return readFileSync(input, "utf8");
  }
  // Read from stdin.
  return readFileSync(0, "utf8");
}

function main(argv: readonly string[]): number {
  let options: CliOptions;
  try {
    options = parseCliArgs(argv);
  } catch (err) {
    process.stderr.write(`error: ${(err as Error).message}\n\n${USAGE}`);
    return 2;
  }

  if (options.help) {
    process.stdout.write(USAGE);
    return 0;
  }

  let text: string;
  try {
    text = readInput(options.input);
  } catch (err) {
    process.stderr.write(`error: cannot read input: ${(err as Error).message}\n`);
    return 2;
  }

  let rendered: string;
  try {
    const bom = parseCryptoBom(text);
    switch (options.format) {
      case "terminal":
        rendered = formatTerminal(bom, {
          color: options.color,
          maxOccurrences: options.maxOccurrences,
        });
        break;
      case "markdown":
        rendered = formatMarkdown(bom);
        break;
      case "html":
        rendered = formatHtml(bom);
        break;
    }
  } catch (err) {
    if (err instanceof InvalidCryptoBomError) {
      process.stderr.write(`error: ${err.message}\n`);
      return 1;
    }
    throw err;
  }

  if (options.out !== undefined) {
    writeFileSync(options.out, rendered, "utf8");
    process.stderr.write(`wrote report to ${options.out}\n`);
  } else {
    process.stdout.write(rendered);
  }
  return 0;
}

process.exit(main(process.argv.slice(2)));
