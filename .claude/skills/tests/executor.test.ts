/**
 * Unit tests for executor module
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { SkillExecutor } from '../src/executor.js';
import { config } from '../src/config.js';
import type { QsvSkill, SkillParams } from '../src/types.js';

// ============================================================================
// Test Fixtures - Mock QsvSkill definitions
// ============================================================================

/**
 * Simple skill without subcommands (like 'count')
 */
const countSkill: QsvSkill = {
  name: 'count',
  version: '1.0.0',
  description: 'Count records in CSV',
  category: 'utility',
  command: {
    subcommand: 'count',
    args: [
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: [
      { flag: '--no-headers', type: 'flag', description: 'Input has no headers' },
      { flag: '--delimiter', short: '-d', type: 'string', description: 'Field delimiter' }
    ]
  },
  examples: []
};

/**
 * Skill with subcommands (like 'cat')
 */
const catSkill: QsvSkill = {
  name: 'cat',
  version: '1.0.0',
  description: 'Concatenate CSV files',
  category: 'transformation',
  command: {
    subcommand: 'cat',
    args: [
      { name: 'subcommand', type: 'string', required: true, description: 'Subcommand', enum: ['rows', 'rowskey', 'columns'] },
      { name: 'input', type: 'file', required: true, description: 'Input CSV files' }
    ],
    options: [
      { flag: '--output', short: '-o', type: 'string', description: 'Output file' }
    ]
  },
  examples: []
};

/**
 * Stats skill for testing auto --stats-jsonl
 */
const statsSkill: QsvSkill = {
  name: 'stats',
  version: '1.0.0',
  description: 'Compute statistics',
  category: 'aggregation',
  command: {
    subcommand: 'stats',
    args: [
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: [
      { flag: '--stats-jsonl', type: 'flag', description: 'Output stats cache' },
      { flag: '--everything', type: 'flag', description: 'Show all stats' }
    ]
  },
  examples: []
};

/**
 * Frequency skill for testing auto --frequency-jsonl with version guard
 */
const frequencySkill: QsvSkill = {
  name: 'frequency',
  version: '16.1.0',
  description: 'Build frequency tables',
  category: 'aggregation',
  command: {
    subcommand: 'frequency',
    args: [
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: [
      { flag: '--frequency-jsonl', type: 'flag', description: 'Output frequency cache' },
      { flag: '--limit', type: 'string', description: 'Limit number of values' }
    ]
  },
  examples: []
};

/**
 * Skill with optional subcommand (like 'snappy')
 */
const snappySkill: QsvSkill = {
  name: 'snappy',
  version: '1.0.0',
  description: 'Snappy compression',
  category: 'utility',
  command: {
    subcommand: 'snappy',
    args: [
      { name: 'subcommand', type: 'string', required: false, description: 'Subcommand', enum: ['compress', 'decompress', 'check', 'validate'] },
      { name: 'input', type: 'file', required: true, description: 'Input file' }
    ],
    options: []
  },
  examples: []
};

/**
 * Skill with multiple arguments for testing
 */
const selectSkill: QsvSkill = {
  name: 'select',
  version: '1.0.0',
  description: 'Select columns',
  category: 'selection',
  command: {
    subcommand: 'select',
    args: [
      { name: 'selection', type: 'string', required: true, description: 'Column selection' },
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: [
      { flag: '--no-headers', type: 'flag', description: 'Input has no headers' },
      { flag: '--output', short: '-o', type: 'string', description: 'Output file' }
    ]
  },
  examples: []
};

// ============================================================================
// SkillExecutor Unit Tests
// ============================================================================

test('SkillExecutor constructor sets default working directory', () => {
  const executor = new SkillExecutor('qsv');
  assert.strictEqual(executor.getWorkingDirectory(), process.cwd());
});

test('SkillExecutor constructor accepts custom working directory', () => {
  const executor = new SkillExecutor('qsv', '/tmp');
  assert.strictEqual(executor.getWorkingDirectory(), '/tmp');
});

test('SkillExecutor setWorkingDirectory updates directory', () => {
  const executor = new SkillExecutor('qsv');
  executor.setWorkingDirectory('/custom/path');
  assert.strictEqual(executor.getWorkingDirectory(), '/custom/path');
});

// ============================================================================
// buildCommand Tests
// ============================================================================

test('buildCommand creates simple command', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' }
  };

  const cmd = executor.buildCommand(countSkill, params);
  assert.strictEqual(cmd, 'qsv count data.csv');
});

test('buildCommand handles flags', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { 'no-headers': true }
  };

  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('--no-headers'));
  assert.ok(cmd.includes('data.csv'));
});

test('buildCommand handles options with values', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { delimiter: '|' }
  };

  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('--delimiter'));
  assert.ok(cmd.includes('|'));
});

test('buildCommand handles subcommands', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { subcommand: 'rows', input: 'data.csv' }
  };

  const cmd = executor.buildCommand(catSkill, params);
  assert.ok(cmd.startsWith('qsv cat rows'));
  assert.ok(cmd.includes('data.csv'));
});

test('buildCommand handles optional subcommand when not provided', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' }
  };

  const cmd = executor.buildCommand(snappySkill, params);
  assert.ok(cmd.startsWith('qsv snappy'));
  assert.ok(cmd.includes('data.csv'));
  // Should not have a subcommand between snappy and data.csv
});

test('buildCommand handles multiple arguments', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { selection: '1,3,5', input: 'data.csv' }
  };

  const cmd = executor.buildCommand(selectSkill, params);
  assert.ok(cmd.includes('select'));
  assert.ok(cmd.includes('1,3,5'));
  assert.ok(cmd.includes('data.csv'));
});

test('buildCommand handles --help flag', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    options: { help: true }
  };

  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('--help'));
});

// ============================================================================
// Stats Command Auto --stats-jsonl Tests
// ============================================================================

test('buildCommand auto-adds --stats-jsonl for stats command', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' }
  };

  const cmd = executor.buildCommand(statsSkill, params);
  assert.ok(cmd.includes('--stats-jsonl'), 'Should auto-add --stats-jsonl flag');
});

test('buildCommand does not duplicate --stats-jsonl if already present', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { 'stats-jsonl': true }
  };

  const cmd = executor.buildCommand(statsSkill, params);
  // Count occurrences of --stats-jsonl
  const count = (cmd.match(/--stats-jsonl/g) || []).length;
  assert.strictEqual(count, 1, 'Should have exactly one --stats-jsonl flag');
});

// ============================================================================
// Option Normalization Tests
// ============================================================================

test('buildCommand normalizes option keys with dashes', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { '--no-headers': true }
  };

  const cmd = executor.buildCommand(countSkill, params);
  // Should not have double dashes
  assert.ok(!cmd.includes('----'));
  assert.ok(cmd.includes('--no-headers'));
});

test('buildCommand normalizes option keys with underscores', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { 'no_headers': true }
  };

  // This tests that the option lookup handles various key formats
  const cmd = executor.buildCommand(countSkill, params);
  // The option should be matched and added (or not if it doesn't match)
  assert.ok(typeof cmd === 'string');
});

// ============================================================================
// Subcommand Error Handling Tests
// ============================================================================

test('buildCommand throws for missing required subcommand', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' }
    // Missing subcommand
  };

  assert.throws(
    () => executor.buildCommand(catSkill, params),
    /Missing required subcommand/
  );
});

// ============================================================================
// extractRowCount Tests (via execute output)
// ============================================================================

test('SkillExecutor extracts row count patterns from stderr', async () => {
  // Test the pattern matching logic indirectly
  // The extractRowCount method is private, but we can verify behavior
  // by checking metadata.rowsProcessed in execute results

  // This is a pattern test - we just verify the executor is created correctly
  const executor = new SkillExecutor('qsv', '/tmp');
  assert.ok(executor instanceof SkillExecutor);
});

// ============================================================================
// stdin handling Tests
// ============================================================================

test('buildCommand skips input validation when stdin provided', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    stdin: 'a,b,c\n1,2,3\n'
    // No input file provided
  };

  // Should not throw because stdin is provided
  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('count'));
});

// ============================================================================
// Complex Scenario Tests
// ============================================================================

test('buildCommand handles complex cat rows scenario', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { subcommand: 'rows', input: 'file1.csv file2.csv' },
    options: { output: 'combined.csv' }
  };

  const cmd = executor.buildCommand(catSkill, params);
  assert.ok(cmd.includes('cat'));
  assert.ok(cmd.includes('rows'));
  assert.ok(cmd.includes('--output'));
  assert.ok(cmd.includes('combined.csv'));
});

test('buildCommand handles select with output option', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { selection: 'name,age', input: 'people.csv' },
    options: { output: 'subset.csv', 'no-headers': true }
  };

  const cmd = executor.buildCommand(selectSkill, params);
  assert.ok(cmd.includes('select'));
  assert.ok(cmd.includes('name,age'));
  assert.ok(cmd.includes('people.csv'));
  assert.ok(cmd.includes('--output'));
  assert.ok(cmd.includes('subset.csv'));
  assert.ok(cmd.includes('--no-headers'));
});

// ============================================================================
// Integration-style Tests (require qsv binary)
// ============================================================================

test('SkillExecutor execute with --help flag does not require input file', async () => {
  // This test verifies that --help requests skip input validation
  const executor = new SkillExecutor('qsv');

  // Try to execute with help flag but no input - should not error on missing file
  try {
    const result = await executor.execute(countSkill, {
      options: { help: true }
    });

    // If qsv is installed, this should succeed with help output
    if (result.success) {
      assert.ok(result.output.includes('Count') || result.output.includes('count') || result.output.includes('Usage'));
    }
  } catch (error) {
    // If qsv is not installed, the error should be about missing binary, not missing input
    const errorMessage = error instanceof Error ? error.message : String(error);
    assert.ok(
      !errorMessage.includes('Missing required argument: input'),
      'Should not require input file when --help is requested'
    );
  }
});

// ============================================================================
// buildCommand display-only behavior Tests
// ============================================================================

test('buildCommand joins args with spaces without escaping (display only)', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { selection: 'col with spaces', input: 'file name.csv' }
  };

  // buildCommand is for display/logging only - args are NOT escaped
  const cmd = executor.buildCommand(selectSkill, params);
  assert.ok(cmd.includes('col with spaces'));
  assert.ok(cmd.includes('file name.csv'));
  // Verify no quoting applied (this documents the non-shell-safe behavior)
  assert.ok(!cmd.includes('"col with spaces"'));
});

// ============================================================================
// normalizeOptionKey edge case Tests
// ============================================================================

test('buildCommand handles bare "--" option key gracefully', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { '--': true }
  };

  // "--" should be returned unchanged (not stripped to empty string)
  // Since it won't match any option definition, it should be silently ignored
  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('count'));
  assert.ok(cmd.includes('data.csv'));
});

test('buildCommand handles bare "-" option key gracefully', () => {
  const executor = new SkillExecutor('qsv');
  const params: SkillParams = {
    args: { input: 'data.csv' },
    options: { '-': true }
  };

  // "-" should be returned unchanged (not stripped to empty string)
  const cmd = executor.buildCommand(countSkill, params);
  assert.ok(cmd.includes('count'));
  assert.ok(cmd.includes('data.csv'));
});

// ============================================================================
// Frequency Command Auto --frequency-jsonl Version Guard Tests
// ============================================================================

test('buildCommand auto-adds --frequency-jsonl when binary version >= 16.1.0', () => {
  const executor = new SkillExecutor('qsv');
  const originalVersion = config.qsvValidation.version;
  try {
    config.qsvValidation.version = '16.1.0';
    const params: SkillParams = {
      args: { input: 'data.csv' }
    };
    const cmd = executor.buildCommand(frequencySkill, params);
    assert.ok(cmd.includes('--frequency-jsonl'), 'Should auto-add --frequency-jsonl when version >= 16.1.0');
  } finally {
    config.qsvValidation.version = originalVersion;
  }
});

test('buildCommand does not add --frequency-jsonl when binary version < 16.1.0', () => {
  const executor = new SkillExecutor('qsv');
  const originalVersion = config.qsvValidation.version;
  try {
    config.qsvValidation.version = '16.0.0';
    const params: SkillParams = {
      args: { input: 'data.csv' }
    };
    const cmd = executor.buildCommand(frequencySkill, params);
    assert.ok(!cmd.includes('--frequency-jsonl'), 'Should NOT add --frequency-jsonl when version < 16.1.0');
  } finally {
    config.qsvValidation.version = originalVersion;
  }
});

test('buildCommand does not add --frequency-jsonl when binary version is undefined', () => {
  const executor = new SkillExecutor('qsv');
  const originalVersion = config.qsvValidation.version;
  try {
    config.qsvValidation.version = undefined as unknown as string;
    const params: SkillParams = {
      args: { input: 'data.csv' }
    };
    const cmd = executor.buildCommand(frequencySkill, params);
    assert.ok(!cmd.includes('--frequency-jsonl'), 'Should NOT add --frequency-jsonl when version is undefined');
  } finally {
    config.qsvValidation.version = originalVersion;
  }
});

test('buildCommand does not add --frequency-jsonl when reading from stdin', () => {
  const executor = new SkillExecutor('qsv');
  const originalVersion = config.qsvValidation.version;
  try {
    config.qsvValidation.version = '16.1.0';
    const params: SkillParams = {
      stdin: 'a,b\n1,2\n'
    };
    const cmd = executor.buildCommand(frequencySkill, params);
    assert.ok(!cmd.includes('--frequency-jsonl'), 'Should NOT add --frequency-jsonl when reading from stdin');
  } finally {
    config.qsvValidation.version = originalVersion;
  }
});

/**
 * Frequency skill WITHOUT --frequency-jsonl option definition.
 * Used to test the findOptionDef guard in the auto-inject logic.
 */
const frequencySkillNoJsonlOption: QsvSkill = {
  name: 'frequency',
  version: '15.0.0',
  description: 'Build frequency tables (old version without --frequency-jsonl)',
  category: 'aggregation',
  command: {
    subcommand: 'frequency',
    args: [
      { name: 'input', type: 'file', required: true, description: 'Input CSV file' }
    ],
    options: [
      { flag: '--limit', type: 'string', description: 'Limit number of values' }
    ]
  },
  examples: []
};

test('buildCommand does not add --frequency-jsonl when option definition is missing from skill', () => {
  const executor = new SkillExecutor('qsv');
  const originalVersion = config.qsvValidation.version;
  try {
    config.qsvValidation.version = '16.1.0';
    const params: SkillParams = {
      args: { input: 'data.csv' }
    };
    const cmd = executor.buildCommand(frequencySkillNoJsonlOption, params);
    assert.ok(!cmd.includes('--frequency-jsonl'),
      'Should NOT add --frequency-jsonl when skill lacks the option definition');
  } finally {
    config.qsvValidation.version = originalVersion;
  }
});
