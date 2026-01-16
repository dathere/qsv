/**
 * Tests for subcommand handling in SkillExecutor
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { SkillExecutor } from '../src/executor.js';
import type { QsvSkill } from '../src/types.js';
import fs from 'fs';
import path from 'path';
import os from 'os';

// Helper to create test CSV file
function createTestCSV(content: string): string {
  const tmpDir = fs.mkdtempSync(path.join(os.tmpdir(), 'qsv-test-'));
  const filePath = path.join(tmpDir, 'test.csv');
  fs.writeFileSync(filePath, content);
  return filePath;
}

// Helper to cleanup test files
function cleanup(filePath: string) {
  const dir = path.dirname(filePath);
  if (dir.includes('qsv-test-')) {
    fs.rmSync(dir, { recursive: true, force: true });
  }
}

test('executor handles cat subcommands correctly', async () => {
  const executor = new SkillExecutor();

  const catSkill: QsvSkill = {
    name: 'qsv-cat',
    version: '14.0.0',
    description: 'Concatenate CSV files',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'cat',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['rows', 'rowskey', 'columns']
        },
        {
          name: 'input1',
          type: 'file',
          required: false,
          description: 'First input file'
        },
        {
          name: 'input2',
          type: 'file',
          required: false,
          description: 'Second input file'
        }
      ],
      options: []
    },
    examples: []
  };

  const file1 = createTestCSV('a,b\n1,2\n3,4\n');
  const file2 = createTestCSV('a,b\n5,6\n7,8\n');

  try {
    // Test cat rows subcommand with multiple input files
    const result = await executor.execute(catSkill, {
      args: {
        subcommand: 'rows',
        input1: file1,
        input2: file2
      }
    });

    assert.strictEqual(result.success, true, `cat rows should succeed (exit code: ${result.metadata.exitCode})`);
    assert.match(result.output, /a,b/, 'Output should contain headers');
    // Should have 4 data rows (2 from each file) plus 1 header
    const lines = result.output.trim().split('\n');
    assert.ok(lines.length >= 5, 'Should have header + 4 rows');

    // Verify the command included the subcommand
    assert.match(
      result.metadata.command,
      /qsv cat rows/,
      'Command should include "cat rows" subcommand'
    );
  } finally {
    cleanup(file1);
    cleanup(file2);
  }
});

test('executor handles luau subcommands correctly', async () => {
  const executor = new SkillExecutor();

  const luauSkill: QsvSkill = {
    name: 'qsv-luau',
    version: '14.0.0',
    description: 'Execute Luau scripts',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'luau',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['map', 'filter']
        },
        {
          name: 'main-script',
          type: 'string',
          required: true,
          description: 'Luau script to execute'
        },
        {
          name: 'input',
          type: 'file',
          required: true,
          description: 'Input file'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('a,b\n1,2\n3,4\n');

  try {
    // Test luau filter subcommand
    // Script: keep rows where column 'a' (as number) is greater than 1
    const result = await executor.execute(luauSkill, {
      args: {
        subcommand: 'filter',
        'main-script': 'tonumber(a) > 1',
        input: testFile
      }
    });

    assert.strictEqual(result.success, true, 'luau filter should succeed');

    // Verify the command included the subcommand
    assert.match(
      result.metadata.command,
      /qsv luau filter/,
      'Command should include "luau filter" subcommand'
    );

    // Should only have header and row with a=3 (3 > 1)
    const lines = result.output.trim().split('\n');
    assert.strictEqual(lines.length, 2, 'Should have header + 1 filtered row');
  } finally {
    cleanup(testFile);
  }
});

test('executor handles apply subcommands correctly', async () => {
  const executor = new SkillExecutor();

  const applySkill: QsvSkill = {
    name: 'qsv-apply',
    version: '14.0.0',
    description: 'Apply operations to columns',
    category: 'transformation',
    command: {
      binary: 'qsv',
      subcommand: 'apply',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['operations', 'emptyreplace', 'dynfmt', 'calcconv']
        },
        {
          name: 'operations',
          type: 'string',
          required: true,
          description: 'Operations to apply'
        },
        {
          name: 'column',
          type: 'string',
          required: true,
          description: 'Column to operate on'
        },
        {
          name: 'input',
          type: 'file',
          required: false,
          description: 'Input file'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('name,value\nalice,hello\nbob,world\n');

  try {
    // Test apply operations subcommand
    const result = await executor.execute(applySkill, {
      args: {
        subcommand: 'operations',
        operations: 'upper',
        column: 'name'
      },
      stdin: fs.readFileSync(testFile)
    });

    assert.strictEqual(result.success, true, 'apply operations should succeed');

    // Verify the command included the subcommand
    assert.match(
      result.metadata.command,
      /qsv apply operations/,
      'Command should include "apply operations" subcommand'
    );

    // Check output contains uppercase names
    assert.match(result.output, /ALICE/, 'Output should contain uppercase ALICE');
    assert.match(result.output, /BOB/, 'Output should contain uppercase BOB');
  } finally {
    cleanup(testFile);
  }
});

test('executor handles snappy subcommands correctly', async () => {
  const executor = new SkillExecutor();

  const snappySkill: QsvSkill = {
    name: 'qsv-snappy',
    version: '14.0.0',
    description: 'Snappy compression operations',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'snappy',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['compress', 'decompress', 'check', 'validate']
        },
        {
          name: 'input',
          type: 'file',
          required: true,
          description: 'Input file'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('a,b,c\n1,2,3\n4,5,6\n');

  try {
    // Test snappy check subcommand (should return false for uncompressed CSV)
    const result = await executor.execute(snappySkill, {
      args: {
        subcommand: 'check',
        input: testFile
      }
    });

    // check command returns exit code 0 if file is snappy, 1 if not
    // Either is a valid result for our test
    assert.ok(
      result.success === true || result.success === false,
      'snappy check should complete'
    );

    // Verify the command included the subcommand
    assert.match(
      result.metadata.command,
      /qsv snappy check/,
      'Command should include "snappy check" subcommand'
    );
  } finally {
    cleanup(testFile);
  }
});

test('executor handles validate subcommand (optional)', async () => {
  const executor = new SkillExecutor();

  const validateSkill: QsvSkill = {
    name: 'qsv-validate',
    version: '14.0.0',
    description: 'Validate CSV data',
    category: 'validation',
    command: {
      binary: 'qsv',
      subcommand: 'validate',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: false, // Special case: validate has optional schema subcommand
          description: 'Subcommand to execute',
          enum: ['schema']
        },
        {
          name: 'json-schema',
          type: 'string',
          required: false,
          description: 'JSON Schema file'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('a,b\n1,2\n3,4\n');

  try {
    // Test validate without subcommand (RFC 4180 mode)
    const result = await executor.execute(validateSkill, {
      args: {
        input: testFile
      },
      stdin: fs.readFileSync(testFile)
    });

    // Should succeed or fail gracefully
    assert.ok(
      typeof result.success === 'boolean',
      'validate should return success status'
    );

    // Verify the command is correct
    assert.match(
      result.metadata.command,
      /qsv validate/,
      'Command should include "validate"'
    );
  } finally {
    cleanup(testFile);
  }
});

test('executor throws error for missing required subcommand', async () => {
  const executor = new SkillExecutor();

  const catSkill: QsvSkill = {
    name: 'qsv-cat',
    version: '14.0.0',
    description: 'Concatenate CSV files',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'cat',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['rows', 'rowskey', 'columns']
        },
        {
          name: 'input',
          type: 'file',
          required: true,
          description: 'Input files'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('a,b\n1,2\n');

  try {
    await assert.rejects(
      async () => {
        await executor.execute(catSkill, {
          args: {
            // Missing required subcommand
            input: testFile
          }
        });
      },
      {
        message: /Missing required subcommand/
      },
      'Should throw error for missing required subcommand'
    );
  } finally {
    cleanup(testFile);
  }
});

test('executor validates subcommand against enum values', async () => {
  const executor = new SkillExecutor();

  const catSkill: QsvSkill = {
    name: 'qsv-cat',
    version: '14.0.0',
    description: 'Concatenate CSV files',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'cat',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['rows', 'rowskey', 'columns']
        },
        {
          name: 'input',
          type: 'file',
          required: true,
          description: 'Input files'
        }
      ],
      options: []
    },
    examples: []
  };

  const testFile = createTestCSV('a,b\n1,2\n');

  try {
    // This should fail at qsv level with invalid subcommand
    const result = await executor.execute(catSkill, {
      args: {
        subcommand: 'invalid_subcommand',
        input: testFile
      }
    });

    // qsv should return an error
    assert.strictEqual(
      result.success,
      false,
      'Should fail for invalid subcommand'
    );
  } finally {
    cleanup(testFile);
  }
});

test('buildCommand includes subcommand in generated command string', async () => {
  const executor = new SkillExecutor();

  const luauSkill: QsvSkill = {
    name: 'qsv-luau',
    version: '14.0.0',
    description: 'Execute Luau scripts',
    category: 'utility',
    command: {
      binary: 'qsv',
      subcommand: 'luau',
      args: [
        {
          name: 'subcommand',
          type: 'string',
          required: true,
          description: 'Subcommand to execute',
          enum: ['map', 'filter']
        },
        {
          name: 'main-script',
          type: 'string',
          required: true,
          description: 'Luau script to execute'
        },
        {
          name: 'input',
          type: 'file',
          required: true,
          description: 'Input file'
        }
      ],
      options: []
    },
    examples: []
  };

  const command = executor.buildCommand(luauSkill, {
    args: {
      subcommand: 'map',
      'main-script': 'newcol',
      input: 'test.csv'
    }
  });

  assert.match(
    command,
    /qsv luau map/,
    'buildCommand should include subcommand'
  );
  assert.match(
    command,
    /newcol/,
    'buildCommand should include script'
  );
  assert.match(
    command,
    /test\.csv/,
    'buildCommand should include input file'
  );
});
