/**
 * Directory Picker MCP App HTML
 *
 * Self-contained HTML for an interactive directory browser rendered as an MCP App.
 * Uses the @modelcontextprotocol/ext-apps App SDK loaded from CDN.
 *
 * The App communicates with the MCP server via:
 * - `qsv_browse_directory` to list directory contents
 * - `qsv_set_working_dir` to confirm the selected directory
 */

/**
 * Returns the self-contained HTML string for the directory picker App.
 * Vanilla JS (no framework) with breadcrumb navigation, directory listing,
 * quick-access buttons, and a "Select" confirmation action.
 */
export function getDirectoryPickerHtml(): string {
  // Use a template literal for the full HTML — no external dependencies except
  // the MCP Apps SDK loaded from esm.sh CDN.
  return /* html */ `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>qsv Directory Picker</title>
<style>
  :root {
    --bg: var(--color-background-primary, #ffffff);
    --bg-secondary: var(--color-background-secondary, #f5f5f5);
    --bg-hover: var(--color-background-tertiary, #e8e8e8);
    --bg-info: var(--color-background-info, #eff6ff);
    --text: var(--color-text-primary, #1a1a1a);
    --text-secondary: var(--color-text-secondary, #666666);
    --text-info: var(--color-text-info, #1d4ed8);
    --border: var(--color-border-primary, #e0e0e0);
    --border-secondary: var(--color-border-secondary, #d0d0d0);
    --ring: var(--color-ring-primary, #3b82f6);
    --ring-info: var(--color-ring-info, #60a5fa);
    --radius: var(--border-radius-md, 8px);
    --radius-sm: var(--border-radius-sm, 4px);
    --font: var(--font-sans, system-ui, -apple-system, sans-serif);
    --font-mono: var(--font-mono, ui-monospace, monospace);
  }

  [data-theme="dark"] {
    --bg: var(--color-background-primary, #1a1a1a);
    --bg-secondary: var(--color-background-secondary, #262626);
    --bg-hover: var(--color-background-tertiary, #333333);
    --bg-info: var(--color-background-info, #1e293b);
    --text: var(--color-text-primary, #e5e5e5);
    --text-secondary: var(--color-text-secondary, #a0a0a0);
    --text-info: var(--color-text-info, #60a5fa);
    --border: var(--color-border-primary, #404040);
    --border-secondary: var(--color-border-secondary, #505050);
  }

  * { box-sizing: border-box; margin: 0; padding: 0; }

  body {
    font-family: var(--font);
    background: var(--bg);
    color: var(--text);
    font-size: 14px;
    line-height: 1.5;
    overflow-x: hidden;
  }

  .container {
    max-width: 600px;
    margin: 0 auto;
    padding: 16px;
  }

  .header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 12px;
  }

  .header h1 {
    font-size: 16px;
    font-weight: 600;
  }

  /* Breadcrumb navigation */
  .breadcrumbs {
    display: flex;
    align-items: center;
    gap: 4px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    margin-bottom: 12px;
    overflow-x: auto;
    white-space: nowrap;
    font-family: var(--font-mono);
    font-size: 13px;
  }

  .breadcrumb-item {
    cursor: pointer;
    color: var(--text-info);
    padding: 2px 4px;
    border-radius: var(--radius-sm);
    flex-shrink: 0;
  }
  .breadcrumb-item:hover { background: var(--bg-hover); }
  .breadcrumb-item.current {
    color: var(--text);
    font-weight: 600;
    cursor: default;
  }
  .breadcrumb-item.current:hover { background: transparent; }
  .breadcrumb-sep { color: var(--text-secondary); flex-shrink: 0; }

  /* Quick access buttons */
  .quick-access {
    display: flex;
    gap: 8px;
    margin-bottom: 12px;
    flex-wrap: wrap;
  }

  .quick-btn {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    padding: 6px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    color: var(--text);
    cursor: pointer;
    font-size: 13px;
    font-family: var(--font);
    transition: background 0.15s;
  }
  .quick-btn:hover { background: var(--bg-hover); }
  .quick-btn.active {
    background: var(--bg-info);
    border-color: var(--ring-info);
    color: var(--text-info);
  }

  /* Directory listing */
  .dir-list {
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
    margin-bottom: 12px;
  }

  .dir-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
    border-bottom: 1px solid var(--border);
    cursor: pointer;
    transition: background 0.1s;
  }
  .dir-item:last-child { border-bottom: none; }
  .dir-item:hover { background: var(--bg-hover); }
  .dir-item.selected {
    background: var(--bg-info);
    border-color: var(--ring-info);
  }

  .dir-icon { flex-shrink: 0; font-size: 18px; }
  .dir-name { flex: 1; font-weight: 500; }
  .dir-meta {
    font-size: 12px;
    color: var(--text-secondary);
    flex-shrink: 0;
  }
  .dir-arrow { color: var(--text-secondary); flex-shrink: 0; }

  .parent-item { font-style: italic; color: var(--text-secondary); }
  .parent-item .dir-name { font-weight: 400; }

  .empty-state {
    padding: 24px;
    text-align: center;
    color: var(--text-secondary);
  }

  /* Status bar */
  .status-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    font-size: 13px;
    color: var(--text-secondary);
    margin-bottom: 12px;
  }

  .status-path {
    font-family: var(--font-mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    flex: 1;
  }

  /* Actions */
  .actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
  }

  .btn {
    padding: 8px 20px;
    border-radius: var(--radius);
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    border: 1px solid var(--border);
    font-family: var(--font);
    transition: background 0.15s, border-color 0.15s;
  }
  .btn-secondary {
    background: var(--bg);
    color: var(--text);
  }
  .btn-secondary:hover { background: var(--bg-hover); }
  .btn-primary {
    background: var(--ring);
    color: #fff;
    border-color: var(--ring);
  }
  .btn-primary:hover { opacity: 0.9; }
  .btn-primary:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  /* Loading spinner */
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 24px;
    color: var(--text-secondary);
  }
  .spinner {
    width: 20px;
    height: 20px;
    border: 2px solid var(--border);
    border-top-color: var(--ring);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }
  @keyframes spin { to { transform: rotate(360deg); } }

  /* Error state */
  .error-msg {
    padding: 12px;
    background: var(--color-background-danger, #fef2f2);
    border: 1px solid var(--color-border-danger, #fca5a5);
    border-radius: var(--radius);
    color: var(--color-text-danger, #dc2626);
    font-size: 13px;
    margin-bottom: 12px;
  }

  /* Success state */
  .success-msg {
    padding: 16px;
    background: var(--color-background-success, #f0fdf4);
    border: 1px solid var(--color-border-success, #86efac);
    border-radius: var(--radius);
    color: var(--color-text-success, #16a34a);
    font-size: 14px;
    text-align: center;
  }
</style>
</head>
<body>
<div class="container" id="app">
  <div class="header">
    <span style="font-size:20px">&#128193;</span>
    <h1>Select Working Directory</h1>
  </div>

  <div id="error" class="error-msg" style="display:none"></div>
  <div id="success" class="success-msg" style="display:none"></div>

  <div id="main-ui">
    <div id="quick-access" class="quick-access"></div>
    <div id="breadcrumbs" class="breadcrumbs"></div>
    <div id="dir-list" class="dir-list">
      <div class="loading"><div class="spinner"></div><span>Loading...</span></div>
    </div>
    <div id="status-bar" class="status-bar">
      <span class="status-path" id="status-path">-</span>
      <span id="status-count">-</span>
    </div>
    <div class="actions">
      <button class="btn btn-primary" id="select-btn" disabled>Select This Directory</button>
    </div>
  </div>
</div>

<script type="module">
import { App } from "https://esm.sh/@modelcontextprotocol/ext-apps@1.2.0/app";
import { applyDocumentTheme, applyHostStyleVariables } from "https://esm.sh/@modelcontextprotocol/ext-apps@1.2.0/styles";

const app = new App({ name: "qsv-directory-picker", version: "1.0.0" });

let currentPath = "";
let knownDirs = [];
let selectedPath = "";

// DOM refs
const errorEl = document.getElementById("error");
const successEl = document.getElementById("success");
const mainUi = document.getElementById("main-ui");
const quickAccessEl = document.getElementById("quick-access");
const breadcrumbsEl = document.getElementById("breadcrumbs");
const dirListEl = document.getElementById("dir-list");
const statusPath = document.getElementById("status-path");
const statusCount = document.getElementById("status-count");
const selectBtn = document.getElementById("select-btn");

function showError(msg) {
  errorEl.textContent = msg;
  errorEl.style.display = "block";
  setTimeout(() => { errorEl.style.display = "none"; }, 5000);
}

function showSuccess(msg) {
  successEl.textContent = msg;
  successEl.style.display = "block";
  mainUi.style.display = "none";
}

function updateSelectedState(path) {
  selectedPath = path;
  selectBtn.disabled = false;
  statusPath.textContent = path;

  // Update quick-btn active state
  quickAccessEl.querySelectorAll(".quick-btn").forEach(btn => {
    btn.classList.toggle("active", btn.dataset.path === path);
  });
}

function renderBreadcrumbs(path) {
  breadcrumbsEl.innerHTML = "";
  const sep = path.includes("\\\\") ? "\\\\" : "/";
  const parts = path.split(sep).filter(Boolean);

  // Root
  const rootEl = document.createElement("span");
  rootEl.className = "breadcrumb-item";
  rootEl.textContent = sep === "/" ? "/" : parts[0] + "\\\\";
  rootEl.onclick = () => navigate(sep === "/" ? "/" : parts[0] + "\\\\");
  breadcrumbsEl.appendChild(rootEl);

  let accumulated = sep === "/" ? "" : "";
  for (let i = sep === "/" ? 0 : 1; i < parts.length; i++) {
    const sepEl = document.createElement("span");
    sepEl.className = "breadcrumb-sep";
    sepEl.textContent = ">";
    breadcrumbsEl.appendChild(sepEl);

    accumulated += sep + parts[i];
    const fullPath = sep === "/" ? accumulated : parts[0] + accumulated;

    const crumb = document.createElement("span");
    crumb.textContent = parts[i];
    const isLast = i === parts.length - 1;
    crumb.className = "breadcrumb-item" + (isLast ? " current" : "");
    if (!isLast) {
      const p = fullPath;
      crumb.onclick = () => navigate(p);
    }
    breadcrumbsEl.appendChild(crumb);
  }
}

function renderDirList(data) {
  dirListEl.innerHTML = "";

  // Parent directory entry
  if (data.parent) {
    const item = document.createElement("div");
    item.className = "dir-item parent-item";
    item.innerHTML = '<span class="dir-icon">&#128194;</span>' +
      '<span class="dir-name">.. (parent directory)</span>' +
      '<span class="dir-arrow">&#8593;</span>';
    item.onclick = () => navigate(data.parent);
    dirListEl.appendChild(item);
  }

  if (!data.subdirectories || data.subdirectories.length === 0) {
    const empty = document.createElement("div");
    empty.className = "empty-state";
    empty.textContent = "No subdirectories";
    dirListEl.appendChild(empty);
    return;
  }

  for (const dir of data.subdirectories) {
    const item = document.createElement("div");
    item.className = "dir-item";

    const metaParts = [];
    if (dir.tabularFileCount > 0) metaParts.push(dir.tabularFileCount + " data file" + (dir.tabularFileCount !== 1 ? "s" : ""));
    if (dir.subdirCount > 0) metaParts.push(dir.subdirCount + " folder" + (dir.subdirCount !== 1 ? "s" : ""));

    item.innerHTML =
      '<span class="dir-icon">&#128193;</span>' +
      '<span class="dir-name">' + escapeHtml(dir.name) + '</span>' +
      (metaParts.length > 0 ? '<span class="dir-meta">' + metaParts.join(", ") + '</span>' : '') +
      '<span class="dir-arrow">&#8250;</span>';

    item.onclick = () => navigate(dir.path);
    dirListEl.appendChild(item);
  }

  statusCount.textContent = data.subdirectories.length + " folder" + (data.subdirectories.length !== 1 ? "s" : "") +
    (data.tabularFileCount > 0 ? ", " + data.tabularFileCount + " data file" + (data.tabularFileCount !== 1 ? "s" : "") : "");
}

function escapeHtml(str) {
  const div = document.createElement("div");
  div.textContent = str;
  return div.innerHTML;
}

async function navigate(path) {
  dirListEl.innerHTML = '<div class="loading"><div class="spinner"></div><span>Loading...</span></div>';
  errorEl.style.display = "none";

  try {
    const result = await app.callTool("qsv_browse_directory", { directory: path });
    const text = result?.content?.find(c => c.type === "text")?.text;
    if (!text) throw new Error("No response from server");
    const data = JSON.parse(text);

    currentPath = data.currentPath || path;
    renderBreadcrumbs(currentPath);
    renderDirList(data);
    updateSelectedState(currentPath);
  } catch (err) {
    showError("Failed to browse: " + (err.message || err));
    dirListEl.innerHTML = '<div class="empty-state">Failed to load directory</div>';
  }
}

function renderQuickAccess(dirs) {
  quickAccessEl.innerHTML = "";
  for (const dir of dirs) {
    const btn = document.createElement("button");
    btn.className = "quick-btn";
    btn.dataset.path = dir.path;
    btn.textContent = dir.label;
    btn.onclick = () => navigate(dir.path);
    quickAccessEl.appendChild(btn);
  }
}

// Select button handler
selectBtn.onclick = async () => {
  if (!selectedPath) return;
  selectBtn.disabled = true;
  selectBtn.textContent = "Setting...";

  try {
    const result = await app.callTool("qsv_set_working_dir", { directory: selectedPath });
    const text = result?.content?.find(c => c.type === "text")?.text || "";
    if (text.toLowerCase().includes("error")) {
      showError(text);
      selectBtn.disabled = false;
      selectBtn.textContent = "Select This Directory";
    } else {
      showSuccess("Working directory set to: " + selectedPath);
    }
  } catch (err) {
    showError("Failed to set directory: " + (err.message || err));
    selectBtn.disabled = false;
    selectBtn.textContent = "Select This Directory";
  }
};

// Apply host theming
function applyTheme(ctx) {
  if (ctx?.theme) applyDocumentTheme(ctx.theme);
  if (ctx?.styles?.variables) applyHostStyleVariables(ctx.styles.variables);
}

app.onhostcontextchanged = (ctx) => applyTheme(ctx);

// Handle tool input (initial data passed by the server)
app.ontoolinput = (input) => {
  const args = input?.arguments;
  if (args?.currentPath) currentPath = args.currentPath;
  if (args?.knownDirs) {
    knownDirs = args.knownDirs;
    renderQuickAccess(knownDirs);
  }
  if (currentPath) navigate(currentPath);
};

// Connect and initialize
app.connect().then(() => {
  const ctx = app.getHostContext();
  applyTheme(ctx);
}).catch(err => {
  showError("Failed to connect: " + (err.message || err));
});
</script>
</body>
</html>`;
}
