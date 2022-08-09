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
      return p.dim;
  }
}

/** Sort components most-urgent first, matching the CLI ordering. */
function sortComponents(components: readonly Component[]): Component[] {
  return [...components].sort((a, b) => {
    if (b.priority !== a.priority) {
      return b.priority - a.priority;
    }
    return a.name.localeCompare(b.name);
  });
}

/** Group components by band, preserving BAND_ORDER. */
function groupByBand(
  components: readonly Component[],
): Map<PriorityBand, Component[]> {
  const map = new Map<PriorityBand, Component[]>();
  for (const band of BAND_ORDER) {
    map.set(band, []);
  }
  for (const c of sortComponents(components)) {
    map.get(c.priority_band)?.push(c);
  }
  return map;
}

/** Options common to the formatters. */
export interface FormatOptions {
  /** Enable ANSI color (terminal only). */
  color?: boolean;
  /** Max occurrences to show per component (default 5, 0 = all). */
  maxOccurrences?: number;
}

/* -------------------------------------------------------------------------- */
/* Terminal                                                                   */
/* -------------------------------------------------------------------------- */

export function formatTerminal(bom: CryptoBom, options: FormatOptions = {}): string {
  const p = palette(options.color ?? false);
  const maxOcc = options.maxOccurrences ?? 5;
  const lines: string[] = [];

  lines.push(`${p.bold}quantumseal CryptoBOM${p.reset} — ${bom.root}`);
  lines.push(`${p.dim}${DISCLAIMER}${p.reset}`);
  lines.push("=".repeat(72));
  lines.push(
    `tool ${bom.tool} v${bom.tool_version}   ` +
      `files=${bom.summary.files_scanned}   ` +
      `components=${bom.summary.component_count}   ` +
      `occurrences=${bom.summary.total_occurrences}`,
  );
