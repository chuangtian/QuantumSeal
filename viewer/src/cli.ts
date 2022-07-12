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
