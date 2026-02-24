#!/usr/bin/env bash
# cowork-setup.sh — SessionStart hook
# Copies a qsv CLAUDE.md template into the Cowork working folder
# if one doesn't already exist.

set -euo pipefail

# Read hook input JSON from stdin
INPUT=$(cat)

# Extract cwd from the hook input
CWD=$(echo "$INPUT" | jq -r '.cwd // empty')

if [ -z "$CWD" ]; then
  exit 0
fi

TEMPLATE="${CLAUDE_PLUGIN_ROOT}/cowork-CLAUDE.md"
TARGET="${CWD}/CLAUDE.md"

# Ensure the template exists
if [ ! -f "$TEMPLATE" ]; then
  exit 0
fi

if [ -f "$TARGET" ]; then
  # Existing CLAUDE.md — don't overwrite
  cat <<EOF
{"additionalContext": "An existing CLAUDE.md was found at ${TARGET}. It was NOT overwritten. Inform the user their existing CLAUDE.md is being used."}
EOF
else
  # Copy the template
  cp "$TEMPLATE" "$TARGET"
  cat <<EOF
{"additionalContext": "A qsv CLAUDE.md was created at ${TARGET}. Inform the user that qsv workflow guidance has been set up in their working folder."}
EOF
fi
