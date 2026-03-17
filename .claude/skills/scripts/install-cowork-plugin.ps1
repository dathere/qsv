#Requires -Version 5.1
<#
.SYNOPSIS
    Install a .plugin file into Claude Desktop's Cowork environment (Windows).

.DESCRIPTION
    Extracts plugin name & version from .claude-plugin/plugin.json inside the archive,
    locates the Cowork local-desktop-app-uploads directory, extracts the archive there
    (replacing any existing version), and registers the plugin in marketplace.json and
    installed_plugins.json. Claude Desktop picks it up on the next Cowork session.

.PARAMETER PluginFile
    Path to the .plugin file (a ZIP archive with .claude-plugin/plugin.json inside).

.EXAMPLE
    .\install-cowork-plugin.ps1 qsv-data-wrangling-17.0.0.plugin
#>

param(
    [Parameter(Mandatory = $true, Position = 0)]
    [string]$PluginFile
)

$ErrorActionPreference = 'Stop'

# --- Validate input ---
if (-not (Test-Path $PluginFile -PathType Leaf)) {
    Write-Error "File not found: $PluginFile"
    exit 1
}

$PluginFile = (Resolve-Path $PluginFile).Path

# Verify it's a zip by checking the magic bytes (PK header)
$header = [System.IO.File]::ReadAllBytes($PluginFile)[0..1]
if ($header[0] -ne 0x50 -or $header[1] -ne 0x4B) {
    Write-Error "$PluginFile is not a valid .plugin archive (expected zip format)"
    exit 1
}

# --- Extract plugin metadata from the archive ---
try {
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $zip = [System.IO.Compression.ZipFile]::OpenRead($PluginFile)
    $entry = $zip.Entries | Where-Object { $_.FullName -eq '.claude-plugin/plugin.json' } | Select-Object -First 1
    if (-not $entry) { throw 'Not found' }
    $reader = [System.IO.StreamReader]::new($entry.Open())
    $pluginJsonText = $reader.ReadToEnd()
    $reader.Close()
    $zip.Dispose()
} catch {
    Write-Error "Archive does not contain .claude-plugin/plugin.json"
    exit 1
}

$pluginJson = $pluginJsonText | ConvertFrom-Json
$pluginName = $pluginJson.name
$pluginVersion = $pluginJson.version

if (-not $pluginName -or -not $pluginVersion) {
    Write-Error "Could not read name/version from plugin.json"
    exit 1
}

# Validate plugin name to prevent path traversal
if ($pluginName -notmatch '^[a-zA-Z0-9_-]+$') {
    Write-Error "Invalid plugin name '$pluginName': must contain only alphanumeric characters, hyphens, and underscores"
    exit 1
}

Write-Host "Installing $pluginName v$pluginVersion"
Write-Host ('=' * 50)

# --- Locate the Cowork plugins directory ---
$baseDir = Join-Path $env:APPDATA 'Claude\local-agent-mode-sessions'

if (-not (Test-Path $baseDir -PathType Container)) {
    Write-Error @"
Claude local-agent-mode-sessions directory not found.
  Expected: $baseDir
  Make sure Claude Desktop is installed and you've used Cowork at least once.
"@
    exit 1
}

# Find the most recently modified local-desktop-app-uploads directory
$uploadsDir = Get-ChildItem -Path $baseDir -Recurse -Directory -Filter 'local-desktop-app-uploads' -ErrorAction SilentlyContinue |
    Where-Object { $_.FullName -match 'cowork_plugins[\\/]marketplaces[\\/]' } |
    Sort-Object LastWriteTime -Descending |
    Select-Object -First 1 -ExpandProperty FullName

if (-not $uploadsDir) {
    Write-Error @"
Could not find local-desktop-app-uploads directory.
  Make sure you have at least one Cowork session with a plugin installed.
  Tip: Install any marketplace plugin first (e.g., Data or Productivity).
"@
    exit 1
}

# Derive paths
$coworkPluginsDir = Split-Path (Split-Path $uploadsDir -Parent) -Parent
$dest = Join-Path $uploadsDir $pluginName
$marketplaceJson = Join-Path $uploadsDir '.claude-plugin\marketplace.json'
$installedJson = Join-Path $coworkPluginsDir 'installed_plugins.json'

Write-Host "  Plugin archive: $PluginFile"
Write-Host "  Install path:   $dest"

# --- Install the plugin files ---
if (Test-Path $dest -PathType Container) {
    # Sanity check: ensure dest is inside uploadsDir before removing
    if (-not $dest.StartsWith($uploadsDir, [System.StringComparison]::OrdinalIgnoreCase)) {
        Write-Error "Install path '$dest' is not inside uploads directory '$uploadsDir'"
        exit 1
    }
    Write-Host '  Replacing existing installation...'
    Remove-Item $dest -Recurse -Force
}

New-Item -ItemType Directory -Path $dest -Force | Out-Null
Expand-Archive -Path $PluginFile -DestinationPath $dest -Force
Write-Host '  Extracted plugin files'

# --- Update marketplace.json ---
if (Test-Path $marketplaceJson -PathType Leaf) {
    $data = Get-Content $marketplaceJson -Raw | ConvertFrom-Json

    # Ensure plugins is a mutable list
    $plugins = [System.Collections.ArrayList]@()
    if ($data.plugins) {
        foreach ($p in $data.plugins) {
            if ($p.name -ne $pluginName) {
                [void]$plugins.Add($p)
            }
        }
    }

    $newEntry = [PSCustomObject]@{
        name    = $pluginName
        version = $pluginVersion
        source  = "./$pluginName"
    }
    [void]$plugins.Add($newEntry)
    $data.plugins = $plugins.ToArray()

    $data | ConvertTo-Json -Depth 10 | Set-Content $marketplaceJson -Encoding UTF8
    Write-Host '  Updated marketplace.json'
} else {
    Write-Host "  Warning: marketplace.json not found at $marketplaceJson - skipping"
}

# --- Update installed_plugins.json ---
if (Test-Path $installedJson -PathType Leaf) {
    $data = Get-Content $installedJson -Raw | ConvertFrom-Json
    $now = (Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ss.000Z')
    $key = "$pluginName@local-desktop-app-uploads"

    $entry = [PSCustomObject]@{
        scope       = 'user'
        installPath = $dest
        version     = $pluginVersion
        installedAt = $now
        lastUpdated = $now
    }

    # Add or replace the plugin entry
    if (-not $data.plugins) {
        $data | Add-Member -NotePropertyName 'plugins' -NotePropertyValue ([PSCustomObject]@{})
    }
    if ($data.plugins.PSObject.Properties[$key]) {
        $data.plugins.$key = @($entry)
    } else {
        $data.plugins | Add-Member -NotePropertyName $key -NotePropertyValue @($entry)
    }

    $data | ConvertTo-Json -Depth 10 | Set-Content $installedJson -Encoding UTF8
    Write-Host '  Updated installed_plugins.json'
} else {
    Write-Host "  Warning: installed_plugins.json not found at $installedJson - skipping"
}

# --- Done ---
Write-Host ''
Write-Host ('=' * 50)
Write-Host "Installed $pluginName v$pluginVersion"
Write-Host ''
Write-Host 'Next steps:'
Write-Host '  1. Start a new Cowork session (or restart Claude Desktop)'
Write-Host '  2. The plugin''s skills, commands, and agents will be available'
Write-Host '  3. If the plugin has hooks, they activate on the next session start'
