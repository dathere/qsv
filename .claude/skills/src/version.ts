/**
 * Version Management
 * 
 * Exports the version from package.json to ensure consistency across the codebase.
 */

import { readFileSync } from 'fs';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * Get version from package.json
 */
function getVersion(): string {
  try {
    // When compiled, __dirname is dist/src/, so go up 2 levels to project root
    const packageJsonPath = join(__dirname, '../../package.json');
    const packageJson = JSON.parse(readFileSync(packageJsonPath, 'utf-8'));
    return packageJson.version || '0.0.0';
  } catch (error) {
    console.error('[Version] Failed to read version from package.json:', error);
    return '0.0.0';
  }
}

export const VERSION = getVersion();
