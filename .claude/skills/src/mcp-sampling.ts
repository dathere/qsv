/**
 * MCP Sampling integration for describegpt.
 *
 * Orchestrates two-phase describegpt execution:
 *   Phase 1: `qsv describegpt --prepare-context` → JSON with prompts & cache state
 *   Phase 2: For uncached phases, call `server.createMessage()` (MCP sampling)
 *   Phase 3: `qsv describegpt --process-response` ← JSON with LLM responses via stdin
 */

import type { Server } from "@modelcontextprotocol/sdk/server/index.js";
import type { CreateMessageResult } from "@modelcontextprotocol/sdk/types.js";
import { spawn } from "child_process";
import { errorResult, successResult } from "./utils.js";

/** Phase context from --prepare-context output */
interface PhaseContext {
  kind: string;
  system_prompt: string;
  user_prompt: string;
  max_tokens: number;
  cache_key: string;
  cached_response: {
    response: string;
    reasoning: string;
    token_usage: { prompt: number; completion: number; total: number; elapsed: number };
  } | null;
}

/** Full output from --prepare-context */
interface PrepareContextOutput {
  phases: PhaseContext[];
  analysis_results: unknown;
  model: string;
  max_tokens: number;
}

/** Phase response for --process-response input */
interface PhaseResponse {
  kind: string;
  response: string;
  reasoning: string;
  token_usage: { prompt: number; completion: number; total: number; elapsed: number };
}

/** Input for --process-response */
interface ProcessResponseInput {
  phases: PhaseResponse[];
  analysis_results: unknown;
  model: string;
}

/**
 * Run a qsv command and capture stdout, with optional stdin data.
 */
function runQsvCapture(
  binPath: string,
  args: string[],
  options?: { cwd?: string; stdinData?: string; timeoutMs?: number },
): Promise<{ stdout: string; stderr: string; exitCode: number }> {
  const timeoutMs = options?.timeoutMs ?? 600_000;

  return new Promise((resolve, reject) => {
    const proc = spawn(binPath, args, {
      stdio: ["pipe", "pipe", "pipe"],
      cwd: options?.cwd,
    });

    let stdout = "";
    let stderr = "";
    let finalized = false;

    const finalize = (err?: Error, code?: number | null) => {
      if (finalized) return;
      finalized = true;
      clearTimeout(timer);
      if (err) {
        reject(err);
      } else {
        resolve({ stdout, stderr, exitCode: code ?? 1 });
      }
    };

    const timer = setTimeout(() => {
      proc.kill("SIGTERM");
      finalize(new Error(`qsv describegpt timed out after ${timeoutMs}ms`));
    }, timeoutMs);

    proc.stdout!.on("data", (chunk) => { stdout += chunk.toString(); });
    proc.stderr!.on("data", (chunk) => { stderr += chunk.toString(); });
    proc.on("close", (code) => finalize(undefined, code));
    proc.on("error", (err) => finalize(err));

    // Write stdin data if provided, then close stdin
    if (options?.stdinData) {
      proc.stdin!.write(options.stdinData, () => {
        proc.stdin!.end();
      });
    } else {
      proc.stdin!.end();
    }
  });
}

/**
 * Execute describegpt with MCP sampling.
 *
 * 1. Run `qsv describegpt --prepare-context` to get prompts and cache state
 * 2. For each uncached phase, call `server.createMessage()` for LLM inference
 * 3. Run `qsv describegpt --process-response` with responses via stdin
 */
export async function executeDescribegptWithSampling(
  server: Server,
  binPath: string,
  originalArgs: string[],
  workingDir: string,
): Promise<{ content: Array<{ type: string; text: string }>; isError?: boolean }> {
  // Phase 1: Get prompt context
  const prepareArgs = [...originalArgs, "--prepare-context"];
  const phase1 = await runQsvCapture(binPath, ["describegpt", ...prepareArgs], {
    cwd: workingDir,
    timeoutMs: 300_000,
  });

  if (phase1.exitCode !== 0) {
    return errorResult(
      `describegpt --prepare-context failed (exit ${phase1.exitCode}):\n${phase1.stderr}`,
    );
  }

  let prepareOutput: PrepareContextOutput;
  try {
    prepareOutput = JSON.parse(phase1.stdout);
  } catch {
    return errorResult(
      `Failed to parse --prepare-context JSON output:\n${phase1.stdout.slice(0, 500)}`,
    );
  }

  // Phase 2: Call createMessage for uncached phases
  const phaseResponses: PhaseResponse[] = [];

  for (const phase of prepareOutput.phases) {
    if (phase.cached_response) {
      // Cache hit — use cached response directly
      phaseResponses.push({
        kind: phase.kind,
        response: phase.cached_response.response,
        reasoning: phase.cached_response.reasoning,
        token_usage: phase.cached_response.token_usage,
      });
      continue;
    }

    // Cache miss — use MCP sampling
    try {
      const samplingResult: CreateMessageResult = await server.createMessage({
        messages: [
          {
            role: "user",
            content: { type: "text", text: phase.user_prompt },
          },
        ],
        systemPrompt: phase.system_prompt,
        maxTokens: phase.max_tokens || 10000,
      });

      // Extract text from the sampling result
      let responseText = "";
      if (samplingResult.content && typeof samplingResult.content === "object" && "text" in samplingResult.content) {
        responseText = (samplingResult.content as { text: string }).text;
      }

      phaseResponses.push({
        kind: phase.kind,
        response: responseText,
        reasoning: "",
        token_usage: { prompt: 0, completion: 0, total: 0, elapsed: 0 },
      });
    } catch (error: unknown) {
      const message = error instanceof Error ? error.message : String(error);
      return errorResult(
        `MCP sampling failed for phase "${phase.kind}": ${message}`,
      );
    }
  }

  // Phase 3: Process responses
  const processInput: ProcessResponseInput = {
    phases: phaseResponses,
    analysis_results: prepareOutput.analysis_results,
    model: prepareOutput.model,
  };

  const processArgs = [...originalArgs, "--process-response"];
  const phase3 = await runQsvCapture(binPath, ["describegpt", ...processArgs], {
    cwd: workingDir,
    stdinData: JSON.stringify(processInput),
    timeoutMs: 300_000,
  });

  if (phase3.exitCode !== 0) {
    return errorResult(
      `describegpt --process-response failed (exit ${phase3.exitCode}):\n${phase3.stderr}`,
    );
  }

  return successResult(phase3.stdout);
}
