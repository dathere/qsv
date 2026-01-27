/**
 * Client Detection Utilities
 *
 * Detects MCP client type to auto-enable tool search features for known
 * Claude clients (Claude Desktop, Claude Code, Claude Cowork).
 */

import type { Implementation } from "@modelcontextprotocol/sdk/types.js";

/**
 * Known Claude client name patterns that support tool search
 * These clients can efficiently discover and use all 62+ qsv tools
 */
const TOOL_SEARCH_CLIENTS = [
  "claude-desktop",
  "claude-code",
  "claude-cowork",
  "claude", // Generic Claude client
];

/**
 * Client type enum for logging and analytics
 */
export type ClientType =
  | "claude-desktop"
  | "claude-code"
  | "claude-cowork"
  | "claude-generic"
  | "other"
  | "unknown";

/**
 * Check if the connected client supports tool search
 *
 * Tool search capability means the client can efficiently:
 * - Discover tools through search/filtering
 * - Handle large tool lists without performance issues
 * - Use deferred/lazy tool loading
 *
 * Known clients with tool search support:
 * - Claude Desktop
 * - Claude Code
 * - Claude Cowork
 *
 * @param clientInfo - Client implementation info from MCP SDK
 * @returns true if client supports tool search, false otherwise
 */
export function isToolSearchCapableClient(
  clientInfo?: Implementation,
): boolean {
  if (!clientInfo?.name) return false;

  // Normalize client name: lowercase and replace spaces with hyphens
  const clientName = clientInfo.name.toLowerCase().replace(/\s+/g, "-");

  // Only treat as tool-search-capable if the name matches a known pattern
  // exactly, or starts with the pattern (allowing suffixes like "-beta")
  // This prevents false positives like "myclaude-wrapper" matching "claude"
  return TOOL_SEARCH_CLIENTS.some(
    (pattern) => clientName === pattern || clientName.startsWith(`${pattern}-`),
  );
}

/**
 * Get the type of connected client
 *
 * Uses strict pattern matching to avoid misclassification.
 * Client name must start with "claude" to be recognized as a Claude client.
 *
 * @param clientInfo - Client implementation info from MCP SDK
 * @returns Client type for logging and analytics
 */
export function getClientType(clientInfo?: Implementation): ClientType {
  if (!clientInfo?.name) return "unknown";

  // Normalize: lowercase and replace spaces with hyphens
  const name = clientInfo.name.toLowerCase().replace(/\s+/g, "-");

  // Must start with 'claude' to be recognized as a Claude client
  // This prevents false positives like "my-desktop-app-claude"
  if (!name.startsWith("claude")) return "other";

  // Check for specific Claude client types using prefix matching
  // "claude-desktop" or "claude-desktop-beta" → claude-desktop
  if (name === "claude-desktop" || name.startsWith("claude-desktop-")) {
    return "claude-desktop";
  }
  if (name === "claude-code" || name.startsWith("claude-code-")) {
    return "claude-code";
  }
  if (name === "claude-cowork" || name.startsWith("claude-cowork-")) {
    return "claude-cowork";
  }

  // Generic Claude client (e.g., "claude", "claude-client", "claude-app")
  return "claude-generic";
}

/**
 * Format client info for logging
 *
 * @param clientInfo - Client implementation info from MCP SDK
 * @returns Human-readable client description
 */
export function formatClientInfo(clientInfo?: Implementation): string {
  if (!clientInfo) return "unknown client";

  // Handle missing or empty name
  if (!clientInfo.name || clientInfo.name.trim() === "") {
    return "unknown client";
  }

  const parts: string[] = [clientInfo.name];

  // Only add version if it's a non-empty string
  if (clientInfo.version && clientInfo.version.trim() !== "") {
    parts.push(`v${clientInfo.version}`);
  }

  return parts.join(" ");
}
