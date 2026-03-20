# Reproducible Analysis

Maintain a machine-readable journal of every data operation so that humans, agents, and machines can independently verify the analysis end-to-end.

## Core Principle

Every analysis should be **independently reproducible**: given the same input files, a third party should be able to replay the exact sequence of operations and arrive at bit-identical results for all deterministic steps.

## Journal Format

Create a journal file named `<analysis-name>.journal.jsonl` alongside the analysis output. Each line is a JSON object representing one operation.

### Entry Schema

```jsonl
{"seq": 1, "ts": "2026-03-19T14:30:00Z", "op": "index", "tool": "qsv_index", "input": "sales.csv", "input_sha256": "a1b2c3...", "input_rows": 50000, "input_cols": 12, "params": {}, "output": "sales.csv.idx", "output_sha256": "d4e5f6...", "duration_ms": 45, "note": "Create index for fast access"}
{"seq": 2, "ts": "2026-03-19T14:30:01Z", "op": "stats", "tool": "qsv_stats", "input": "sales.csv", "input_sha256": "a1b2c3...", "params": {"cardinality": true, "stats_jsonl": true}, "output": "sales.stats.csv", "output_sha256": "f7a8b9...", "duration_ms": 320, "note": "Generate stats cache with cardinality"}
```

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `seq` | integer | 1-based sequence number within the journal |
| `ts` | string | ISO 8601 UTC timestamp of when the operation ran |
| `op` | string | Human-readable operation name (e.g., "stats", "filter", "join") |
| `tool` | string or null | Exact MCP tool name used (e.g., `qsv_stats`, `qsv_sqlp`); null for journal-level entries (`init`, `complete`) |
| `input` | string, array, or null | Input file path(s), relative to working directory; null for journal-level entries |
| `input_sha256` | string, array, or null | SHA-256 hash(es) of input file(s); null for journal-level entries |
| `params` | object | All parameters passed to the tool (excluding input/output paths) |
| `output` | string or null | Output file path, or null if result was displayed only |
| `output_sha256` | string or null | SHA-256 hash of output file, or null |
| `duration_ms` | integer | Wall-clock execution time in milliseconds |
| `note` | string | Brief explanation of *why* this step was performed |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `input_rows` | integer | Row count of input (from `qsv_count`) |
| `input_cols` | integer | Column count of input (from `qsv_headers`) |
| `output_rows` | integer | Row count of output |
| `output_cols` | integer | Column count of output |
| `delta_rows` | integer | Rows added/removed (output_rows - input_rows) |
| `deterministic` | boolean | Whether this step produces identical output every run (default: true) |
| `ai_generated` | boolean | Whether this step involved AI inference (e.g., describegpt) |
| `sql` | string | Full SQL query text (for sqlp operations) |
| `error` | string | Error message if the operation failed |
| `version` | string | qsv version string (capture once at journal start) |

## How to Compute Hashes

Use `qsv_sqlp` or shell commands to compute SHA-256 hashes:

```bash
# Via shell (when available)
shasum -a 256 sales.csv | cut -d' ' -f1

# Via qsv sqlp (for CSV content hash)
# Hash the output file after each step
```

Alternatively, note the file size and row count as a lighter-weight fingerprint when hashing is impractical:

```jsonl
{"seq": 1, "input": "huge_file.csv", "input_fingerprint": {"rows": 5000000, "cols": 42, "bytes": 1073741824}, "note": "additional fields omitted for brevity"}
```

## Journal Lifecycle

### Starting a Journal

At the beginning of any analysis, create the journal and record the environment:

```jsonl
{"seq": 0, "ts": "2026-03-19T14:29:55Z", "op": "init", "tool": null, "input": null, "params": {"working_dir": "/path/to/data", "qsv_version": "0.142.0 (polars-0.46.0)", "platform": "darwin-aarch64"}, "output": "analysis.journal.jsonl", "note": "Initialize reproducibility journal"}
```

### During Analysis

Log every data operation. For each step:
1. Record the entry *after* the operation completes (so you have the output hash and duration)
2. Include the `note` field explaining the analytical reasoning — this is what makes the journal useful to human reviewers
3. Mark `deterministic: false` for any AI-generated step (describegpt, chart selection, narrative)

### Closing a Journal

At the end, write a summary entry:

```jsonl
{"seq": 99, "ts": "2026-03-19T15:10:00Z", "op": "complete", "tool": null, "input": "sales.csv", "input_sha256": "a1b2c3...", "params": {"total_steps": 98, "deterministic_steps": 95, "ai_steps": 3, "final_output": "analysis_report.md"}, "output": "analysis.journal.jsonl", "note": "Analysis complete. 95 of 98 steps are deterministic and independently reproducible."}
```

## Verification Protocol

### For Humans

1. Open the `.journal.jsonl` file
2. Review each `note` field to understand the analytical reasoning
3. Check that the sequence of operations makes logical sense
4. Verify `input_sha256` of the first entry matches your copy of the source data
5. Spot-check any step by re-running the `tool` with the recorded `params`

### For Agents

1. Parse the `.journal.jsonl` file
2. Verify the `seq` 0 entry to confirm environment compatibility (qsv version, platform)
3. For each entry where `deterministic` is true (or absent):
   a. Compute `sha256` of the input file — must match `input_sha256`
   b. Execute the `tool` with the recorded `params`
   c. Compute `sha256` of the output — must match `output_sha256`
   d. If mismatch, flag the step and stop
4. For entries where `deterministic: false`, skip hash verification but log that the step was AI-generated
5. Report: `N of M deterministic steps verified, K AI-generated steps skipped`

### For Machines (CI/CD)

```bash
#!/bin/bash
# replay-journal.sh — replay and verify a journal
JOURNAL="$1"
FAILURES=0

jq -c 'select(.seq > 0 and .op != "complete" and (.deterministic // true))' "$JOURNAL" | while read -r entry; do
  SEQ=$(echo "$entry" | jq -r '.seq')
  INPUT=$(echo "$entry" | jq -r '.input')
  EXPECTED=$(echo "$entry" | jq -r '.output_sha256')

  # Verify input hash
  ACTUAL_INPUT_HASH=$(shasum -a 256 "$INPUT" | cut -d' ' -f1)
  INPUT_HASH=$(echo "$entry" | jq -r '.input_sha256')
  if [ "$ACTUAL_INPUT_HASH" != "$INPUT_HASH" ]; then
    echo "FAIL step $SEQ: input hash mismatch"
    FAILURES=$((FAILURES + 1))
    continue
  fi

  # Re-execute and verify output hash (tool-specific replay logic here)
  # ...

  echo "PASS step $SEQ"
done

echo "$FAILURES failures"
exit $FAILURES
```

## Integration with Commands

When any `/data-*` command is invoked and the user requests reproducibility (or the output is a formal deliverable), maintain a journal:

| Command | Journal Approach |
|---------|-----------------|
| `/data-profile` | Log every profiling step (index, sniff, stats, frequency, etc.) |
| `/data-clean` | Log each cleaning operation with before/after row counts |
| `/data-join` | Log both inputs with hashes, join parameters, output verification |
| `/csv-query` | Log the SQL query text in the `sql` field |
| `/data-validate` | Log each validation check and its pass/fail result |
| `/data-viz` | Log data preparation steps; mark chart generation as `deterministic: false` |
| `/data-describe` | Log stats step as deterministic, describegpt step as `ai_generated: true` |
| `/data-convert` | Log input/output formats and hashes |

## Integration with GenAI Disclaimer

The journal complements the `genai-disclaimer` skill:
- The journal records *what* was done and enables replay
- The disclaimer communicates *which parts* are AI-generated vs. deterministic
- Together, they provide full transparency for stakeholders

Use the journal's `deterministic` and `ai_generated` flags to auto-generate the disclaimer's attribution table.

## Best Practices

- **Always hash inputs**: The input hash is the anchor for reproducibility — without it, verification is impossible
- **Log failures too**: If a step fails and you retry with different parameters, log both attempts (the failure with an `error` field, then the successful retry)
- **Include row count deltas**: `delta_rows` makes it easy to spot where data was filtered, joined, or deduplicated
- **Use relative paths**: All file paths should be relative to the working directory so the journal is portable
- **Version pin**: Record `qsv_version` in the init entry — different versions may produce different stats precision
- **One journal per analysis**: Don't append unrelated analyses to the same journal file
- **Commit journals to version control**: They're small (a few KB) and provide audit trail
