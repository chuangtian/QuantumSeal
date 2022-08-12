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
  lines.push("");

  if (bom.components.length === 0) {
    lines.push(`${p.green}No cryptographic indicators detected.${p.reset}`);
    return lines.join("\n") + "\n";
  }

  for (const c of sortComponents(bom.components)) {
    const color = bandColor(p, c.priority_band);
    lines.push(
      `${color}[${String(c.priority).padStart(3)}] ${c.name}` +
        ` — ${c.priority_band.toUpperCase()}${p.reset}`,
    );
    lines.push(
      `      ${p.dim}risk=${c.quantum_risk} category=${c.category}` +
        ` files=${c.file_count} occurrences=${c.occurrence_count}${p.reset}`,
    );
    lines.push(`      ${c.guidance}`);
    const shown = maxOcc === 0 ? c.occurrences : c.occurrences.slice(0, maxOcc);
    for (const occ of shown) {
      lines.push(`        ${p.dim}${occ.file}:${occ.line}${p.reset}  «${occ.excerpt}»`);
    }
    if (maxOcc !== 0 && c.occurrences.length > maxOcc) {
      lines.push(`        ${p.dim}… and ${c.occurrences.length - maxOcc} more${p.reset}`);
    }
    lines.push("");
  }
  return lines.join("\n") + "\n";
}

/* -------------------------------------------------------------------------- */
/* Markdown                                                                   */
/* -------------------------------------------------------------------------- */

export function formatMarkdown(bom: CryptoBom): string {
  const out: string[] = [];
  out.push(`# quantumseal CryptoBOM — \`${bom.root}\``);
  out.push("");
  out.push(`> ${DISCLAIMER}`);
  out.push("");
  out.push(
    `**Tool:** ${bom.tool} v${bom.tool_version} · ` +
      `**Files scanned:** ${bom.summary.files_scanned} · ` +
      `**Components:** ${bom.summary.component_count} · ` +
      `**Occurrences:** ${bom.summary.total_occurrences}`,
  );
  out.push("");

  // Band histogram table.
  out.push("## Priority summary");
  out.push("");
  out.push("| Band | Components |");
  out.push("| --- | ---: |");
  for (const band of BAND_ORDER) {
    const count = bom.summary.priority_bands[band] ?? 0;
    if (count > 0) {
      out.push(`| ${band} | ${count} |`);
    }
  }
  out.push("");

  if (bom.components.length === 0) {
    out.push("_No cryptographic indicators detected._");
    return out.join("\n") + "\n";
  }

  // Component table.
  out.push("## Components");
  out.push("");
  out.push("| Priority | Band | Component | Category | Quantum risk | Files | Occurrences |");
  out.push("| ---: | --- | --- | --- | --- | ---: | ---: |");
  for (const c of sortComponents(bom.components)) {
    out.push(
      `| ${c.priority} | ${c.priority_band} | ${mdEscape(c.name)} | ${c.category} | ` +
        `${c.quantum_risk} | ${c.file_count} | ${c.occurrence_count} |`,
    );
  }
  out.push("");

  // Guidance + occurrences detail.
  out.push("## Details");
  out.push("");
  for (const c of sortComponents(bom.components)) {
    out.push(`### ${mdEscape(c.name)} (priority ${c.priority}, ${c.priority_band})`);
    out.push("");
    out.push(`- **Quantum risk:** ${c.quantum_risk_label}`);
    out.push(`- **Guidance:** ${mdEscape(c.guidance)}`);
    out.push("");
    for (const occ of c.occurrences) {
      out.push(`  - \`${occ.file}:${occ.line}\` — \`${mdEscape(occ.excerpt)}\``);
