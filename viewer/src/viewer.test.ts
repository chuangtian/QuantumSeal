/**
 * Tests for @quantumseal/viewer using the Node.js built-in test runner
 * (`node --test`). No external test framework — dependency-free.
 */

import assert from "node:assert/strict";
import { test } from "node:test";

import {
  formatHtml,
  formatMarkdown,
  formatTerminal,
  sortComponents,
} from "./formatters";
import {
  InvalidCryptoBomError,
  loadCryptoBom,
  parseCryptoBom,
  type CryptoBom,
} from "./model";

const SAMPLE: CryptoBom = {
  tool: "quantumseal",
  tool_version: "0.1.0",
  schema: "quantumseal-cbom/1",
  root: "fixtures/legacy_service",
  analysis_only: true,
  disclaimer: "Static analysis only.",
  summary: {
    files_scanned: 2,
    entries_visited: 4,
    component_count: 2,
    total_occurrences: 5,
    priority_bands: { critical: 1, high: 1 },
  },
  components: [
    {
      id: "rsa",
      name: "RSA",
      category: "public_key",
      quantum_risk: "high_shor",
      quantum_risk_label: "High (Shor-breakable)",
