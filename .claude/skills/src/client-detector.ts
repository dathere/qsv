/**
 * Client Detection Utilities
 *
 * Detects MCP client type to auto-enable tool search features for known
 * Claude clients (Claude Desktop, Claude Code, Claude Cowork).
 */

import type { Implementation } from '@modelcontextprotocol/sdk/types.js';

/**
 * Known Claude client name patterns that support tool search
 * These clients can efficiently discover and use all 62+ qsv tools
 */
const TOOL_SEARCH_CLIENTS = [
  'claude-desktop',
  'claude-code',
  'claude-cowork',
  'claude',  // Generic Claude client
];

/**
 * Client type enum for logging and analytics
 */
export type ClientType =
  | 'claude-desktop'
  | 'claude-code'
  | 'claude-cowork'
  | 'claude-generic'
  | 'other'
  | 'unknown';

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
export function isToolSearchCapableClient(clientInfo?: Implementation): boolean {
  if (!clientInfo?.name) return false;

  // Normalize client name: lowercase and replace spaces with hyphens
  const clientName = clientInfo.name.toLowerCase().replace(/\s+/g, '-');

  return TOOL_SEARCH_CLIENTS.some(pattern => clientName.includes(pattern));
}

/**
 * Get the type of connected client
 *
 * @param clientInfo - Client implementation info from MCP SDK
 * @returns Client type for logging and analytics
 */
export function getClientType(clientInfo?: Implementation): ClientType {
  if (!clientInfo?.name) return 'unknown';

  const name = clientInfo.name.toLowerCase();

  // Must contain 'claude' to be recognized as a Claude client
  if (!name.includes('claude')) return 'other';

  // Now check for specific Claude client types
  if (name.includes('desktop')) return 'claude-desktop';
  if (name.includes('code')) return 'claude-code';
  if (name.includes('cowork')) return 'claude-cowork';

  // Generic Claude client
  return 'claude-generic';
}

/**
 * Format client info for logging
 *
 * @param clientInfo - Client implementation info from MCP SDK
 * @returns Human-readable client description
 */
export function formatClientInfo(clientInfo?: Implementation): string {
  if (!clientInfo) return 'unknown client';

  // Handle missing or empty name
  if (!clientInfo.name || clientInfo.name.trim() === '') {
    return 'unknown client';
  }

  const parts: string[] = [clientInfo.name];

  // Only add version if it's a non-empty string
  if (clientInfo.version && clientInfo.version.trim() !== '') {
    parts.push(`v${clientInfo.version}`);
  }

  return parts.join(' ');
}
