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
