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
    }
    out.push("");
  }
  return out.join("\n") + "\n";
}

function mdEscape(text: string): string {
  return text.replace(/([|`])/g, "\\$1");
}

/* -------------------------------------------------------------------------- */
/* HTML                                                                       */
/* -------------------------------------------------------------------------- */

export function formatHtml(bom: CryptoBom): string {
  const rows = sortComponents(bom.components)
    .map((c) => {
      const occ = c.occurrences
        .map(
          (o) =>
            `<li><code>${htmlEscape(o.file)}:${o.line}</code> — ` +
            `<span class="excerpt">${htmlEscape(o.excerpt)}</span></li>`,
        )
        .join("");
      return `
      <tr class="band-${c.priority_band}">
        <td class="num">${c.priority}</td>
        <td><span class="badge badge-${c.priority_band}">${c.priority_band}</span></td>
        <td>${htmlEscape(c.name)}</td>
        <td>${htmlEscape(c.category)}</td>
        <td>${htmlEscape(c.quantum_risk_label)}</td>
        <td class="num">${c.file_count}</td>
        <td class="num">${c.occurrence_count}</td>
      </tr>
      <tr class="detail">
        <td colspan="7">
          <div class="guidance">${htmlEscape(c.guidance)}</div>
          <ul class="occurrences">${occ}</ul>
        </td>
      </tr>`;
    })
    .join("");

  const bandCounts = BAND_ORDER.map((band) => {
    const n = bom.summary.priority_bands[band] ?? 0;
    return n > 0
      ? `<span class="chip chip-${band}">${band}: ${n}</span>`
      : "";
  }).join("");

  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta name="viewport" content="width=device-width, initial-scale=1" />
<title>quantumseal CryptoBOM — ${htmlEscape(bom.root)}</title>
<style>
  :root { color-scheme: light dark; }
  body { font: 15px/1.5 system-ui, sans-serif; margin: 0; padding: 2rem; background: #0f1115; color: #e6e6e6; }
  h1 { font-size: 1.5rem; margin: 0 0 .25rem; }
  .disclaimer { background: #2a1f00; border-left: 4px solid #d9a300; padding: .6rem .9rem; border-radius: 4px; color: #ffe9a8; margin: 1rem 0; }
  .meta { color: #9aa4b2; margin-bottom: 1rem; }
  .chips { margin: 1rem 0; display: flex; gap: .5rem; flex-wrap: wrap; }
  .chip { padding: .2rem .6rem; border-radius: 999px; font-size: .8rem; font-weight: 600; }
  .chip-critical, .badge-critical { background: #7f1d1d; color: #fff; }
  .chip-high, .badge-high { background: #9a3412; color: #fff; }
  .chip-medium, .badge-medium { background: #854d0e; color: #fff; }
  .chip-low, .badge-low { background: #1e40af; color: #fff; }
  .chip-informational, .badge-informational { background: #374151; color: #d1d5db; }
  table { border-collapse: collapse; width: 100%; margin-top: 1rem; }
  th, td { text-align: left; padding: .5rem .6rem; border-bottom: 1px solid #232733; vertical-align: top; }
  th { color: #9aa4b2; font-size: .8rem; text-transform: uppercase; letter-spacing: .04em; }
  td.num { text-align: right; font-variant-numeric: tabular-nums; }
  .badge { padding: .1rem .5rem; border-radius: 4px; font-size: .75rem; font-weight: 600; }
  tr.detail td { background: #12151c; color: #c8d0da; }
  .guidance { margin-bottom: .5rem; }
  .occurrences { margin: 0; padding-left: 1.2rem; }
  .excerpt { color: #a6e3a1; }
  code { background: #1b1f27; padding: .1rem .3rem; border-radius: 3px; }
  footer { margin-top: 2rem; color: #6b7280; font-size: .8rem; }
</style>
</head>
<body>
  <h1>quantumseal CryptoBOM</h1>
  <div class="meta">${htmlEscape(bom.root)} · ${bom.tool} v${htmlEscape(bom.tool_version)} ·
    ${bom.summary.files_scanned} files scanned · ${bom.summary.component_count} components ·
    ${bom.summary.total_occurrences} occurrences</div>
  <div class="disclaimer">${htmlEscape(DISCLAIMER)}</div>
  <div class="chips">${bandCounts}</div>
  ${
    bom.components.length === 0
      ? `<p>No cryptographic indicators detected.</p>`
      : `<table>
    <thead>
      <tr><th>Priority</th><th>Band</th><th>Component</th><th>Category</th>
      <th>Quantum risk</th><th>Files</th><th>Occurrences</th></tr>
    </thead>
    <tbody>${rows}</tbody>
  </table>`
  }
  <footer>Generated by @quantumseal/viewer. Static analysis only — not a cryptographic implementation.</footer>
</body>
</html>
`;
}

function htmlEscape(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#39;");
}

/** Group export kept for potential downstream tooling/testing. */
export { groupByBand, sortComponents };
