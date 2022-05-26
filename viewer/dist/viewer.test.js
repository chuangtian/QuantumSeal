"use strict";
/**
 * Tests for @quantumseal/viewer using the Node.js built-in test runner
 * (`node --test`). No external test framework — dependency-free.
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const strict_1 = __importDefault(require("node:assert/strict"));
const node_test_1 = require("node:test");
const formatters_1 = require("./formatters");
const model_1 = require("./model");
const SAMPLE = {
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
function serialize(bom) {
    return JSON.stringify(bom);
}
(0, node_test_1.test)("parseCryptoBom round-trips a valid document", () => {
    const bom = (0, model_1.parseCryptoBom)(serialize(SAMPLE));
    strict_1.default.equal(bom.tool, "quantumseal");
    strict_1.default.equal(bom.components.length, 2);
    strict_1.default.equal(bom.analysis_only, true);
});
(0, node_test_1.test)("loadCryptoBom rejects wrong schema", () => {
    strict_1.default.throws(() => (0, model_1.loadCryptoBom)({ ...SAMPLE, schema: "other/1" }), model_1.InvalidCryptoBomError);
});
(0, node_test_1.test)("parseCryptoBom rejects invalid JSON", () => {
    strict_1.default.throws(() => (0, model_1.parseCryptoBom)("{not json"), model_1.InvalidCryptoBomError);
});
(0, node_test_1.test)("loadCryptoBom rejects missing required field", () => {
    const broken = { ...SAMPLE };
    delete broken.tool;
    strict_1.default.throws(() => (0, model_1.loadCryptoBom)(broken), model_1.InvalidCryptoBomError);
});
(0, node_test_1.test)("sortComponents orders by descending priority", () => {
    const sorted = (0, formatters_1.sortComponents)(SAMPLE.components);
    strict_1.default.equal(sorted[0].id, "md5"); // 97
    strict_1.default.equal(sorted[1].id, "rsa"); // 80
});
(0, node_test_1.test)("formatTerminal includes disclaimer and both components", () => {
    const out = (0, formatters_1.formatTerminal)(SAMPLE, { color: false });
    strict_1.default.match(out, /static analysis/i);
    strict_1.default.match(out, /MD5/);
    strict_1.default.match(out, /RSA/);
    // No ANSI escapes when color disabled.
    strict_1.default.ok(!out.includes("\u001b["));
});
(0, node_test_1.test)("formatTerminal respects maxOccurrences", () => {
    const out = (0, formatters_1.formatTerminal)(SAMPLE, { color: false, maxOccurrences: 1 });
    strict_1.default.match(out, /and 2 more/);
});
(0, node_test_1.test)("formatMarkdown produces a component table", () => {
    const md = (0, formatters_1.formatMarkdown)(SAMPLE);
    strict_1.default.match(md, /\| Priority \| Band \| Component \|/);
    strict_1.default.match(md, /\| 97 \| critical \| MD5 \|/);
    strict_1.default.match(md, /## Details/);
});
(0, node_test_1.test)("formatHtml is a self-contained document", () => {
    const html = (0, formatters_1.formatHtml)(SAMPLE);
