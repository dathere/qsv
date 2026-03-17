#!/usr/bin/env bash
# install-cowork-plugin.sh — Install a .plugin file into Claude Desktop's Cowork environment
#
# Usage:
#   bash install-cowork-plugin.sh <plugin-file>
#   bash install-cowork-plugin.sh qsv-data-wrangling-17.0.0.plugin
#
# What it does:
#   1. Extracts plugin name & version from .claude-plugin/plugin.json inside the archive
#   2. Locates the Cowork local-desktop-app-uploads directory
#   3. Extracts the archive there (replacing any existing version)
#   4. Registers the plugin in marketplace.json and installed_plugins.json
#   5. Claude Desktop picks it up on the next Cowork session
#
# Requirements: macOS with Claude Desktop + Cowork (uses only built-in macOS tools)

set -euo pipefail

# --- Argument handling ---
PLUGIN_FILE="${1:-}"
if [ -z "$PLUGIN_FILE" ]; then
  echo "Usage: bash install-cowork-plugin.sh <plugin-file>"
  echo "Example: bash install-cowork-plugin.sh qsv-data-wrangling-17.0.0.plugin"
  exit 1
fi

if [ ! -f "$PLUGIN_FILE" ]; then
  echo "Error: File not found: $PLUGIN_FILE"
  exit 1
fi

# Verify it's a zip archive
if ! file "$PLUGIN_FILE" | grep -q "Zip archive"; then
  echo "Error: $PLUGIN_FILE is not a valid .plugin archive (expected zip format)"
  exit 1
fi

# --- Extract plugin metadata from the archive ---
PLUGIN_JSON=$(unzip -p "$PLUGIN_FILE" .claude-plugin/plugin.json 2>/dev/null) || {
  echo "Error: Archive does not contain .claude-plugin/plugin.json"
  exit 1
}

# Extract name and version using osascript/JXA (no Python or jq needed)
# Values are passed via environment variables to avoid shell injection
PLUGIN_META=$(PLUGIN_JSON_RAW="$PLUGIN_JSON" osascript -l JavaScript -e "
  ObjC.import('stdlib');
  var raw = $.getenv('PLUGIN_JSON_RAW').js;
  var meta = JSON.parse(raw);
  meta.name + '\n' + meta.version;
" 2>/dev/null) || {
  echo "Error: Could not parse plugin.json"
  exit 1
}
PLUGIN_NAME=$(echo "$PLUGIN_META" | head -1)
PLUGIN_VERSION=$(echo "$PLUGIN_META" | tail -1)

if [ -z "$PLUGIN_NAME" ] || [ -z "$PLUGIN_VERSION" ]; then
  echo "Error: Could not read name/version from plugin.json"
  exit 1
fi

# Validate plugin name to prevent path traversal
if ! echo "$PLUGIN_NAME" | grep -qE '^[a-zA-Z0-9_-]+$'; then
  echo "Error: Invalid plugin name '$PLUGIN_NAME': must contain only alphanumeric characters, hyphens, and underscores"
  exit 1
fi

echo "Installing $PLUGIN_NAME v$PLUGIN_VERSION"
echo "=================================================="

# --- Locate the Cowork plugins directory ---
BASE_DIR="$HOME/Library/Application Support/Claude/local-agent-mode-sessions"

if [ ! -d "$BASE_DIR" ]; then
  echo "Error: Claude local-agent-mode-sessions directory not found."
  echo "  Expected: $BASE_DIR"
  echo "  Make sure Claude Desktop is installed and you've used Cowork at least once."
  exit 1
fi

UPLOADS_DIR=$(find "$BASE_DIR" -type d -name "local-desktop-app-uploads" -path "*/cowork_plugins/marketplaces/*" -print0 2>/dev/null | xargs -0 -r stat -f '%m %N' 2>/dev/null | sort -rn | head -1 | cut -d' ' -f2- || true)

if [ -z "$UPLOADS_DIR" ]; then
  echo "Error: Could not find local-desktop-app-uploads directory."
  echo "  Make sure you have at least one Cowork session with a plugin installed."
  echo "  Tip: Install any marketplace plugin first (e.g., Data or Productivity)."
  exit 1
fi

# Derive paths
COWORK_PLUGINS_DIR=$(dirname "$(dirname "$UPLOADS_DIR")")
DEST="$UPLOADS_DIR/$PLUGIN_NAME"
MARKETPLACE_JSON="$UPLOADS_DIR/.claude-plugin/marketplace.json"
INSTALLED_JSON="$COWORK_PLUGINS_DIR/installed_plugins.json"

echo "  Plugin archive: $PLUGIN_FILE"
echo "  Install path:   $DEST"

# --- Install the plugin files ---
if [ -d "$DEST" ]; then
  # Sanity check: ensure DEST is inside UPLOADS_DIR before removing
  if [[ "$DEST" != "$UPLOADS_DIR"/* ]]; then
    echo "Error: Install path '$DEST' is not inside uploads directory '$UPLOADS_DIR'"
    exit 1
  fi
  echo "  Replacing existing installation..."
  rm -rf "$DEST"
fi

mkdir -p "$DEST"

# Validate ZIP entries for path traversal (Zip Slip) before extracting
BAD_ENTRIES=$(unzip -l "$PLUGIN_FILE" 2>/dev/null | awk 'NR>3 && !/^-/ {print $4}' | grep -E '(^/|\.\./)' || true)
if [ -n "$BAD_ENTRIES" ]; then
  echo "Error: Archive contains unsafe paths (absolute or traversal):"
  echo "$BAD_ENTRIES"
  exit 1
fi

unzip -qo "$PLUGIN_FILE" -d "$DEST"
echo "  Extracted plugin files"

# --- Update marketplace.json using osascript (JXA — ships with macOS) ---
# All values are passed via environment variables to avoid shell injection
if [ -f "$MARKETPLACE_JSON" ]; then
  MJ_PATH="$MARKETPLACE_JSON" P_NAME="$PLUGIN_NAME" P_VERSION="$PLUGIN_VERSION" \
  osascript -l JavaScript -e "
    ObjC.import('stdlib');
    var app = Application.currentApplication(); app.includeStandardAdditions = true;
    var mj = $.getenv('MJ_PATH').js;
    var name = $.getenv('P_NAME').js;
    var version = $.getenv('P_VERSION').js;
    var data = JSON.parse(app.read(Path(mj)));
    data.plugins = (data.plugins || []).filter(function(p) { return p.name !== name; });
    data.plugins.push({ name: name, version: version, source: './' + name });
    var fh = app.openForAccess(Path(mj), { writePermission: true });
    try { app.setEof(fh, { to: 0 }); app.write(JSON.stringify(data, null, 2) + '\n', { to: fh }); }
    finally { app.closeAccess(fh); }
  " 2>/dev/null
  echo "  Updated marketplace.json"
else
  echo "  Warning: marketplace.json not found at $MARKETPLACE_JSON — skipping"
fi

# --- Update installed_plugins.json using osascript (JXA) ---
if [ -f "$INSTALLED_JSON" ]; then
  NOW=$(date -u '+%Y-%m-%dT%H:%M:%S.000Z')
  IJ_PATH="$INSTALLED_JSON" P_NAME="$PLUGIN_NAME" P_VERSION="$PLUGIN_VERSION" \
  P_DEST="$DEST" P_NOW="$NOW" \
  osascript -l JavaScript -e "
    ObjC.import('stdlib');
    var app = Application.currentApplication(); app.includeStandardAdditions = true;
    var ij = $.getenv('IJ_PATH').js;
    var name = $.getenv('P_NAME').js;
    var version = $.getenv('P_VERSION').js;
    var dest = $.getenv('P_DEST').js;
    var now = $.getenv('P_NOW').js;
    var data = JSON.parse(app.read(Path(ij)));
    if (!data.plugins) data.plugins = {};
    data.plugins[name + '@local-desktop-app-uploads'] = [{
      scope: 'user', installPath: dest, version: version,
      installedAt: now, lastUpdated: now
    }];
    var fh = app.openForAccess(Path(ij), { writePermission: true });
    try { app.setEof(fh, { to: 0 }); app.write(JSON.stringify(data, null, 2) + '\n', { to: fh }); }
    finally { app.closeAccess(fh); }
  " 2>/dev/null
  echo "  Updated installed_plugins.json"
else
  echo "  Warning: installed_plugins.json not found at $INSTALLED_JSON — skipping"
fi

# --- Done ---
echo ""
echo "=================================================="
echo "Installed $PLUGIN_NAME v$PLUGIN_VERSION"
echo ""
echo "Next steps:"
echo "  1. Start a new Cowork session (or restart Claude Desktop)"
echo "  2. The plugin's skills, commands, and agents will be available"
echo "  3. If the plugin has hooks, they activate on the next session start"
