/**
 * Command guidance system — provides contextual hints for tool selection and usage.
 *
 * Guidance data is loaded at runtime from data/command-guidance.yaml.
 * Call loadCommandGuidance() once at server startup before using getCommandGuidance().
 */

import { readFile } from "fs/promises";
import { join, dirname } from "path";
import { fileURLToPath } from "url";
import { existsSync } from "fs";
import { parse as parseYaml } from "yaml";
import type { QsvSkill } from "./types.js";
import { config } from "./config.js";

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Consolidated guidance for each command.
 * Combines when-to-use, common patterns, error prevention,
 * and behavioral flags into a single lookup.
 */
export interface CommandGuidance {
  whenToUse?: string;
  commonPattern?: string;
  errorPrevention?: string;
  needsMemoryWarning?: boolean;
  needsIndexHint?: boolean;
  hasCommonMistakes?: boolean;
}

// Module-level cache — populated by loadCommandGuidance(), read by getCommandGuidance()
const DANGEROUS_KEYS = new Set(["__proto__", "constructor", "prototype"]);

let commandGuidance: Record<string, CommandGuidance> =
  Object.create(null) as Record<string, CommandGuidance>;
let guidanceLoaded = false;

/**
 * Resolve the path to command-guidance.yaml.
 * Handles both production (dist/command-guidance.js → ../data/)
 * and test (dist/src/command-guidance.js → ../../data/) layouts.
 */
function resolveGuidancePath(): string {
  // Production: dist/command-guidance.js → ../data/
  const productionPath = join(__dirname, "../data/command-guidance.yaml");
  // Test build: dist/src/command-guidance.js → ../../data/
  const testPath = join(__dirname, "../../data/command-guidance.yaml");

  if (existsSync(productionPath)) return productionPath;
  if (existsSync(testPath)) return testPath;
  return productionPath; // fallback for clear error message
}

const VALID_STRING_FIELDS = ["whenToUse", "commonPattern", "errorPrevention"];
const VALID_BOOLEAN_FIELDS = [
  "needsMemoryWarning",
  "needsIndexHint",
  "hasCommonMistakes",
];

/** Type guard for a single guidance entry. */
const ALL_VALID_FIELDS = [...VALID_STRING_FIELDS, ...VALID_BOOLEAN_FIELDS];

/** Type guard for a single guidance entry. Warns on unrecognized keys (likely typos). */
function isValidGuidanceEntry(
  entry: unknown,
  cmd?: string,
): entry is CommandGuidance {
  if (typeof entry !== "object" || entry === null) return false;
  const obj = entry as Record<string, unknown>;

  for (const key of VALID_STRING_FIELDS) {
    if (key in obj && typeof obj[key] !== "string") return false;
  }
  for (const key of VALID_BOOLEAN_FIELDS) {
    if (key in obj && typeof obj[key] !== "boolean") return false;
  }

  // Warn on unrecognized keys — catches typos like "wehnToUse"
  const unknownKeys = Object.keys(obj).filter(
    (k) => !ALL_VALID_FIELDS.includes(k),
  );
  for (const key of unknownKeys) {
    console.error(
      `[Guidance] Unknown key "${key}" in entry "${cmd ?? "?"}" — possible typo`,
    );
  }

  // Reject entries where ALL keys are unrecognized (entirely bogus entry)
  if (unknownKeys.length > 0 && unknownKeys.length === Object.keys(obj).length) {
    return false;
  }
  return true;
}

/**
 * Load command guidance from YAML file.
 * Caches result after first successful load; subsequent calls return the cache.
 */
export async function loadCommandGuidance(): Promise<
  Record<string, CommandGuidance>
> {
  if (guidanceLoaded) return commandGuidance;

  const yamlPath = resolveGuidancePath();
  try {
    const content = await readFile(yamlPath, "utf-8");
    const parsed = parseYaml(content) as Record<string, unknown>;

    if (typeof parsed !== "object" || parsed === null || Array.isArray(parsed)) {
      throw new Error("YAML root must be a mapping, not a sequence or scalar");
    }

    const result: Record<string, CommandGuidance> =
      Object.create(null) as Record<string, CommandGuidance>;
    for (const [cmd, entry] of Object.entries(parsed)) {
      if (DANGEROUS_KEYS.has(cmd)) {
        console.error(`[Guidance] Rejecting dangerous key "${cmd}"`);
        continue;
      }
      if (isValidGuidanceEntry(entry, cmd)) {
        result[cmd] = entry;
      } else {
        console.error(`[Guidance] Skipping invalid entry for "${cmd}"`);
      }
    }

    if (Object.keys(result).length === 0) {
      throw new Error("YAML parsed but produced 0 valid entries");
    }

    commandGuidance = result;
    guidanceLoaded = true;
    console.error(
      `[Guidance] Loaded ${Object.keys(result).length} command guidance entries`,
    );
    return commandGuidance;
  } catch (error) {
    console.error(`[Guidance] Failed to load ${yamlPath}:`, error);
    // Leave guidanceLoaded = false so a subsequent call can retry
    // (e.g., transient I/O failure at startup)
    return commandGuidance;
  }
}

/**
 * Get already-loaded guidance (synchronous).
 * Returns empty object if loadCommandGuidance() hasn't been called yet.
 */
export function getCommandGuidance(): Record<string, CommandGuidance> {
  return commandGuidance;
}

/**
 * Reset guidance state (for testing only).
 */
export function _resetGuidance(): void {
  commandGuidance = Object.create(null) as Record<string, CommandGuidance>;
  guidanceLoaded = false;
}

/**
 * Enhance parameter descriptions with examples and common values
 */
export function enhanceParameterDescription(
  paramName: string,
  description: string,
): string {
  let enhanced = description;

  // Add examples for common parameters
  switch (paramName) {
    case "delimiter":
      enhanced += ' e.g. "," "\\t" "|" ";"';
      break;
    case "select":
      enhanced +=
        ' e.g. "1,3,5" (specific columns), "1-10" (range), "!SSN,!password" (exclude), "name,age,city" (by name), "_" (last column), "/<regex>/" (regex).';
      break;
    case "output":
    case "output_file":
      enhanced +=
        " Tip: Use absolute paths. Omit for small results (returned directly), or specify for large datasets (auto-saved if >850KB).";
      break;
    case "no_headers":
      enhanced +=
        " Use when CSV has no header row. First row will be treated as data.";
      break;
    case "ignore_case":
      enhanced += " Makes pattern matching case-insensitive.";
      break;
  }

  return enhanced;
}

/**
 * Enhance tool description with contextual guidance
 *
 * Uses concise description from README.md and adds guidance hints
 * that help Claude select the right tool. For detailed help,
 * use the qsv_help tool which calls `qsv <command> --help`.
 */
export function enhanceDescription(skill: QsvSkill): string {
  const commandName = skill.command.subcommand;
  const guidance = getCommandGuidance()[commandName];

  // Use concise description from README.md
  let description = skill.description;

  // Add when-to-use guidance (critical for tool selection)
  if (guidance?.whenToUse) {
    description += `\n\n💡 ${guidance.whenToUse}`;
  }

  // Add subcommand requirement for commands that need it
  if (commandName === "cat") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "rows", input: "file.csv"}).`;
  } else if (commandName === "geocode") {
    description += `\n\n🔧 SUBCOMMAND: Must pass subcommand via args (e.g., args: {subcommand: "suggest", column: "city", input: "data.csv"}).`;
  }

  // Add common patterns (helps Claude compose workflows)
  if (guidance?.commonPattern) {
    description += `\n\n📋 ${guidance.commonPattern}`;
  }

  // Add performance hints only for commands that benefit from indexing
  if (skill.hints) {
    // Only show memory warnings for memory-intensive commands
    if (guidance?.needsMemoryWarning) {
      if (skill.hints.memory === "full") {
        description += "\n\n⚠️  Loads entire CSV. Best <100MB.";
      } else if (skill.hints.memory === "proportional") {
        description += "\n\n⚠️  Memory ∝ unique values.";
      }
    }

    // Only show index hints for commands that are index-accelerated
    if (guidance?.needsIndexHint && skill.hints.indexed) {
      description +=
        "\n\n🚀 Index-accelerated. Run qsv_index first on files >10MB.";
    }
  }

  // Add error prevention hints only for commands with common mistakes
  if (guidance?.hasCommonMistakes && guidance?.errorPrevention) {
    description += `\n\n⚠️  ${guidance.errorPrevention}`;
  }

  // Add usage examples from skill JSON (if available)
  // Configurable via QSV_MCP_MAX_EXAMPLES environment variable (default: 5, max: 20, 0 to disable)
  if (skill.examples && skill.examples.length > 0 && config.maxExamples > 0) {
    const maxExamples = config.maxExamples;
    const examplesToShow = skill.examples.slice(0, maxExamples);

    description += "\n\n📝 EXAMPLES:";
    for (const example of examplesToShow) {
      description += `\n• ${example.command}`;
    }

    if (skill.examples.length > maxExamples) {
      description += `\n  (${skill.examples.length - maxExamples} more - use help=true for full list)`;
    }
  }

  return description;
}
