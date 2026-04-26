---
name: review-respond
description: Respond to all pending review comments on the current PR — fetch comments, apply fixes, verify accuracy, test, commit, and reply. Use when addressing Copilot reviews, GitHub PR reviews, or any batch of review feedback.
argument-hint: "[PR number]"
---

# Review Response Workflow

Batch-process all pending review comments on a PR in a single pass.

## Step 1: Identify the PR

If a PR number was provided as an argument, use it. Otherwise, detect the current PR:

```bash
gh pr view --json number,url,headRefName --jq '.number'
```

## Step 2: Fetch all pending review comments

```bash
gh api repos/{owner}/{repo}/pulls/{number}/comments --jq '.[] | select(.position != null) | {id, path, line: .original_line, body, user: .user.login}'
```

Also check for PR review threads with unresolved status:

```bash
gh api graphql -f query='{ repository(owner:"{owner}", name:"{repo}") { pullRequest(number:{number}) { reviewThreads(first:100) { nodes { isResolved comments(first:10) { nodes { body author { login } path line } } } } } } }'
```

## Step 3: Route comments by reviewer type

- **Copilot comments** (author is `copilot` or `github-actions`): Invoke `/copilot-review {PR number}` to handle these — it fetches, evaluates, applies, and replies automatically.
- **Human reviewer comments**: Process each one individually in the steps below.

## Step 4: Apply fixes for human reviewer comments

For each unresolved human review comment:

1. **Locate the code with Serena, not bare Read**:
   - `mcp__serena__get_symbols_overview` on the referenced file to see what's there.
   - `mcp__serena__find_symbol` (with `include_body=true` only when you need the body) to jump straight to the function/method/struct the comment is about. Use `name_path_pattern` like `Args/cat_rowskey` rather than scanning the whole file.
   - `mcp__serena__find_referencing_symbols` before changing any signature, return type, or removing a symbol — confirms the blast radius across the codebase.
   - Only fall back to `Read` when the comment is about plain text (docs, comments, USAGE strings) that has no symbolic structure, or when Serena returns empty.

2. **Verify library/API claims with Context7 before agreeing or disagreeing**:
   - If the reviewer asserts behavior of a third-party crate/library/SDK (e.g., "the csv crate's `byte_headers()` consumes the row", "tokio's `spawn_blocking` returns `JoinHandle`"), call `mcp__context7__resolve-library-id` then `mcp__context7__query-docs` to confirm before acting. Reviewer claims are not always correct; library behavior can change between versions.
   - Skip Context7 only when the comment is purely about local code (project-internal logic, naming, project conventions).

3. Apply the fix using `Edit` (preferred) or `mcp__serena__replace_symbol_body` for whole-symbol rewrites.

## Step 5: Verification gate

**Before committing**, verify accuracy of all changes:

- If any change touches documentation that references counts (tool counts, command counts, feature lists): **explicitly list each item by name, then count the list**. Never state a number without showing the enumeration.
- If any change references file paths, confirm they exist (`Bash` with `ls`/`test -f`).
- If any change references a symbol (function, struct, method, constant), use `mcp__serena__find_symbol` to confirm it exists at the path you wrote — not Grep. A grep hit on the name is not proof the symbol resolves there.
- If any change references CLI flags or options, verify they exist in the source or `--help` output.
- If any change relies on third-party library behavior, the relevant Context7 query from Step 4 must already be in this conversation. If you're verifying after the fact, run it now — do not commit on unverified library assumptions.

## Step 6: Run tests

Run the appropriate test suite based on what was changed:

- Rust source (`.rs`): `cargo test -F all_features`
- MCP server (`.ts`): `npm test` (from `.claude/skills/`)
- Specific command: `cargo t {command_name} -F all_features`

## Step 7: Commit

```bash
git add <changed files>
git commit -m "address review: <concise summary of fixes>"
```

## Step 8: Reply to resolved comments

For each comment that was addressed, reply via the GitHub API:

```bash
gh api repos/{owner}/{repo}/pulls/{number}/comments/{comment_id}/replies -f body="Fixed: <what was changed>"
```

## Step 9: Report summary

Output a summary table:

| Metric | Value |
|--------|-------|
| Comments addressed | N |
| Copilot (via /copilot-review) | N |
| Human reviewer | N |
| Tests | passed/failed/skipped |
| Commit | `<hash>` |
