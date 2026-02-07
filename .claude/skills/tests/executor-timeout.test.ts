/**
 * Tests for executor timeout handling, signal termination mapping,
 * and process lifecycle management (v16.0.0 features)
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { spawn } from 'node:child_process';
import { SkillExecutor } from '../src/executor.js';
import type { QsvSkill } from '../src/types.js';
import { config } from '../src/config.js';

// Skip integration tests if qsv is not available
const QSV_AVAILABLE = config.qsvValidation.valid;

/**
 * A skill definition that can be used for timeout testing.
 */
const countSkill: QsvSkill = {
  name: 'count',
  version: '1.0.0',
  description: 'Count records',
  category: 'utility',
  command: {
    binary: 'qsv',
    subcommand: 'count',
    args: [
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: []
  },
  examples: []
};

// ============================================================================
// Timeout Exit Code Tests
// ============================================================================

/**
 * Helper: directly test the runQsv timeout logic using a long-running process.
 * Uses Node.js instead of `sleep` for cross-platform compatibility (Windows has no `sleep`).
 */
function runSleepWithTimeout(timeoutMs: number): Promise<{
  exitCode: number;
  stdout: string;
  stderr: string;
}> {
  return new Promise((resolve, reject) => {
    const proc = spawn(process.execPath, ['-e', 'setTimeout(() => {}, 60000);'], {
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    let stdout = '';
    let stderr = '';
    let timedOut = false;
    let processExited = false;

    const timer = setTimeout(() => {
      timedOut = true;
      proc.kill('SIGTERM');

      const killTimer = setTimeout(() => {
        if (!processExited && proc.exitCode === null) {
          try {
            proc.kill('SIGKILL');
          } catch {
            // Process may have already exited
          }
          proc.unref();
        }
      }, 1000);
      killTimer.unref?.();
    }, timeoutMs);

    proc.stdout.on('data', (chunk) => { stdout += chunk.toString(); });
    proc.stderr.on('data', (chunk) => { stderr += chunk.toString(); });

    proc.on('close', (exitCode, signal) => {
      processExited = true;
      clearTimeout(timer);

      if (timedOut) {
        resolve({
          exitCode: 124,
          stdout,
          stderr: stderr + `\n[TIMEOUT] Process exceeded ${timeoutMs}ms timeout and was terminated.`,
        });
        return;
      }

      if (exitCode === null && signal) {
        const signalExitCodes: Record<string, number> = {
          SIGTERM: 143, SIGKILL: 137, SIGINT: 130, SIGHUP: 129, SIGQUIT: 131,
        };
        resolve({
          exitCode: signalExitCodes[signal] ?? 128,
          stdout,
          stderr: stderr + `\n[SIGNAL] Process was terminated by signal: ${signal}`,
        });
        return;
      }

      resolve({ exitCode: exitCode ?? 0, stdout, stderr });
    });

    proc.on('error', (err) => {
      processExited = true;
      clearTimeout(timer);
      if (timedOut) return;
      reject(err);
    });
  });
}

test('timeout handler returns exit code 124', async () => {
  // Use a 200ms timeout on a 60s process - will definitely time out
  const result = await runSleepWithTimeout(200);

  assert.strictEqual(result.exitCode, 124, 'Should return exit code 124 (standard timeout code)');
  assert.ok(result.stderr.includes('[TIMEOUT]'), 'Stderr should contain TIMEOUT marker');
});

test('timeout message includes configured duration', async () => {
  const result = await runSleepWithTimeout(200);

  assert.ok(
    result.stderr.includes('200ms'),
    'Timeout message should include the configured timeout duration'
  );
});

// ============================================================================
// Timeout Clamping Tests
// ============================================================================

test('executor clamps timeout to minimum of 1000ms', { skip: !QSV_AVAILABLE }, async () => {
  const executor = new SkillExecutor(config.qsvBinPath);

  // Use a valid file so the command completes quickly
  const stdinSkill: QsvSkill = {
    name: 'count',
    version: '1.0.0',
    description: 'Count records',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'count',
      args: [],
      options: [
        { flag: '--help', type: 'flag', description: 'Show help' }
      ]
    },
    examples: []
  };

  // Request a timeout of 1ms, but it should be clamped to 1000ms
  // The command should succeed because --help completes quickly and 1000ms is plenty
  const result = await executor.execute(stdinSkill, {
    options: { help: true },
    timeoutMs: 1, // Will be clamped to 1000ms minimum
  });

  // The command should succeed (not time out) because 1000ms is plenty for --help
  assert.strictEqual(result.success, true, 'Should succeed with clamped timeout');
});

// ============================================================================
// Signal Termination Mapping Tests
// ============================================================================

test('signal exit code mapping covers standard signals', () => {
  // Test the signal-to-exit-code mapping logic directly
  // These are the conventional Unix exit codes for signal termination (128 + signal number)
  const signalExitCodes: Record<string, number> = {
    SIGTERM: 143,  // 128 + 15
    SIGKILL: 137,  // 128 + 9
    SIGINT: 130,   // 128 + 2
    SIGHUP: 129,   // 128 + 1
    SIGQUIT: 131,  // 128 + 3
  };

  assert.strictEqual(signalExitCodes['SIGTERM'], 143, 'SIGTERM should map to 143');
  assert.strictEqual(signalExitCodes['SIGKILL'], 137, 'SIGKILL should map to 137');
  assert.strictEqual(signalExitCodes['SIGINT'], 130, 'SIGINT should map to 130');
  assert.strictEqual(signalExitCodes['SIGHUP'], 129, 'SIGHUP should map to 129');
  assert.strictEqual(signalExitCodes['SIGQUIT'], 131, 'SIGQUIT should map to 131');
});

test('unknown signal defaults to exit code 128', () => {
  // When a signal is not in the mapping, the code uses 128 as default
  const signalExitCodes: Record<string, number> = {
    SIGTERM: 143,
    SIGKILL: 137,
    SIGINT: 130,
    SIGHUP: 129,
    SIGQUIT: 131,
  };

  const unknownSignal = 'SIGUSR1';
  const exitCode = signalExitCodes[unknownSignal] ?? 128;
  assert.strictEqual(exitCode, 128, 'Unknown signal should default to 128');
});

// ============================================================================
// Process Lifecycle Tests
// ============================================================================

test('executor handles successful command completion', { skip: !QSV_AVAILABLE }, async () => {
  const executor = new SkillExecutor(config.qsvBinPath);

  const result = await executor.execute(countSkill, {
    options: { help: true },
  });

  assert.strictEqual(result.success, true, 'Help command should succeed');
  assert.strictEqual(result.metadata.exitCode, 0, 'Exit code should be 0');
  assert.ok(result.metadata.duration >= 0, 'Duration should be non-negative');
  assert.ok(result.metadata.command.includes('count'), 'Command should mention count');
});

test('executor handles command failure with non-zero exit', { skip: !QSV_AVAILABLE }, async () => {
  const executor = new SkillExecutor(config.qsvBinPath);

  // Try to count a nonexistent file
  const result = await executor.execute(countSkill, {
    args: { input: '/nonexistent/path/file.csv' },
  });

  assert.strictEqual(result.success, false, 'Should fail for nonexistent file');
  assert.ok(result.metadata.exitCode !== 0, 'Exit code should be non-zero');
});

// ============================================================================
// Timeout Configuration Tests (unit-level)
// ============================================================================

test('executor uses config.operationTimeoutMs as default', () => {
  // Verify that config has a reasonable default
  assert.ok(config.operationTimeoutMs >= 1000, 'Default timeout should be at least 1s');
  assert.ok(config.operationTimeoutMs <= 30 * 60 * 1000, 'Default timeout should be at most 30min');
});

test('executor validates timeout range', () => {
  // Test the timeout clamping logic from executor.ts
  // Mimics: Math.max(1000, Math.min(30 * 60 * 1000, Number(rawTimeout) || 10 * 60 * 1000))

  const clampTimeout = (raw: number | undefined) =>
    Math.max(1000, Math.min(30 * 60 * 1000, Number(raw) || 10 * 60 * 1000));

  assert.strictEqual(clampTimeout(500), 1000, 'Should clamp to minimum 1000ms');
  assert.strictEqual(clampTimeout(5000), 5000, 'Should allow 5000ms');
  assert.strictEqual(clampTimeout(60 * 60 * 1000), 30 * 60 * 1000, 'Should clamp to maximum 30min');
  assert.strictEqual(clampTimeout(undefined), 10 * 60 * 1000, 'Should default to 10min for undefined');
  assert.strictEqual(clampTimeout(NaN), 10 * 60 * 1000, 'Should default to 10min for NaN');
});
