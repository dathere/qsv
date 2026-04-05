/**
 * MCP tool dispatch and handler implementations.
 */

import { randomUUID } from "crypto";
import { stat } from "fs/promises";
import { basename, dirname, extname, isAbsolute, join } from "path";
import { tmpdir } from "os";
import type { Server } from "@modelcontextprotocol/sdk/server/index.js";
import type {
  FilesystemProviderExtended,
  SkillCategory,
} from "./types.js";
import { SKILL_CATEGORIES } from "./types.js";
import type { SkillExecutor } from "./executor.js";
import { runQsvSimple } from "./executor.js";
import type { SkillLoader } from "./loader.js";
import { executeDescribegptWithSampling, runQsvCapture } from "./mcp-sampling.js";
import { config, getDetectionDiagnostics, MINIMUM_QSV_VERSION } from "./config.js";
import { formatBytes, errorResult, successResult, getErrorMessage, isReservedCachePath, reservedCachePathError, describegptFallbackResult } from "./utils.js";
import {
  getDuckDbStatus,
} from "./duckdb.js";
import {
  PIPELINE_METADATA,
  FINAL_OUTPUT_FILE,
  COMMON_COMMANDS,
  NON_TABULAR_COMMANDS,
  FILE_PATH_OUTPUT_OPTIONS,
  MAX_LOG_MESSAGE_LEN,
  LOG_ENTRY_TYPES,
  isBinaryOutputFormat,
} from "./tool-constants.js";
import type { PipelineMetadata } from "./tool-constants.js";
import { acquireSlot, releaseSlot, getActiveOperationCount } from "./concurrency.js";
import {
  getCurrentWorkingDir,
  resolveAndConvertInputFile,
  autoIndexIfNeeded,
  buildFileNotFoundError,
  resolveFilePathParams,
  buildSkillExecParams,
  collectAdditionalInputFiles,
  formatToolResult,
  resolveParamAliases,
  shouldUseTempFile,
  paramKeyToFlag,
} from "./file-operations.js";
import {
  ensureParquet,
  ensureStatsCache,
  convertCsvToParquet,
  tryDuckDbExecution,
} from "./parquet-bridge.js";
import {
  MULTI_TABLE_PATTERN,
  normalizeTableRefs,
  translateSql,
} from "./duckdb.js";
import { COMMAND_GUIDANCE } from "./command-guidance.js";

/** Token usage from a describegpt cached response. */
interface DescribegptTokenUsage {
  prompt: number;
  completion: number;
  total: number;
  elapsed: number;
}

/** A single phase from describegpt --prepare-context output. */
interface DescribegptPhase {
  kind: string;
  system_prompt?: string;
  user_prompt?: string;
  cached_response: {
    response: string;
    reasoning: string;
    token_usage: DescribegptTokenUsage;
  } | null;
}

/** Output structure of describegpt --prepare-context. */
interface DescribegptPrepareOutput {
  phases: DescribegptPhase[];
  analysis_results: unknown;
  model: string;
}

/** A single phase response (cached or agent-provided) for describegpt --process-response. */
interface PhaseResponse {
  kind: string;
  response: string;
  reasoning: string;
  token_usage: DescribegptTokenUsage;
}

/**
 * Run --prepare-context and return prompts to the agent for LLM inference.
 * Used when MCP sampling is not available (e.g., Claude Desktop).
 * The agent answers the prompts, then calls the tool again with _llm_responses.
 */
async function prepareContextForAgent(
  params: Record<string, unknown>,
  inputFile: string,
  outputFile: string | undefined,
  workingDir: string,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
  // Ensure output_file is fully qualified so --process-response writes to the right place
  const resolvedOutput = outputFile && !isAbsolute(outputFile)
    ? join(workingDir, outputFile)
    : outputFile;
  const cliArgs = buildDescribegptArgs(params, inputFile, resolvedOutput);
  const prepareArgs = [...cliArgs, "--prepare-context"];
  const result = await runQsvCapture(config.qsvBinPath, ["describegpt", ...prepareArgs], {
    cwd: workingDir,
    timeoutMs: 300_000,
  });

  if (result.exitCode !== 0) {
    return errorResult(`describegpt --prepare-context failed:\n${result.stderr}`);
  }

  let prepareOutput: DescribegptPrepareOutput;
  try {
    prepareOutput = JSON.parse(result.stdout);
  } catch {
    return errorResult(`Failed to parse --prepare-context JSON output:\n${result.stdout.slice(0, 500)}`);
  }

  // Build prompt sections for uncached phases
  const promptSections: string[] = [];
  const cachedPhases: string[] = [];

  for (const phase of prepareOutput.phases) {
    if (phase.cached_response) {
      cachedPhases.push(phase.kind);
      continue;
    }
    promptSections.push(
      `## Phase: ${phase.kind}\n\n` +
      `**System prompt:**\n${phase.system_prompt}\n\n` +
      `**User prompt:**\n${phase.user_prompt}`,
    );
  }

  if (promptSections.length === 0) {
    // All phases cached — process directly
    return await processAgentResponses([], params, inputFile, outputFile, workingDir);
  }

  const cachedNote = cachedPhases.length > 0
    ? `\n\nCached (no response needed): ${cachedPhases.join(", ")}`
    : "";

  const uncachedKinds = prepareOutput.phases
    .filter((p) => !p.cached_response)
    .map((p) => `{"kind": "${p.kind}", "response": "<your response>"}`)
    .join(", ");

  return successResult(
    `describegpt needs LLM inference for ${promptSections.length} phase(s).${cachedNote}\n\n` +
    `Please respond to each prompt below, then call this tool again with the SAME parameters ` +
    `plus \`_llm_responses\` containing your answers.\n\n` +
    promptSections.join("\n\n---\n\n") +
    `\n\n---\n\n` +
    `**To complete:** Call qsv_describegpt again with the same options plus:\n` +
    `\`_llm_responses\`: [${uncachedKinds}]`,
  );
}

/**
 * Process agent-provided LLM responses for describegpt.
 * Re-runs --prepare-context to get cached responses, merges with agent responses,
 * then runs --process-response.
 */
async function processAgentResponses(
  llmResponses: Array<{ kind: string; response: string }>,
  params: Record<string, unknown>,
  inputFile: string,
  outputFile: string | undefined,
  workingDir: string,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
  // Ensure output_file is fully qualified so --process-response writes to the right place
  const resolvedOutput = outputFile && !isAbsolute(outputFile)
    ? join(workingDir, outputFile)
    : outputFile;
  // Re-run prepare-context to get analysis_results and cached responses
  const cliArgs = buildDescribegptArgs(params, inputFile, resolvedOutput);
  const prepareArgs = [...cliArgs, "--prepare-context"];
  const phase1 = await runQsvCapture(config.qsvBinPath, ["describegpt", ...prepareArgs], {
    cwd: workingDir,
    timeoutMs: 300_000,
  });

  if (phase1.exitCode !== 0) {
    return errorResult(`describegpt --prepare-context failed:\n${phase1.stderr}`);
  }

  let prepareOutput: DescribegptPrepareOutput;
  try {
    prepareOutput = JSON.parse(phase1.stdout);
  } catch {
    return errorResult(`Failed to parse --prepare-context JSON output:\n${phase1.stdout.slice(0, 500)}`);
  }

  // Build phase responses: cached + agent-provided
  const phaseResponses: PhaseResponse[] = [];
  for (const phase of prepareOutput.phases) {
    if (phase.cached_response) {
      phaseResponses.push({
        kind: phase.kind,
        response: phase.cached_response.response,
        reasoning: phase.cached_response.reasoning,
        token_usage: phase.cached_response.token_usage,
      });
    } else {
      const agentResponse = llmResponses.find((r) => r.kind === phase.kind);
      if (!agentResponse) {
        return errorResult(`Missing response for phase "${phase.kind}"`);
      }
      phaseResponses.push({
        kind: phase.kind,
        response: agentResponse.response,
        reasoning: "",
        token_usage: { prompt: 0, completion: 0, total: 0, elapsed: 0 },
      });
    }
  }

  // Run process-response
  const processInput = {
    phases: phaseResponses,
    analysis_results: prepareOutput.analysis_results,
    model: prepareOutput.model,
  };

  const processArgs = [...cliArgs, "--process-response"];
  const phase3 = await runQsvCapture(config.qsvBinPath, ["describegpt", ...processArgs], {
    cwd: workingDir,
    stdinData: JSON.stringify(processInput),
    timeoutMs: 300_000,
  });

  if (phase3.exitCode !== 0) {
    return errorResult(`describegpt --process-response failed:\n${phase3.stderr}`);
  }

  const resultText = phase3.stdout.trim()
    ? phase3.stdout
    : describegptFallbackResult(cliArgs);
  return successResult(resultText);
}

/**
 * Build CLI args for describegpt from MCP tool params.
 * Translates MCP parameter names back to qsv CLI flags.
 */
function buildDescribegptArgs(
  params: Record<string, unknown>,
  inputFile: string,
  outputFile?: string,
): string[] {
  const args: string[] = [];

  // Map known params to CLI flags
  const flagMap: Record<string, string> = {
    dictionary: "--dictionary",
    description: "--description",
    tags: "--tags",
    all: "--all",
    prompt: "--prompt",
    base_url: "--base-url",
    model: "--model",
    api_key: "--api-key",
    max_tokens: "--max-tokens",
    format: "--format",
    num_tags: "--num-tags",
    tag_vocab: "--tag-vocab",
    num_examples: "--num-examples",
    truncate_str: "--truncate-str",
    stats_options: "--stats-options",
    freq_options: "--freq-options",
    enum_threshold: "--enum-threshold",
    prompt_file: "--prompt-file",
    sample_size: "--sample-size",
    sql_results: "--sql-results",
    language: "--language",
    addl_props: "--addl-props",
    timeout: "--timeout",
    user_agent: "--user-agent",
    addl_cols: "--addl-cols",
    addl_cols_list: "--addl-cols-list",
    session: "--session",
    session_len: "--session-len",
    no_cache: "--no-cache",
    disk_cache_dir: "--disk-cache-dir",
    redis_cache: "--redis-cache",
    fresh: "--fresh",
    quiet: "--quiet",
    fewshot_examples: "--fewshot-examples",
    delimiter: "--delimiter",
    no_headers: "--no-headers",
  };

  // Build a reverse lookup: normalized param name → CLI flag
  // This handles params passed as "dictionary", "--dictionary", or "—dictionary"
  const reverseLookup = new Map<string, string>();
  for (const [param, flag] of Object.entries(flagMap)) {
    reverseLookup.set(param, flag);
  }

  for (const [rawParam, value] of Object.entries(params)) {
    // Normalize: strip leading --, convert - to _
    const normalized = rawParam.replace(/^--/, "").replace(/-/g, "_");
    const flag = reverseLookup.get(normalized);
    if (!flag) continue;
    if (value === undefined || value === null || value === false) continue;
    if (value === true) {
      args.push(flag);
    } else {
      args.push(flag, String(value));
    }
  }

  if (outputFile) {
    args.push("--output", outputFile);
  }

  // Input file goes last
  args.push(inputFile);

  return args;
}

/**
 * Handle execution of a qsv tool
 */
export async function handleToolCall(
  toolName: string,
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
  server?: Server,
) {
  // Acquire concurrency slot (queue if all slots are busy)
  const slotResult = await acquireSlot(config.concurrencyWaitTimeoutMs);
  if (slotResult !== true) {
    const activeOps = getActiveOperationCount();
    const reason = slotResult === "backpressure"
      ? `Concurrency queue is full (${activeOps} operations running). `
      : `Operation queued but timed out after ${Math.round(config.concurrencyWaitTimeoutMs / 1000)}s waiting for a slot. ` +
        `${activeOps} operation${activeOps !== 1 ? "s" : ""} still running. `;
    return errorResult(reason + `Try running operations sequentially.`);
  }

  try {
    // Extract command name from tool name (qsv_select -> select)
    const commandName = toolName.replace("qsv_", "");

    // Load the skill
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);

    if (!skill) {
      const totalCommands = loader.getStats().total;
      const remainingCommands = totalCommands - COMMON_COMMANDS.length;

      return errorResult(
        `Error: Skill '${skillName}' not found.\n\n` +
        `Please verify the command name is correct. ` +
        `Available commands include: ${COMMON_COMMANDS.join(", ")}, and ${remainingCommands} others. ` +
        `Use 'qsv_command' with the 'command' parameter for less common commands.`,
      );
    }

    // Extract input_file and output_file (with LLM alias resolution)
    const { inputFile: rawInputFile, outputFile: rawOutputFile } = resolveParamAliases(params);
    let inputFile = rawInputFile;
    let outputFile = rawOutputFile;
    const isHelpRequest = params.help === true || params["--help"] === true;

    // Normalize help flags: once we've interpreted "--help", remove it so it
    // is not forwarded as a duplicate CLI option alongside options.help=true.
    if ("--help" in params) {
      delete params["--help"];
    }

    if (!inputFile && !isHelpRequest) {
      return errorResult("Error: input_file parameter is required (unless using help=true to view command documentation)");
    }

    // Resolve file paths using filesystem provider if available (skip for help requests)
    if (filesystemProvider && inputFile) {
      try {
        inputFile = await resolveAndConvertInputFile(inputFile, filesystemProvider);

        // Auto-index native CSV files (skip for help requests)
        if (!isHelpRequest) {
          await autoIndexIfNeeded(inputFile);
        }

        if (outputFile) {
          const originalOutputFile = outputFile;
          outputFile = await filesystemProvider.resolvePath(outputFile);
          console.error(
            `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${outputFile}`,
          );
        }
      } catch (error: unknown) {
        console.error(`[MCP Tools] Error resolving file path:`, error);
        const errorMessage = await buildFileNotFoundError(
          inputFile,
          error,
          filesystemProvider,
        );
        return errorResult(errorMessage);
      }
    }

    // Resolve additional file-path parameters (positional args and options beyond input/output)
    if (filesystemProvider && !isHelpRequest) {
      await resolveFilePathParams(params, skill, filesystemProvider);
    }

    // Prevent overwriting reserved cache files (output_file and file-path output options)
    if (outputFile && isReservedCachePath(outputFile)) {
      return errorResult(reservedCachePathError(outputFile));
    }
    for (const [key, value] of Object.entries(params)) {
      if (!value || typeof value !== "string") continue;
      const flag = paramKeyToFlag(key);
      if (FILE_PATH_OUTPUT_OPTIONS.has(flag) && isReservedCachePath(value)) {
        return errorResult(reservedCachePathError(value));
      }
    }

    // DuckDB/Parquet-first interception for sqlp queries
    let parquetConversionWarning = "";
    if (commandName === "sqlp" && !isHelpRequest && inputFile) {
      const rawSql = params.sql as string | undefined;
      if (rawSql) {
        // Normalize uppercase _T_N references to lowercase _t_N for consistent handling
        const sql = normalizeTableRefs(rawSql);
        params.sql = sql;
        try {
          // Skip DuckDB for multi-table queries (_t_2, _t_3, etc.) — sqlp handles
          // multiple input files natively, so let the original input flow through.
          // NOTE: Multi-table queries don't benefit from Parquet auto-conversion;
          // users should manually convert all input files with qsv_to_parquet first.
          if (MULTI_TABLE_PATTERN.test(sql)) {
            console.error(`[MCP Tools] DuckDB: Multi-table query detected (_t_2+), falling back to sqlp`);
          } else {
            // Auto-convert CSV to Parquet (skip for SKIP_INPUT which already has explicit refs)
            let parquetFile = inputFile;
            if (inputFile !== "SKIP_INPUT") {
              parquetFile = await ensureParquet(inputFile);
            }

            // Try DuckDB execution (single-table path)
            const duckDbResult = await tryDuckDbExecution(sql, parquetFile, params, outputFile);
            if (duckDbResult !== null) {
              return duckDbResult;
            }

            // DuckDB unavailable or unsupported format — fall through to sqlp
            // If we converted to Parquet, rewrite SQL via translateSql and use SKIP_INPUT
            if (parquetFile !== inputFile && parquetFile.endsWith(".parquet")) {
              const rewrittenSql = translateSql(sql, parquetFile);
              params.sql = rewrittenSql;
              inputFile = "SKIP_INPUT";
              console.error(`[MCP Tools] sqlp fallback with Parquet: ${rewrittenSql}`);
            }
          }
        } catch (error: unknown) {
          // Parquet conversion or DuckDB failed — warn and fall through to sqlp with original input
          const errorMsg = getErrorMessage(error);
          console.error(
            `[MCP Tools] Parquet/DuckDB interception failed, falling back to sqlp:`,
            errorMsg,
          );
          // Include warning in result so the agent/user knows the optimization was skipped
          parquetConversionWarning = `[Warning] Parquet auto-conversion was skipped (${errorMsg}). Query ran against original CSV which may be slower.`;
        }
      }
    }

    // Determine if we should use a temp file for output (skip for help requests
    // and binary output formats like parquet/arrow/avro which can't be read as UTF-8)
    let autoCreatedTempFile = false;
    if (
      !outputFile &&
      !isHelpRequest &&
      inputFile &&
      commandName !== "describegpt" &&
      !isBinaryOutputFormat(commandName, params) &&
      (await shouldUseTempFile(commandName, inputFile))
    ) {
      const tempExt = config.outputFormat === "tsv" && !NON_TABULAR_COMMANDS.has(commandName) ? "tsv" : "csv";
      const tempFileName = `qsv-output-${randomUUID()}.${tempExt}`;
      outputFile = join(tmpdir(), tempFileName);
      autoCreatedTempFile = true;
      console.error(`[MCP Tools] Auto-created temp output file: ${outputFile}`);
    }

    // Build execution parameters
    const { args, options } = buildSkillExecParams(
      skill,
      params,
      inputFile,
      outputFile,
      isHelpRequest,
    );

    console.error(
      `[MCP Tools] Executing skill with args:`,
      JSON.stringify(args),
    );
    console.error(
      `[MCP Tools] Executing skill with options:`,
      JSON.stringify(options),
    );

    // Intercept describegpt: use MCP sampling or agent-as-LLM fallback
    // Skip for help requests — let them fall through to normal execution
    if (commandName === "describegpt" && server && inputFile && !isHelpRequest) {
      // Block SQL RAG mode (--prompt) in MCP server mode
      if (params.prompt !== undefined || params["--prompt"] !== undefined) {
        return errorResult(
          `The --prompt option (SQL RAG chat mode) is not supported in MCP server mode.\n\n` +
          `In MCP mode, use describegpt for data dictionaries, descriptions, and tags only (--dictionary, --description, --tags, --all).\n` +
          `For natural language questions about your data, ask the connected LLM directly — ` +
          `it can use other qsv tools (sqlp, frequency, stats) to answer your question.`,
        );
      }

      // Block LLM API options that don't apply in MCP mode (sampling handles these)
      const blockedLlmParams = ["base_url", "api_key", "model", "max_tokens"];
      for (const param of blockedLlmParams) {
        const dashParam = `--${param.replace(/_/g, "-")}`;
        if (params[param] !== undefined || params[dashParam] !== undefined) {
          return errorResult(
            `The --${param.replace(/_/g, "-")} option is not needed in MCP server mode.\n` +
            `describegpt uses the connected LLM automatically via MCP sampling — no API configuration required.`,
          );
        }
      }

      // Check if this is a Phase 3 callback with agent-provided LLM responses.
      // The value may arrive as a JSON string (when passed through a typed schema that
      // doesn't declare it) or as an already-parsed array.
      let llmResponses: Array<{ kind: string; response: string }> | undefined;
      const rawLlmResponses = params._llm_responses;

      // Validate that every element in the array has the required shape.
      const validateLlmResponseElements = (arr: unknown[]): string | null => {
        for (let i = 0; i < arr.length; i++) {
          const el = arr[i];
          if (el === null || typeof el !== "object" || Array.isArray(el)) {
            return `_llm_responses[${i}] must be an object with "kind" and "response" string fields, got ${el === null ? "null" : Array.isArray(el) ? "array" : typeof el}.`;
          }
          const obj = el as Record<string, unknown>;
          if (typeof obj.kind !== "string" || typeof obj.response !== "string") {
            return `_llm_responses[${i}] must have "kind" and "response" string fields.`;
          }
        }
        return null;
      };

      if (rawLlmResponses !== undefined) {
        if (typeof rawLlmResponses === "string") {
          try {
            const parsed: unknown = JSON.parse(rawLlmResponses);
            if (!Array.isArray(parsed)) {
              return errorResult(
                `_llm_responses must be a JSON array, got ${typeof parsed}.`,
              );
            }
            const validationError = validateLlmResponseElements(parsed);
            if (validationError) {
              return errorResult(validationError);
            }
            llmResponses = parsed as Array<{ kind: string; response: string }>;
          } catch {
            return errorResult(
              `Failed to parse _llm_responses JSON string. Expected an array of {kind, response} objects.`,
            );
          }
        } else if (Array.isArray(rawLlmResponses)) {
          const validationError = validateLlmResponseElements(rawLlmResponses);
          if (validationError) {
            return errorResult(validationError);
          }
          llmResponses = rawLlmResponses as Array<{ kind: string; response: string }>;
        } else {
          return errorResult(
            `Invalid _llm_responses format. Expected a JSON array or string, got ${typeof rawLlmResponses}.`,
          );
        }
      }
      if (llmResponses) {
        return await processAgentResponses(
          llmResponses, params, inputFile, outputFile, getCurrentWorkingDir(),
        );
      }

      // Require at least one inference option (only for new requests, not _llm_responses callbacks).
      // Normalize keys the same way buildDescribegptArgs does: strip leading --, convert - to _.
      const inferenceOptions = new Set(["dictionary", "description", "tags", "all"]);
      const hasInferenceOption = Object.entries(params).some(([rawKey, value]) => {
        const normalized = rawKey.replace(/^--/, "").replace(/-/g, "_");
        return inferenceOptions.has(normalized) && value === true;
      });
      if (!hasInferenceOption) {
        return errorResult(
          `describegpt requires at least one inference option: --dictionary, --description, --tags, or --all.\n\n` +
          `Example: qsv_describegpt(input_file="data.csv", all=true)`,
        );
      }

      // Auto-generate output file if not specified.
      // describegpt output (data dictionaries, descriptions, tags) should always persist to a file.
      // Default format is Markdown, so use .md extension. Place alongside the input file.
      if (!outputFile && inputFile) {
        const inputBasename = basename(inputFile, extname(inputFile));
        const inputDir = dirname(inputFile);
        outputFile = join(inputDir, `${inputBasename}.describegpt.md`);
      }

      const capabilities = server.getClientCapabilities();
      if (capabilities?.sampling) {
        // Build original CLI args from the resolved params
        const cliArgs = buildDescribegptArgs(params, inputFile, outputFile);
        return await executeDescribegptWithSampling(
          server,
          config.qsvBinPath,
          cliArgs,
          getCurrentWorkingDir(),
        );
      }

      // No sampling available — return prompts for agent-as-LLM fallback
      return await prepareContextForAgent(params, inputFile, outputFile, getCurrentWorkingDir());
    }

    // Execute the skill
    const result = await executor.execute(skill, { args, options });

    // Auto-run cheap moarstats (without --advanced or --bivariate) after successful stats execution
    // to enrich the .stats.csv cache with ~18 additional columns at minimal cost.
    // Note: moarstats overwrites the stats CSV in-place by default (no --output needed).
    // This only triggers for commandName === "stats", so moarstats itself won't cause recursion.
    let moarstatsNote = "";
    if (commandName === "stats" && result.success && inputFile && !isHelpRequest) {
      try {
        const moarstatsSkill = await loader.load("qsv-moarstats");
        if (moarstatsSkill) {
          console.error(`[MCP Tools] Auto-running moarstats to enrich stats cache`);
          const moarstatsResult = await executor.execute(moarstatsSkill, {
            args: { input: inputFile },
            options: {},
          });
          if (moarstatsResult.success) {
            const duration = moarstatsResult.metadata?.duration ?? "?";
            moarstatsNote = `\n\n📊 Auto-enriched stats cache with moarstats (~18 additional columns, ${duration}ms)`;
            console.error(`[MCP Tools] moarstats auto-enrichment succeeded (${duration}ms)`);
          } else {
            console.error(`[MCP Tools] moarstats auto-enrichment failed: ${moarstatsResult.stderr}`);
          }
        }
      } catch (error: unknown) {
        console.error(`[MCP Tools] moarstats auto-enrichment error:`, getErrorMessage(error));
      }
    }

    // Format and return result
    if (result.success) {
      const formattedResult = await formatToolResult(
        result,
        commandName,
        inputFile,
        outputFile,
        autoCreatedTempFile,
        params,
      );
      // Find the first text content element for prepending/appending notes.
      // Note: find() returns a reference, so mutating textContent.text
      // modifies the element inside formattedResult.content in-place.
      const textContent = formattedResult.content?.find(
        (c: { type: string }) => c.type === "text",
      ) as { type: "text"; text: string } | undefined;

      // Prepend Parquet conversion warning if any
      if (parquetConversionWarning) {
        if (textContent) {
          textContent.text = parquetConversionWarning + "\n\n" + textContent.text;
        } else {
          console.error(`[MCP Tools] Could not prepend Parquet warning to result: unexpected content structure`);
        }
      }

      // Prepend Polars SQL engine header for sqlp results
      // (after parquet warning so final order is: engine header → warning → output,
      // consistent with the error path)
      if (commandName === "sqlp" && !isHelpRequest) {
        if (textContent) {
          textContent.text = "🐻‍❄️ Engine: Polars SQL\n\n" + textContent.text;
        }
      }
      // Append moarstats auto-enrichment note if applicable
      if (moarstatsNote) {
        if (textContent) {
          textContent.text += moarstatsNote;
        } else {
          console.error(`[MCP Tools] Could not append moarstats note to result: unexpected content structure`);
        }
      }
      // Attach pipeline metadata for reproducibility manifest.
      // Note: formatToolResult can return isError (e.g., temp file I/O failure)
      // even though the qsv command itself succeeded — derive success from the result.
      const finalOut = (formattedResult as Record<string | symbol, unknown>)[FINAL_OUTPUT_FILE] as string | undefined;
      const formatSuccess = !("isError" in formattedResult && formattedResult.isError === true);
      (formattedResult as Record<string | symbol, unknown>)[PIPELINE_METADATA] = {
        inputFile,
        outputFile: finalOut,
        commandLine: result.metadata?.command,
        durationMs: result.metadata?.duration,
        success: formatSuccess,
        additionalInputFiles: collectAdditionalInputFiles(skill, params),
      } satisfies PipelineMetadata;
      return formattedResult;
    } else {
      const cmdLine = result.metadata?.command ? `\nCommand: ${result.metadata.command}` : "";
      const stderr = result.stderr.trimEnd();
      const engineHeader = commandName === "sqlp" && !isHelpRequest ? "🐻‍❄️ Engine: Polars SQL\n\n" : "";
      const errorMsg = parquetConversionWarning
        ? `${engineHeader}${parquetConversionWarning}\n\nError executing ${commandName}:\n${stderr}${cmdLine}`
        : `${engineHeader}Error executing ${commandName}:\n${stderr}${cmdLine}`;
      const errResult = errorResult(errorMsg);
      // Attach pipeline metadata even for failures
      (errResult as Record<string | symbol, unknown>)[PIPELINE_METADATA] = {
        inputFile,
        outputFile,
        commandLine: result.metadata?.command,
        durationMs: result.metadata?.duration,
        success: false,
        additionalInputFiles: collectAdditionalInputFiles(skill, params),
      } satisfies PipelineMetadata;
      return errResult;
    }
  } catch (error: unknown) {
    return errorResult(`Unexpected error: ${getErrorMessage(error)}`);
  } finally {
    releaseSlot();
  }
}

/**
 * Handle execution of the generic qsv_command tool
 */
export async function handleGenericCommand(
  params: Record<string, unknown>,
  executor: SkillExecutor,
  loader: SkillLoader,
  filesystemProvider?: FilesystemProviderExtended,
  server?: Server,
) {
  try {
    const commandName = params.command as string | undefined;

    if (!commandName) {
      return errorResult("Error: command parameter is required");
    }

    // Log the command for observability — handleToolCall validates against loaded skills
    const skillName = `qsv-${commandName}`;
    const skill = await loader.load(skillName);
    if (!skill) {
      console.warn(`[handleGenericCommand] Unrecognized command: ${commandName} (no skill '${skillName}' found)`);
    }

    // Flatten nested args and options objects into the params
    // This handles cases where Claude passes:
    // {"command": "luau", "args": {...}, "options": {...}, "input_file": "...", "output_file": "..."}
    const flattenedParams: Record<string, unknown> = {};

    // Copy top-level params (except 'args' and 'options')
    for (const [key, value] of Object.entries(params)) {
      if (key !== "args" && key !== "options") {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'args' object
    if (params.args && typeof params.args === "object") {
      const argsObj = params.args as Record<string, unknown>;
      for (const [key, value] of Object.entries(argsObj)) {
        flattenedParams[key] = value;
      }
    }

    // Flatten nested 'options' object
    if (params.options && typeof params.options === "object") {
      const optionsObj = params.options as Record<string, unknown>;
      for (const [key, value] of Object.entries(optionsObj)) {
        flattenedParams[key] = value;
      }
    }

    console.error(
      `[handleGenericCommand] Flattened params:`,
      JSON.stringify(flattenedParams),
    );

    // Forward to handleToolCall with the qsv_ prefix and flattened params
    return await handleToolCall(
      `qsv_${commandName}`,
      flattenedParams,
      executor,
      loader,
      filesystemProvider,
      server,
    );
  } catch (error: unknown) {
    return errorResult(`Unexpected error: ${getErrorMessage(error)}`);
  }
}

/**
 * Handle qsv_config tool call
 */
export async function handleConfigTool(
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const validation = config.qsvValidation;
  const extensionMode = config.isExtensionMode;

  let configText = `# qsv Configuration\n\n`;

  // qsv Binary Information
  configText += `## qsv Binary\n\n`;
  if (validation.valid) {
    configText += `✅ **Status:** Validated\n`;
    configText += `📍 **Path:** \`${validation.path}\`\n`;
    configText += `🏷️ **Version:** ${validation.version}\n`;
    if (validation.commandCount) {
      configText += `🔧 **Available Commands:** ${validation.commandCount}\n`;
    }
    if (validation.totalMemory) {
      configText += `💾 **System Total Memory:** ${validation.totalMemory}\n`;
    }
  } else {
    configText += `❌ **Status:** Validation Failed\n`;
    configText += `⚠️ **Error:** ${validation.error}\n`;

    // Show auto-detection diagnostics
    const diagnostics = getDetectionDiagnostics();
    if (diagnostics.whichAttempted) {
      configText += `\n### 🔍 Auto-Detection Diagnostics\n\n`;

      // Show which/where attempt
      configText += `**PATH search (which/where):**\n`;
      if (diagnostics.whichResult) {
        configText += `✅ Found: \`${diagnostics.whichResult}\`\n\n`;
      } else if (diagnostics.whichError) {
        configText += `❌ Failed: ${diagnostics.whichError}\n\n`;
      } else {
        configText += `❌ Not found in PATH\n\n`;
      }

      // Show common locations checked
      if (diagnostics.locationsChecked.length > 0) {
        configText += `**Common locations checked:**\n\n`;
        diagnostics.locationsChecked.forEach((loc) => {
          configText += `- \`${loc.path}\`\n`;
          if (loc.exists) {
            configText += `  - ✅ File exists\n`;
            if (loc.isFile !== undefined) {
              configText += `  - ${loc.isFile ? "✅" : "❌"} Is regular file: ${loc.isFile}\n`;
            }
            if (loc.executable !== undefined) {
              configText += `  - ${loc.executable ? "✅" : "❌"} Executable: ${loc.executable}\n`;
            }
            if (loc.version) {
              configText += `  - ✅ Version: ${loc.version}\n`;
            }
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          } else {
            configText += `  - ❌ Does not exist\n`;
            if (loc.error) {
              configText += `  - ⚠️ Error: ${loc.error}\n`;
            }
          }
        });
        configText += `\n`;
      }
    }
  }

  // DuckDB Information
  configText += `\n## DuckDB\n\n`;
  const duckDbStatus = getDuckDbStatus();
  if (!config.useDuckDb) {
    configText += `⏸️ **Status:** Disabled (QSV_MCP_USE_DUCKDB=false)\n`;
  } else if (duckDbStatus.status === "available") {
    configText += `✅ **Status:** Available\n`;
    configText += `📍 **Path:** \`${duckDbStatus.binPath}\`\n`;
    configText += `🏷️ **Version:** ${duckDbStatus.version}\n`;
    configText += `ℹ️ SQL queries are routed through DuckDB for better compatibility and performance.\n`;
  } else if (duckDbStatus.status === "unavailable") {
    configText += `❌ **Status:** Unavailable\n`;
    configText += `⚠️ **Reason:** ${duckDbStatus.reason}\n`;
    configText += `ℹ️ SQL queries use Polars SQL (sqlp) as fallback.\n`;
  } else {
    configText += `⏳ **Status:** Pending (detected on first SQL query)\n`;
    configText += `ℹ️ DuckDB will be auto-detected when the first SQL query runs.\n`;
  }

  // Working Directory
  configText += `\n## Working Directory\n\n`;
  if (filesystemProvider) {
    const workingDir = filesystemProvider.getWorkingDirectory();
    configText += `📁 **Current:** \`${workingDir}\`\n`;
  } else {
    configText += `📁 **Current:** \`${config.workingDir}\`\n`;
  }

  // Allowed Directories
  configText += `\n## Allowed Directories\n\n`;
  if (config.allowedDirs.length > 0) {
    configText += `🔓 **Access granted to:**\n`;
    config.allowedDirs.forEach((dir) => {
      configText += `   - \`${dir}\`\n`;
    });
  } else {
    configText += `ℹ️ Only working directory is accessible\n`;
  }
  if (config.isPluginMode) {
    configText += `\n📌 _Plugin mode: additional directories are auto-added as needed at runtime._\n`;
  }

  // Performance Settings
  configText += `\n## Performance Settings\n\n`;
  configText += `⏱️ **Timeout:** ${config.operationTimeoutMs}ms (${Math.round(config.operationTimeoutMs / 1000)}s)\n`;
  configText += `💾 **Max Output Size:** ${formatBytes(config.maxOutputSize)}\n`;
  configText += `🔧 **Auto-Regenerate Skills:** ${config.autoRegenerateSkills ? "Enabled" : "Disabled"}\n`;
  configText += `📄 **Output Format:** ${config.outputFormat.toUpperCase()}\n`;

  // Update Check Settings
  configText += `\n## Update Settings\n\n`;
  configText += `🔍 **Check Updates on Startup:** ${config.checkUpdatesOnStartup ? "Enabled" : "Disabled"}\n`;
  configText += `📢 **Update Notifications:** ${config.notifyUpdates ? "Enabled" : "Disabled"}\n`;

  // Mode
  configText += `\n## Deployment Mode\n\n`;
  if (config.isPluginMode) {
    configText += `🔌 **Claude Plugin Mode** (relaxed directory security)\n`;
  } else if (extensionMode) {
    configText += `🧩 **Desktop Extension Mode**\n`;
  } else {
    configText += `🖥️ **Legacy MCP Server Mode**\n`;
  }

  // Help Text
  configText += `\n---\n\n`;
  if (!validation.valid) {
    configText += `### ⚠️ Action Required\n\n`;
    if (extensionMode) {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Open Claude Desktop Settings > Extensions > qsv\n`;
      configText += `3. Update "qsv Binary Path" or ensure qsv is in your system PATH\n`;
      configText += `4. Save settings (extension will auto-restart)\n`;
    } else {
      configText += `To fix the qsv binary issue:\n`;
      configText += `1. Install qsv from https://github.com/dathere/qsv#installation\n`;
      configText += `2. Ensure qsv is in your PATH or set QSV_MCP_BIN_PATH\n`;
      configText += `3. Restart the MCP server\n`;
    }
  } else {
    configText += `### 💡 Tip\n\n`;
    configText += `These are the actual resolved values used by the server. The configuration UI may show template variables like \`\${HOME}/Downloads\` which get expanded to the paths shown above.\n`;
  }

  return successResult(configText);
}

/**
 * Handle qsv_search_tools call
 * Searches loaded skills and returns matching tools
 * Marks found tools as loaded for deferred loading
 *
 * @param params - Search parameters (query, category, limit)
 * @param loader - SkillLoader instance for searching skills
 * @param loadedTools - Optional Set to track loaded tools for deferred loading
 */
export async function handleSearchToolsCall(
  params: Record<string, unknown>,
  loader: SkillLoader,
  loadedTools?: Set<string>,
): Promise<{ content: Array<{ type: string; text: string }> }> {
  const rawQuery = params.query;
  const query = typeof rawQuery === "string" ? rawQuery : rawQuery != null ? String(rawQuery) : "";
  const category = params.category as string | undefined;
  const limit = Math.min(Math.max(1, (params.limit as number) || 5), 20);

  if (!query || query.trim().length === 0) {
    return {
      content: [
        {
          type: "text",
          text: "Error: query parameter is required",
        },
      ],
    };
  }

  // Ensure all skills are loaded
  await loader.loadAll();

  // Search using the loader's search method
  let results = loader.search(query);

  // Apply category filter if specified
  if (category) {
    results = results.filter((skill) => skill.category === category);
  }

  // Also check if query matches category name (for discovery)
  const queryLower = query.toLowerCase();
  const matchedCategory = SKILL_CATEGORIES.find((cat) => queryLower.includes(cat));

  if (matchedCategory && !category) {
    // Add skills from matching category that weren't already found
    const categorySkills = loader.getByCategory(matchedCategory as SkillCategory);
    const existingNames = new Set(results.map((r) => r.name));
    for (const skill of categorySkills) {
      if (!existingNames.has(skill.name)) {
        results.push(skill);
      }
    }
  }

  // Try regex matching if query looks like a regex pattern
  const isRegexPattern = query.startsWith("/") && query.endsWith("/");
  if (isRegexPattern) {
    try {
      const regexStr = query.slice(1, -1);
      const regex = new RegExp(regexStr, "i");
      const allSkills = loader.getAll();

      results = allSkills.filter(
        (skill) =>
          regex.test(skill.name) ||
          regex.test(skill.description) ||
          regex.test(skill.command.subcommand) ||
          skill.examples?.some((ex) => regex.test(ex.description)),
      );
    } catch (regexError) {
      // Invalid regex, fall back to text search (already done above)
    }
  }

  // Sort by relevance (exact name match first, then description match)
  results.sort((a, b) => {
    const aNameMatch = a.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const bNameMatch = b.name.toLowerCase().includes(queryLower) ? 1 : 0;
    const aCommandMatch = a.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;
    const bCommandMatch = b.command.subcommand
      .toLowerCase()
      .includes(queryLower)
      ? 1
      : 0;

    const aScore = aNameMatch * 2 + aCommandMatch * 2;
    const bScore = bNameMatch * 2 + bCommandMatch * 2;

    return bScore - aScore;
  });

  // Limit results
  const limitedResults = results.slice(0, limit);

  // Mark found tools as loaded for deferred loading
  // This allows them to appear in subsequent ListTools responses
  if (loadedTools) {
    for (const skill of limitedResults) {
      const toolName = skill.name.replace("qsv-", "qsv_");
      loadedTools.add(toolName);
    }
    console.error(
      `[MCP Tools] Marked ${limitedResults.length} tools as loaded for deferred loading`,
    );
  }

  if (limitedResults.length === 0) {
    // Provide helpful suggestions
    const allCategories = loader.getCategories();
    const totalSkills = loader.getStats().total;

    return {
      content: [
        {
          type: "text",
          text:
            `No tools found matching "${query}".\n\n` +
            `Try:\n` +
            `- Different keywords (e.g., "filter", "join", "sort", "stats")\n` +
            `- Category filter: ${allCategories.join(", ")}\n` +
            `- Regex pattern: /pattern/\n\n` +
            `Total available tools: ${totalSkills}`,
        },
      ],
    };
  }

  // Format results as tool references
  let resultText = `Found ${results.length} tool${results.length !== 1 ? "s" : ""} matching "${query}"`;
  if (category) {
    resultText += ` in category "${category}"`;
  }
  if (results.length > limit) {
    resultText += ` (showing top ${limit})`;
  }
  resultText += ":\n\n";

  for (const skill of limitedResults) {
    const toolName = skill.name.replace("qsv-", "qsv_");
    // Truncate description to first sentence for conciseness
    let shortDesc = skill.description.split(".")[0];
    if (shortDesc.length > 100) {
      shortDesc = shortDesc.substring(0, 97) + "...";
    }

    // Get when-to-use guidance if available
    const whenToUse = COMMAND_GUIDANCE[skill.command.subcommand]?.whenToUse;

    resultText += `**${toolName}** [${skill.category}]\n`;
    resultText += `  ${shortDesc}\n`;
    if (whenToUse) {
      resultText += `  💡 ${whenToUse}\n`;
    }
    resultText += "\n";
  }

  // Add tip for using the tools
  resultText += `---\n`;
  resultText += `💡 To use a tool, call it directly: e.g., \`qsv_${limitedResults[0].command.subcommand}\` with \`input_file\` parameter.\n`;
  resultText += `📖 For detailed help on any command, use \`help: true\` parameter.`;

  return {
    content: [
      {
        type: "text",
        text: resultText,
      },
    ],
  };
}

/**
 * Handle qsv_to_parquet tool call
 * Converts CSV to Parquet using DuckDB (primary) or `qsv to parquet` (fallback)
 */
export async function handleToParquetCall(
  params: Record<string, unknown>,
  filesystemProvider?: FilesystemProviderExtended,
): Promise<{
  content: Array<{ type: string; text: string }>;
  isError?: boolean;
}> {
  // Extract input_file and output_file (with LLM alias resolution)
  const { inputFile: rawInputFile, outputFile: rawOutputFile } = resolveParamAliases(params);
  let inputFile = rawInputFile;
  let outputFile = rawOutputFile;

  if (!inputFile) {
    return errorResult("Error: input_file parameter is required");
  }

  // Resolve input file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalInputFile = inputFile;
      inputFile = await filesystemProvider.resolvePath(inputFile);
      console.error(
        `[MCP Tools] Resolved input file: ${originalInputFile} -> ${inputFile}`,
      );
    } catch (error: unknown) {
      return errorResult(`Error resolving input file path: ${getErrorMessage(error)}`);
    }
  }

  // Generate output path if not provided
  if (!outputFile) {
    // Replace common CSV-like extensions with .parquet, or append .parquet if none match
    const lowerInput = inputFile.toLowerCase();
    // Supported CSV-like extensions, including snappy-compressed variants
    const csvLikeExtensions = [
      ".csv.sz",
      ".tsv.sz",
      ".tab.sz",
      ".ssv.sz",
      ".csv",
      ".tsv",
      ".tab",
      ".ssv",
    ];

    let matched = false;
    for (const ext of csvLikeExtensions) {
      if (lowerInput.endsWith(ext)) {
        outputFile = inputFile.slice(0, -ext.length) + ".parquet";
        matched = true;
        break;
      }
    }

    if (!matched) {
      outputFile = inputFile + ".parquet";
    }
  }

  // At this point outputFile is guaranteed to be defined (either provided or generated)
  let resolvedOutputFile: string = outputFile as string;

  // Resolve output file path using filesystem provider if available
  if (filesystemProvider) {
    try {
      const originalOutputFile = resolvedOutputFile;
      resolvedOutputFile =
        await filesystemProvider.resolvePath(resolvedOutputFile);
      console.error(
        `[MCP Tools] Resolved output file: ${originalOutputFile} -> ${resolvedOutputFile}`,
      );
    } catch (error: unknown) {
      return errorResult(`Error resolving output file path: ${getErrorMessage(error)}`);
    }
  }

  // Prevent overwriting reserved cache files
  if (isReservedCachePath(resolvedOutputFile)) {
    return errorResult(reservedCachePathError(resolvedOutputFile));
  }

  const startTime = Date.now();

  try {
    // Step 1: Ensure stats cache is up-to-date (needed by both DuckDB and qsv to parquet paths)
    const { needStats, statsFile } = await ensureStatsCache(inputFile);

    // Step 3: Convert to Parquet (DuckDB with ZSTD when available, qsv to parquet with ZSTD otherwise)
    // Schema generation (Steps 2-2.5) is deferred to the qsv to parquet fallback path only
    const { engine, needSchema, schemaFile, schemaSkipped, outputPath } = await convertCsvToParquet(inputFile, resolvedOutputFile, statsFile);
    const duration = Date.now() - startTime;

    // Get output file size for reporting — use outputPath (actual file written)
    let fileSizeInfo = "";
    try {
      const outputStats = await stat(outputPath);
      fileSizeInfo = ` (${formatBytes(outputStats.size)})`;
    } catch (error: unknown) {
      console.warn(`[MCP Tools] Could not stat output file for size reporting: ${outputPath}`, error);
    }

    const statsStatus = needStats ? "generated" : "reused (up-to-date)";
    const schemaStatus = schemaSkipped ? "skipped (DuckDB)" : needSchema ? "generated" : "reused (up-to-date)";

    return successResult(
      `✅ Successfully converted CSV to Parquet with optimized schema\n\n` +
      `Input: ${inputFile}\n` +
      `Output: ${outputPath}${fileSizeInfo}\n` +
      `Engine: ${engine}\n` +
      `Stats: ${statsFile}\n` +
      `Schema: ${schemaFile}\n` +
      `Duration: ${duration}ms\n\n` +
      `Stats cache: ${statsStatus}\n` +
      `Polars schema: ${schemaStatus}\n` +
      `The Parquet file is now ready for fast SQL queries.\n` +
      (getDuckDbStatus().status === "available"
        ? `🦆 DuckDB detected — qsv_sqlp will auto-route SQL queries through DuckDB for this file.`
        : `Use: qsv_sqlp with input_file="SKIP_INPUT" and sql="SELECT ... FROM read_parquet('${outputPath}')".`),
    );
  } catch (error: unknown) {
    return errorResult(`Error converting CSV to Parquet: ${getErrorMessage(error)}`);
  }
}

/**
 * Handle a qsv_log tool invocation.
 *
 * Writes a `u-` prefixed entry to the qsv audit log via `qsv log`.
 * Logging failures are swallowed — this tool should never break a workflow.
 */
export async function handleLogCall(
  params: Record<string, unknown>,
  workingDir: string,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
  // Validate required params before coercing
  if (params.entry_type == null) {
    return errorResult("entry_type is required.");
  }
  if (params.message == null) {
    return errorResult("message is required.");
  }

  const entryType = String(params.entry_type);
  const rawMessage = String(params.message);

  // Validate entry_type
  if (!LOG_ENTRY_TYPES.has(entryType)) {
    return errorResult(
      `Invalid entry_type "${entryType}". Must be one of: ${[...LOG_ENTRY_TYPES].join(", ")}`,
    );
  }

  // Validate message
  if (rawMessage.trim().length === 0) {
    return errorResult("message must be a non-empty string.");
  }

  // Trim, strip newlines, and truncate if needed (use Array.from for Unicode-safe truncation)
  const sanitized = rawMessage.trim().replace(/[\r\n]+/g, " ");
  // Fast path: if UTF-16 length is within limit, codepoint count is too
  let message: string;
  if (sanitized.length <= MAX_LOG_MESSAGE_LEN) {
    message = sanitized;
  } else {
    const codepoints = Array.from(sanitized);
    message =
      codepoints.length > MAX_LOG_MESSAGE_LEN
        ? codepoints.slice(0, MAX_LOG_MESSAGE_LEN).join("")
        : sanitized;
  }

  const logId = `u-${randomUUID()}`;

  try {
    await runQsvSimple(config.qsvBinPath, [
      "log",
      "qsv_log",
      logId,
      `[${entryType}] ${message}`,
    ], {
      timeoutMs: 5_000,
      cwd: workingDir,
    });
  } catch (err) {
    const errMsg = getErrorMessage(err);
    console.error(`[qsv_log] write failed: ${errMsg}`);
    return successResult(`Log write failed (non-fatal): ${errMsg.slice(0, 100)}. Workflow continues.`);
  }

  return successResult(`Logged ${entryType} entry.`);
}

// ─── qsv_setup tool ─────────────────────────────────────────────────────────

/**
 * Result returned from handleSetupCall
 */
export interface SetupCallResult {
  revalidated: boolean;
  response: { content: Array<{ type: string; text: string }>; isError?: boolean };
}

/**
 * Handle the qsv_setup tool call.
 *
 * @param args - Tool arguments (must include confirm: true)
 * @param revalidateCallback - Called after successful install to refresh config.
 *   Returns the revalidation result so the handler can check if the binary is now valid.
 */
export async function handleSetupCall(
  args: Record<string, unknown>,
  revalidateCallback: () => { path: string; validation: { valid: boolean; version?: string; error?: string } },
): Promise<SetupCallResult> {
  // Dynamic import to avoid loading installer.ts at startup
  const { installQsv } = await import("./installer.js");

  if (args.confirm !== true) {
    return {
      revalidated: false,
      response: errorResult(
        "Installation requires explicit confirmation. Call qsv_setup with confirm=true to proceed.",
      ),
    };
  }

  const result = await installQsv();

  // Manual instructions — not an error, just guidance
  if (result.method === "manual" && result.instructions) {
    return {
      revalidated: false,
      response: successResult(
        `No supported package manager found for automatic installation.\n\n${result.instructions}`,
      ),
    };
  }

  if (!result.success) {
    return {
      revalidated: false,
      response: errorResult(
        `Installation via ${result.method} failed:\n${result.error}\n\n` +
        `You can try installing manually from https://github.com/dathere/qsv/releases/latest`,
      ),
    };
  }

  // Install succeeded — revalidate the binary
  const revalidation = revalidateCallback();

  if (revalidation.validation.valid) {
    const paths = result.binaryPaths;
    const binaryInfo = paths
      ? (paths.qsv
        ? `Binaries: ${paths.qsvmcp} (MCP server)\n         ${paths.qsv} (CLI)`
        : `Binary: ${paths.qsvmcp}`)
      : `Binary: ${revalidation.path}`;
    return {
      revalidated: true,
      response: successResult(
        `qsv installed successfully via ${result.method}!\n\n` +
        `${binaryInfo}\n` +
        `Version: ${revalidation.validation.version}\n\n` +
        `${paths?.qsv ? 'The full qsv toolkit is now available.' : 'The qsv MCP server is now available.'} You can proceed with data tasks.`,
      ),
    };
  }

  // Installed but revalidation failed (version too old, missing Polars, etc.)
  return {
    revalidated: false,
    response: errorResult(
      `qsv was installed via ${result.method}, but validation failed:\n` +
      `${revalidation.validation.error}\n\n` +
      `The installed version may not meet the minimum requirements. ` +
      `Please install qsv >= ${MINIMUM_QSV_VERSION} with Polars support ` +
      `from https://github.com/dathere/qsv#installation`,
    ),
  };
}
