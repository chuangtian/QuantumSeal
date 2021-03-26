/**
 * Report formatters for quantumseal CryptoBOM documents.
 *
 * Three output formats, all produced with the standard library only:
 *   - `formatTerminal`  ANSI-colored text for a console.
 *   - `formatMarkdown`  GitHub-flavored Markdown with a summary table.
 *   - `formatHtml`      a self-contained, dependency-free HTML page.
 */
import { type Component, type CryptoBom, type PriorityBand } from "./model";
/** Sort components most-urgent first, matching the CLI ordering. */
declare function sortComponents(components: readonly Component[]): Component[];
/** Group components by band, preserving BAND_ORDER. */
declare function groupByBand(components: readonly Component[]): Map<PriorityBand, Component[]>;
/** Options common to the formatters. */
export interface FormatOptions {
    /** Enable ANSI color (terminal only). */
    color?: boolean;
