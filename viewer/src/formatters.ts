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

