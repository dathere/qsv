#!/usr/bin/env bash
# cowork-setup.sh — SessionStart hook
# Copies a qsv CLAUDE.md template into the Cowork working folder
# if one doesn't already exist.

set -euo pipefail

# Allow opting out via environment variable
if [ "${QSV_NO_COWORK_SETUP:-}" = "1" ]; then
  exit 0
fi

# Require jq for JSON parsing
if ! command -v jq &>/dev/null; then
  echo '{"additionalContext": "jq is not installed. Skipping qsv CLAUDE.md deployment. Install jq for automatic workflow guidance setup."}'
  exit 0
fi

# Extract cwd from hook input JSON (limit to 64KB; timeout after 5s to avoid hangs)
# Pipe directly through head into jq to avoid platform-specific read/timeout issues
CWD=$(head -c 65536 | jq -r '.cwd // empty' 2>/dev/null) || CWD=""

if [ -z "$CWD" ]; then
  exit 0
fi

# Resolve to real path to prevent path traversal via symlinks (POSIX-portable)
CWD=$(cd "$CWD" 2>/dev/null && pwd -P) || exit 0

# Guard against deploying into the plugin's own directory tree
if [ -z "${CLAUDE_PLUGIN_ROOT:-}" ]; then
  echo '{"additionalContext": "CLAUDE_PLUGIN_ROOT is not set. Skipping qsv CLAUDE.md deployment."}'
  exit 0
fi
PLUGIN_ROOT=$(cd "$CLAUDE_PLUGIN_ROOT" 2>/dev/null && pwd -P) || exit 0
case "$CWD" in
  "$PLUGIN_ROOT"|"$PLUGIN_ROOT"/*) exit 0 ;;
esac

TEMPLATE="${CLAUDE_PLUGIN_ROOT}/cowork-CLAUDE.md"
TARGET="${CWD}/CLAUDE.md"

# Ensure the template exists
if [ ! -f "$TEMPLATE" ]; then
  exit 0
fi

if [ -f "$TARGET" ]; then
  # Existing CLAUDE.md — don't overwrite
  jq -n --arg path "$TARGET" \
    '{"additionalContext": "An existing CLAUDE.md was found at \($path). It was NOT overwritten. The existing file will be used for workflow guidance."}'
else
  # Copy the template
  if cp "$TEMPLATE" "$TARGET" 2>/dev/null; then
    jq -n --arg path "$TARGET" \
      '{"additionalContext": "A qsv CLAUDE.md was created at \($path). qsv workflow guidance has been set up in the working folder."}'
  else
    jq -n --arg path "$CWD" \
      '{"additionalContext": "Could not create CLAUDE.md in \($path) (directory may not be writable). Skipping qsv workflow guidance setup."}'
  fi
fi
