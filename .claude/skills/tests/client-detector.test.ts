/**
 * Unit tests for client detection utilities
 */

import { test, describe } from 'node:test';
import assert from 'node:assert';
import {
  isToolSearchCapableClient,
  getClientType,
  formatClientInfo,
  type ClientType,
} from '../src/client-detector.js';

describe('isToolSearchCapableClient', () => {
  test('returns true for Claude Desktop client', () => {
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-desktop', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'Claude Desktop', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'CLAUDE-DESKTOP', version: '1.0.0' }), true);
  });

  test('returns true for Claude Code client', () => {
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-code', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'Claude Code', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'CLAUDE-CODE', version: '1.0.0' }), true);
  });

  test('returns true for Claude Cowork client', () => {
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-cowork', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'Claude Cowork', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'CLAUDE-COWORK', version: '1.0.0' }), true);
  });

  test('returns true for generic Claude client', () => {
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'Claude', version: '1.0.0' }), true);
    // Suffix variants should work (e.g., "claude-beta", "claude-internal")
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-beta', version: '1.0.0' }), true);
  });

  test('returns true for Claude client suffix variants', () => {
    // Suffix variants of known clients should work
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-desktop-beta', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-code-internal', version: '1.0.0' }), true);
    assert.strictEqual(isToolSearchCapableClient({ name: 'claude-cowork-dev', version: '1.0.0' }), true);
  });

  test('returns false for unknown clients', () => {
    assert.strictEqual(isToolSearchCapableClient({ name: 'cursor', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'vscode', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'generic-mcp-client', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'my-custom-client', version: '1.0.0' }), false);
  });

  test('returns false for clients with claude substring but not matching pattern', () => {
    // These should NOT match - they contain "claude" but don't start with known patterns
    assert.strictEqual(isToolSearchCapableClient({ name: 'myclaude-wrapper', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'not-claude-but-has-it', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'my-claude-app', version: '1.0.0' }), false);
    assert.strictEqual(isToolSearchCapableClient({ name: 'vscode-claude-plugin', version: '1.0.0' }), false);
  });

  test('returns false for undefined or missing client info', () => {
    assert.strictEqual(isToolSearchCapableClient(undefined), false);
    assert.strictEqual(isToolSearchCapableClient({} as any), false);
    assert.strictEqual(isToolSearchCapableClient({ name: '', version: '1.0.0' }), false);
  });
});

describe('getClientType', () => {
  test('detects Claude Desktop client', () => {
    assert.strictEqual(getClientType({ name: 'claude-desktop', version: '1.0.0' }), 'claude-desktop');
    assert.strictEqual(getClientType({ name: 'Claude Desktop', version: '1.0.0' }), 'claude-desktop');
    assert.strictEqual(getClientType({ name: 'CLAUDE-DESKTOP', version: '1.0.0' }), 'claude-desktop');
  });

  test('detects Claude Code client', () => {
    assert.strictEqual(getClientType({ name: 'claude-code', version: '1.0.0' }), 'claude-code');
    assert.strictEqual(getClientType({ name: 'Claude Code', version: '1.0.0' }), 'claude-code');
    assert.strictEqual(getClientType({ name: 'CLAUDE-CODE', version: '1.0.0' }), 'claude-code');
  });

  test('detects Claude Cowork client', () => {
    assert.strictEqual(getClientType({ name: 'claude-cowork', version: '1.0.0' }), 'claude-cowork');
    assert.strictEqual(getClientType({ name: 'Claude Cowork', version: '1.0.0' }), 'claude-cowork');
    assert.strictEqual(getClientType({ name: 'CLAUDE-COWORK', version: '1.0.0' }), 'claude-cowork');
  });

  test('detects generic Claude client', () => {
    assert.strictEqual(getClientType({ name: 'claude', version: '1.0.0' }), 'claude-generic');
    assert.strictEqual(getClientType({ name: 'Claude', version: '1.0.0' }), 'claude-generic');
    // Other claude- prefixed names are generic
    assert.strictEqual(getClientType({ name: 'claude-app', version: '1.0.0' }), 'claude-generic');
    assert.strictEqual(getClientType({ name: 'claude-client', version: '1.0.0' }), 'claude-generic');
  });

  test('detects Claude client suffix variants', () => {
    // Suffix variants should be classified correctly
    assert.strictEqual(getClientType({ name: 'claude-desktop-beta', version: '1.0.0' }), 'claude-desktop');
    assert.strictEqual(getClientType({ name: 'claude-code-internal', version: '1.0.0' }), 'claude-code');
    assert.strictEqual(getClientType({ name: 'claude-cowork-dev', version: '1.0.0' }), 'claude-cowork');
  });

  test('returns other for unknown clients', () => {
    assert.strictEqual(getClientType({ name: 'cursor', version: '1.0.0' }), 'other');
    assert.strictEqual(getClientType({ name: 'vscode', version: '1.0.0' }), 'other');
    assert.strictEqual(getClientType({ name: 'generic-mcp-client', version: '1.0.0' }), 'other');
  });

  test('returns other for clients with claude substring but not starting with claude', () => {
    // These should NOT be classified as Claude clients
    assert.strictEqual(getClientType({ name: 'myclaude-wrapper', version: '1.0.0' }), 'other');
    assert.strictEqual(getClientType({ name: 'my-claude-app', version: '1.0.0' }), 'other');
    assert.strictEqual(getClientType({ name: 'vscode-claude-plugin', version: '1.0.0' }), 'other');
    assert.strictEqual(getClientType({ name: 'my-desktop-app-claude', version: '1.0.0' }), 'other');
  });

  test('returns unknown for undefined or missing client info', () => {
    assert.strictEqual(getClientType(undefined), 'unknown');
    assert.strictEqual(getClientType({} as any), 'unknown');
    assert.strictEqual(getClientType({ name: '', version: '1.0.0' }), 'unknown');
  });
});

describe('formatClientInfo', () => {
  test('formats client with name and version', () => {
    assert.strictEqual(formatClientInfo({ name: 'claude-desktop', version: '1.0.0' }), 'claude-desktop v1.0.0');
    assert.strictEqual(formatClientInfo({ name: 'Claude Code', version: '2.5.3' }), 'Claude Code v2.5.3');
  });

  test('formats client with only name', () => {
    assert.strictEqual(formatClientInfo({ name: 'claude-desktop' } as any), 'claude-desktop');
    assert.strictEqual(formatClientInfo({ name: 'Claude Code', version: '' }), 'Claude Code');
  });

  test('returns unknown client for undefined or missing info', () => {
    assert.strictEqual(formatClientInfo(undefined), 'unknown client');
    assert.strictEqual(formatClientInfo({} as any), 'unknown client');
    assert.strictEqual(formatClientInfo({ name: '', version: '1.0.0' }), 'unknown client');
  });
});
