/**
 * Tests for tool-handlers helpers.
 *
 * Covers the pure validators extracted out of handleToolCall during the
 * command-interceptor refactor. The stateful interceptors
 * (runSqlpParquetInterception, runDescribegptInterception,
 * runMoarstatsAutoEnrich) require live qsv/duckdb binaries and are exercised
 * via qsv-integration.test.ts and mcp-tools.test.ts.
 */

import { test } from "node:test";
import assert from "node:assert";
import { validateLlmResponsesShape } from "../src/tool-handlers.js";

// ─────────────────────────────────────────────────────────────────────────────
// validateLlmResponsesShape
// ─────────────────────────────────────────────────────────────────────────────

test("validateLlmResponsesShape accepts an empty array", () => {
  assert.strictEqual(validateLlmResponsesShape([]), null);
});

test("validateLlmResponsesShape accepts well-formed entries", () => {
  const ok = [
    { kind: "dictionary", response: "col1: int\ncol2: str" },
    { kind: "description", response: "A small dataset of widgets." },
    { kind: "tags", response: "widgets,inventory" },
  ];
  assert.strictEqual(validateLlmResponsesShape(ok), null);
});

test("validateLlmResponsesShape rejects null elements with a precise message", () => {
  const out = validateLlmResponsesShape([null]);
  assert.match(out ?? "", /_llm_responses\[0\] must be an object/);
  assert.match(out ?? "", /got null/);
});

test("validateLlmResponsesShape rejects nested arrays as elements", () => {
  const out = validateLlmResponsesShape([["kind", "response"]]);
  assert.match(out ?? "", /_llm_responses\[0\]/);
  assert.match(out ?? "", /got array/);
});

test("validateLlmResponsesShape rejects primitive elements", () => {
  const numberOut = validateLlmResponsesShape([42]);
  assert.match(numberOut ?? "", /_llm_responses\[0\]/);
  assert.match(numberOut ?? "", /got number/);

  const stringOut = validateLlmResponsesShape(["dictionary"]);
  assert.match(stringOut ?? "", /_llm_responses\[0\]/);
  assert.match(stringOut ?? "", /got string/);
});

test("validateLlmResponsesShape requires both kind and response as strings", () => {
  // Missing 'response'
  const missing = validateLlmResponsesShape([{ kind: "dictionary" }]);
  assert.match(missing ?? "", /must have "kind" and "response" string fields/);

  // 'kind' wrong type
  const wrongKind = validateLlmResponsesShape([{ kind: 42, response: "ok" }]);
  assert.match(wrongKind ?? "", /must have "kind" and "response" string fields/);

  // 'response' wrong type
  const wrongResp = validateLlmResponsesShape([{ kind: "tags", response: 99 }]);
  assert.match(wrongResp ?? "", /must have "kind" and "response" string fields/);
});

test("validateLlmResponsesShape reports the index of the first bad element", () => {
  const arr = [
    { kind: "dictionary", response: "ok" },
    { kind: "tags", response: "ok" },
    { kind: 0 as unknown as string, response: "broken" },
    { kind: "description", response: "ok" },
  ];
  const out = validateLlmResponsesShape(arr);
  assert.match(out ?? "", /_llm_responses\[2\]/);
});

test("validateLlmResponsesShape accepts empty strings (only types matter)", () => {
  // Empty kind/response is structurally valid — the validator only checks
  // shape, not semantic emptiness. Semantic checks belong downstream.
  const out = validateLlmResponsesShape([{ kind: "", response: "" }]);
  assert.strictEqual(out, null);
});

test("validateLlmResponsesShape tolerates extra keys on elements", () => {
  // Extra keys must not cause rejection — the schema is intentionally open
  // so future qsv versions can add fields without breaking the validator.
  const out = validateLlmResponsesShape([
    { kind: "dictionary", response: "ok", extra: 1, anotherField: true },
  ]);
  assert.strictEqual(out, null);
});
