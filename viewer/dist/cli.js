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
