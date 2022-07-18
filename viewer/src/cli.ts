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
