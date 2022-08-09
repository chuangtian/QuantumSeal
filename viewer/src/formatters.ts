/**
 * Report formatters for quantumseal CryptoBOM documents.
 *
 * Three output formats, all produced with the standard library only:
 *   - `formatTerminal`  ANSI-colored text for a console.
 *   - `formatMarkdown`  GitHub-flavored Markdown with a summary table.
 *   - `formatHtml`      a self-contained, dependency-free HTML page.
 */

import {
  BAND_ORDER,
  type Component,
  type CryptoBom,
  type PriorityBand,
} from "./model";

const DISCLAIMER =
  "This report presents static analysis of cryptographic indicators only. " +
  "It is not a cryptographic implementation and not a security audit; every " +
  "finding warrants human review.";

/** ANSI color codes (empty when color is disabled). */
interface Palette {
  reset: string;
  bold: string;
  dim: string;
  red: string;
  yellow: string;
  blue: string;
  green: string;
  magenta: string;
}

function palette(enabled: boolean): Palette {
  if (!enabled) {
    return {
      reset: "",
      bold: "",
      dim: "",
      red: "",
      yellow: "",
      blue: "",
      green: "",
      magenta: "",
    };
  }
  return {
    reset: "\u001b[0m",
    bold: "\u001b[1m",
    dim: "\u001b[2m",
    red: "\u001b[31m",
    yellow: "\u001b[33m",
    blue: "\u001b[34m",
    green: "\u001b[32m",
    magenta: "\u001b[35m",
  };
}

function bandColor(p: Palette, band: PriorityBand): string {
  switch (band) {
    case "critical":
      return p.red + p.bold;
    case "high":
      return p.red;
    case "medium":
      return p.yellow;
    case "low":
      return p.blue;
    case "informational":
