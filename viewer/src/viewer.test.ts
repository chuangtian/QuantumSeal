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
      priority: 80,
      priority_band: "high",
      file_count: 2,
      occurrence_count: 3,
      guidance: "Migrate RSA to PQC.",
      occurrences: [
        { file: "a.rs", line: 1, excerpt: "rsa", needle: "rsa" },
        { file: "b.rs", line: 2, excerpt: "rsa-oaep", needle: "rsa-oaep" },
        { file: "c.rs", line: 3, excerpt: "rsa-pss", needle: "rsa-pss" },
      ],
    },
    {
      id: "md5",
      name: "MD5",
      category: "hash",
      quantum_risk: "critical_deprecated",
      quantum_risk_label: "Critical",
      priority: 97,
      priority_band: "critical",
      file_count: 1,
      occurrence_count: 2,
      guidance: "Replace MD5.",
      occurrences: [
        { file: "a.rs", line: 4, excerpt: "md5", needle: "md5" },
        { file: "a.rs", line: 5, excerpt: "md5", needle: "md5" },
      ],
    },
  ],
};

function serialize(bom: CryptoBom): string {
  return JSON.stringify(bom);
}

test("parseCryptoBom round-trips a valid document", () => {
  const bom = parseCryptoBom(serialize(SAMPLE));
  assert.equal(bom.tool, "quantumseal");
  assert.equal(bom.components.length, 2);
  assert.equal(bom.analysis_only, true);
});

