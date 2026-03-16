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
# Requirements: macOS with Claude Desktop + Cowork, unzip, python3 (ships with macOS)

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

PLUGIN_NAME=$(echo "$PLUGIN_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['name'])")
PLUGIN_VERSION=$(echo "$PLUGIN_JSON" | python3 -c "import sys,json; print(json.load(sys.stdin)['version'])")

if [ -z "$PLUGIN_NAME" ] || [ -z "$PLUGIN_VERSION" ]; then
  echo "Error: Could not read name/version from plugin.json"
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

UPLOADS_DIR=$(find "$BASE_DIR" -type d -name "local-desktop-app-uploads" -path "*/cowork_plugins/marketplaces/*" 2>/dev/null | head -1)

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
  echo "  Replacing existing installation..."
  rm -rf "$DEST"
fi

mkdir -p "$DEST"
unzip -qo "$PLUGIN_FILE" -d "$DEST"
echo "  Extracted plugin files"

# --- Update marketplace.json ---
if [ -f "$MARKETPLACE_JSON" ]; then
  python3 -c "
import json, sys

with open('$MARKETPLACE_JSON', 'r') as f:
    data = json.load(f)

plugins = data.get('plugins', [])

# Remove existing entry for this plugin name
plugins = [p for p in plugins if p.get('name') != '$PLUGIN_NAME']

# Add new entry
plugins.append({
    'name': '$PLUGIN_NAME',
    'version': '$PLUGIN_VERSION',
    'source': './$PLUGIN_NAME'
})

data['plugins'] = plugins

with open('$MARKETPLACE_JSON', 'w') as f:
    json.dump(data, f, indent=2)
    f.write('\n')

print('  Updated marketplace.json')
"
else
  echo "  Warning: marketplace.json not found at $MARKETPLACE_JSON — skipping"
fi

# --- Update installed_plugins.json ---
if [ -f "$INSTALLED_JSON" ]; then
  python3 -c "
import json, datetime

with open('$INSTALLED_JSON', 'r') as f:
    data = json.load(f)

key = '$PLUGIN_NAME@local-desktop-app-uploads'
now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%dT%H:%M:%S.000Z')

data.setdefault('plugins', {})[key] = [{
    'scope': 'user',
    'installPath': '$DEST',
    'version': '$PLUGIN_VERSION',
    'installedAt': now,
    'lastUpdated': now
}]

with open('$INSTALLED_JSON', 'w') as f:
    json.dump(data, f, indent=2)
    f.write('\n')

print('  Updated installed_plugins.json')
"
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
