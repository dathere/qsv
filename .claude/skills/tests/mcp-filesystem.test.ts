/**
 * Unit tests for filesystem resource provider
 */

import { test } from 'node:test';
import assert from 'node:assert';
import { FilesystemResourceProvider } from '../src/mcp-filesystem.js';
import { config } from '../src/config.js';
import { mkdtemp, writeFile, rmdir, unlink, mkdir, realpath, symlink } from 'fs/promises';
import { join } from 'path';
import { tmpdir, homedir } from 'os';

test('listFiles enforces file limit', async () => {
  // Create temporary directory
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  
  try {
    // Create more files than the limit
    const fileCount = config.maxFilesPerListing + 10;
    for (let i = 0; i < fileCount; i++) {
      await writeFile(join(tempDir, `test${i}.csv`), 'col1,col2\nval1,val2\n');
    }

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Pass undefined explicitly to use working directory
    const result = await provider.listFiles(undefined);
    
    // Should return exactly the limit, not more
    assert.strictEqual(result.resources.length, config.maxFilesPerListing);
  } finally {
    // Cleanup
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath prevents directory traversal', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  
  try {
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Should reject paths with ..
    await assert.rejects(
      async () => {
        await provider.resolvePath('../../etc/passwd');
      },
      /Access denied|outside allowed directories|Path does not exist/
    );

    // Should reject absolute paths outside allowed directories
    await assert.rejects(
      async () => {
        await provider.resolvePath('/etc/passwd');
      },
      /Access denied|outside allowed directories|Path does not exist/
    );
  } finally {
    try {
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('needsConversion detects Excel and JSONL formats', () => {
  const provider = new FilesystemResourceProvider();
  
  assert.strictEqual(provider.needsConversion('file.xlsx'), true);
  assert.strictEqual(provider.needsConversion('file.xls'), true);
  assert.strictEqual(provider.needsConversion('file.jsonl'), true);
  assert.strictEqual(provider.needsConversion('file.ndjson'), true);
  assert.strictEqual(provider.needsConversion('file.csv'), false);
  assert.strictEqual(provider.needsConversion('file.tsv'), false);
});

test('getConversionCommand returns correct command', () => {
  const provider = new FilesystemResourceProvider();

  assert.strictEqual(provider.getConversionCommand('file.xlsx'), 'excel');
  assert.strictEqual(provider.getConversionCommand('file.jsonl'), 'jsonl');
  assert.strictEqual(provider.getConversionCommand('file.csv'), null);
});

test('listFiles excludes converted files with UUID pattern', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a converted file (UUID pattern from ConvertedFileManager)
    await writeFile(
      join(tempDir, 'data.xlsx.converted.06488439-4c06-4abc-999e-9e6af19b1234.csv'),
      'col1,col2\nval1,val2\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the converted file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('listFiles excludes qsv-output temporary files', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a temporary output file
    await writeFile(
      join(tempDir, 'qsv-output-a1b2c3d4-e5f6-7890-abcd-ef1234567890.csv'),
      'col1,col2\nval1,val2\n'
    );

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the temp output file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('listFiles excludes files with .tmp. pattern', async () => {
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    // Create a normal CSV file
    await writeFile(join(tempDir, 'normal.csv'), 'col1,col2\nval1,val2\n');

    // Create a temp file with .tmp. pattern
    await writeFile(join(tempDir, 'data.tmp.csv'), 'col1,col2\nval1,val2\n');

    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    const result = await provider.listFiles(undefined);

    // Should only include the normal CSV, not the temp file
    assert.strictEqual(result.resources.length, 1);
    assert.strictEqual(result.resources[0].name, 'normal.csv');
  } finally {
    try {
      const { readdir } = await import('fs/promises');
      const files = await readdir(tempDir);
      for (const file of files) {
        await unlink(join(tempDir, file));
      }
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

// Note: listWorkingDirFiles was removed as file resources are no longer exposed via MCP.
// The qsv_list_files tool uses listFiles() which is still available.

test('setWorkingDirectory expands tilde to home directory', async () => {
  const home = homedir();

  // Create a provider with home directory in allowed directories
  const provider = new FilesystemResourceProvider({
    workingDirectory: home,
    allowedDirectories: [home],
  });

  // Set working directory using tilde
  provider.setWorkingDirectory('~');

  // Should resolve to home directory
  assert.strictEqual(provider.getWorkingDirectory(), home);
});

test('setWorkingDirectory expands ~/path to home directory subpath', async () => {
  const home = homedir();
  const tempSubdir = `qsv-tilde-test-${Date.now()}`;
  const tempPath = join(home, tempSubdir);

  try {
    // Create a test subdirectory in home
    await mkdir(tempPath);

    // Create a provider with home directory in allowed directories
    const provider = new FilesystemResourceProvider({
      workingDirectory: home,
      allowedDirectories: [home],
    });

    // Set working directory using ~/subdir syntax
    provider.setWorkingDirectory(`~/${tempSubdir}`);

    // Should resolve to the subdirectory under home
    assert.strictEqual(provider.getWorkingDirectory(), tempPath);
  } finally {
    try {
      await rmdir(tempPath);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath expands tilde in file paths', async () => {
  const home = homedir();
  const tempSubdir = `qsv-tilde-test-${Date.now()}`;
  const tempPath = join(home, tempSubdir);
  const testFile = 'test.csv';

  try {
    // Create a test subdirectory and file in home
    await mkdir(tempPath);
    await writeFile(join(tempPath, testFile), 'col1,col2\nval1,val2\n');

    // Create a provider with home directory in allowed directories
    const provider = new FilesystemResourceProvider({
      workingDirectory: home,
      allowedDirectories: [home],
    });

    // Resolve path using ~/subdir/file syntax
    const resolved = await provider.resolvePath(`~/${tempSubdir}/${testFile}`);

    // Should resolve to the full path under home
    assert.strictEqual(resolved, join(tempPath, testFile));
  } finally {
    try {
      await unlink(join(tempPath, testFile));
      await rmdir(tempPath);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath leaves absolute paths unchanged', async () => {
  // Use realpath to resolve symlinks (e.g., /var -> /private/var on macOS)
  // because resolvePath uses realpath internally
  const rawTempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  const tempDir = await realpath(rawTempDir);

  try {
    // Create a test file
    await writeFile(join(tempDir, 'test.csv'), 'col1,col2\nval1,val2\n');

    // workingDirectory is automatically added to allowedDirs by constructor
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Resolve an absolute path (no tilde)
    const resolved = await provider.resolvePath(join(tempDir, 'test.csv'));

    // Should be the same absolute path
    assert.strictEqual(resolved, join(tempDir, 'test.csv'));
  } finally {
    try {
      await unlink(join(tempDir, 'test.csv'));
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('setWorkingDirectory expands ~\\ (backslash) for Windows compatibility', async () => {
  const home = homedir();
  const tempSubdir = `qsv-tilde-test-${Date.now()}`;
  const tempPath = join(home, tempSubdir);

  try {
    // Create a test subdirectory in home
    await mkdir(tempPath);

    // Create a provider with home directory in allowed directories
    const provider = new FilesystemResourceProvider({
      workingDirectory: home,
      allowedDirectories: [home],
    });

    // Set working directory using ~\subdir syntax (Windows-style)
    provider.setWorkingDirectory(`~\\${tempSubdir}`);

    // Should resolve to the subdirectory under home
    assert.strictEqual(provider.getWorkingDirectory(), tempPath);
  } finally {
    try {
      await rmdir(tempPath);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath leaves relative paths unchanged (no tilde)', async () => {
  // Use realpath to resolve symlinks (e.g., /var -> /private/var on macOS)
  // because resolvePath uses realpath internally
  const rawTempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  const tempDir = await realpath(rawTempDir);

  try {
    // Create a test file
    await writeFile(join(tempDir, 'test.csv'), 'col1,col2\nval1,val2\n');

    // workingDirectory is automatically added to allowedDirs by constructor
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Resolve a relative path (no tilde)
    const resolved = await provider.resolvePath('test.csv');

    // Should resolve relative to working directory
    assert.strictEqual(resolved, join(tempDir, 'test.csv'));
  } finally {
    try {
      await unlink(join(tempDir, 'test.csv'));
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('resolvePath prevents cross-drive access on Windows', async () => {
  // This test verifies the fix for the Windows cross-drive vulnerability
  // where path.relative() returns an absolute path when paths are on different drives.
  // On Windows: relative("C:\\allowed", "D:\\malicious") returns "D:\\malicious" (absolute)
  // The fix rejects any path where the relative result is absolute.
  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));

  try {
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
    });

    // Use platform-appropriate absolute path that's definitely outside temp
    // On Windows this would be a different drive, on Unix it's /etc
    const outsidePath = process.platform === 'win32'
      ? 'C:\\Windows\\System32'
      : '/etc';

    await assert.rejects(
      async () => {
        await provider.resolvePath(outsidePath);
      },
      /Access denied|outside allowed directories|Path does not exist/
    );
  } finally {
    try {
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

// ============================================================================
// Symlink Canonicalization Tests
// ============================================================================

test('constructor resolves symlinks in workingDirectory', async () => {
  // Skip on Windows where symlinks require elevated privileges
  if (process.platform === 'win32') {
    return;
  }

  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  const symlinkPath = join(tmpdir(), `qsv-symlink-test-${Date.now()}`);

  try {
    // Create symlink to temp directory
    await symlink(tempDir, symlinkPath);

    // Create provider with symlink path
    const provider = new FilesystemResourceProvider({
      workingDirectory: symlinkPath,
    });

    // getWorkingDirectory should return the resolved real path, not the symlink
    const resolvedTempDir = await realpath(tempDir);
    assert.strictEqual(provider.getWorkingDirectory(), resolvedTempDir);
  } finally {
    try {
      await unlink(symlinkPath);
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});

test('constructor resolves symlinks in allowedDirectories', async () => {
  // Skip on Windows where symlinks require elevated privileges
  if (process.platform === 'win32') {
    return;
  }

  const tempDir = await mkdtemp(join(tmpdir(), 'qsv-test-'));
  const symlinkPath = join(tmpdir(), `qsv-symlink-test-${Date.now()}`);

  try {
    // Create symlink to temp directory
    await symlink(tempDir, symlinkPath);

    // Create a test file in the temp directory
    await writeFile(join(tempDir, 'test.csv'), 'col1,col2\nval1,val2\n');

    // Create provider with symlink in allowedDirectories
    const provider = new FilesystemResourceProvider({
      workingDirectory: tempDir,
      allowedDirectories: [symlinkPath],
    });

    // Should be able to access files via the resolved path
    const resolved = await provider.resolvePath('test.csv');
    const resolvedTempDir = await realpath(tempDir);
    assert.strictEqual(resolved, join(resolvedTempDir, 'test.csv'));
  } finally {
    try {
      await unlink(join(tempDir, 'test.csv'));
      await unlink(symlinkPath);
      await rmdir(tempDir);
    } catch {
      // Ignore cleanup errors
    }
  }
});
