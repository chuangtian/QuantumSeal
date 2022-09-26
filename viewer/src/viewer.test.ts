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
