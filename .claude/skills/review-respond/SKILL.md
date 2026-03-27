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

1. Read the referenced file and the specific lines
2. Understand what the reviewer is asking for
3. Apply the fix

## Step 5: Verification gate

**Before committing**, verify accuracy of all changes:

- If any change touches documentation that references counts (tool counts, command counts, feature lists): **explicitly list each item by name, then count the list**. Never state a number without showing the enumeration.
- If any change references file paths, grep to confirm they exist.
- If any change references CLI flags or options, verify they exist in the source or `--help` output.

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
