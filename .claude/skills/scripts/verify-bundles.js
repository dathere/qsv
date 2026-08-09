#!/usr/bin/env node
/**
 * Verify the built .mcpb and .plugin release artifacts.
 *
 * Run after `npm run mcpb:package && npm run plugin:package`. Intended for both
 * CI (see .github/workflows/mcp-server-ci.yml) and the manual release checklist
 * in mcp-release-prep/SKILL.md.
 *
 * Exists because nothing else exercises the packaging path: archiver was bumped
 * ^7 -> ^8 in #4241 and both packagers were hard-broken for weeks without a
 * single failing check (issue #4378).
 *
 * The load-bearing check is startsFromExtractedBundle(): it runs the server out
 * of the unpacked .mcpb exactly as Claude Desktop would. A bundle missing a
 * runtime dependency still builds and still installs -- it only fails on the
 * user's machine. Anything that changes what gets archived (issue #4379) needs
 * that check, not just a file listing.
 *
 * Requires `unzip` on PATH.
 */

import { execFileSync, spawn } from 'child_process';
import { existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync } from 'fs';
import { tmpdir } from 'os';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const rootDir = join(dirname(fileURLToPath(import.meta.url)), '..');
const repoRoot = join(rootDir, '..', '..');

// A bundle far below these is a sign the archive was truncated or a payload
// directory silently went missing.
const MIN_MCPB_BYTES = 1_000_000;
const MIN_PLUGIN_BYTES = 20_000;
const SERVER_START_TIMEOUT_MS = 90_000;
const SEMVER = /^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/;

const failures = [];
const check = (label, ok, detail = '') => {
  if (ok) {
    console.log(`  PASS  ${label}`);
  } else {
    console.log(`  FAIL  ${label}${detail ? ` -- ${detail}` : ''}`);
    failures.push(label);
  }
  return ok;
};

const readJson = (path) => JSON.parse(readFileSync(path, 'utf8'));
const unzipTo = (archive, dest) =>
  execFileSync('unzip', ['-qo', archive, '-d', dest], { stdio: 'pipe' });

// Counted with fs rather than `sh -c "ls ... | wc -l"`: the latter interpolates a
// path into a shell command, so a checkout directory containing spaces or shell
// metacharacters would break the check -- a silent wrong answer in the one script
// whose job is to be trustworthy.
const countSkillJsons = (dir) =>
  readdirSync(dir).filter((f) => f.startsWith('qsv-') && f.endsWith('.json')).length;

/**
 * Start the server from the extracted bundle and wait for its readiness line.
 * Resolves false on a non-zero early exit (e.g. ERR_MODULE_NOT_FOUND) or if the
 * marker never arrives.
 */
function startsFromExtractedBundle(extractDir) {
  return new Promise((resolve) => {
    const child = spawn(process.execPath, [join(extractDir, 'server', 'mcp-server.js')], {
      cwd: extractDir,
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    let stderr = '';
    let settled = false;
    const finish = (ok, why) => {
      if (settled) return;
      settled = true;
      clearTimeout(timer);
      child.kill('SIGKILL');
      if (!ok && stderr.trim()) {
        console.log(`        server stderr (tail):\n${stderr.trim().split('\n').slice(-12).map((l) => `          ${l}`).join('\n')}`);
      }
      resolve({ ok, why });
    };

    const timer = setTimeout(
      () => finish(false, `no readiness marker within ${SERVER_START_TIMEOUT_MS}ms`),
      SERVER_START_TIMEOUT_MS,
    );

    child.stderr.on('data', (buf) => {
      stderr += buf.toString();
      if (stderr.includes('Ready to accept requests')) finish(true, '');
    });
    child.on('error', (err) => finish(false, err.message));
    child.on('exit', (code) => finish(false, `exited early with code ${code}`));
  });
}

async function main() {
  const pkg = readJson(join(rootDir, 'package.json'));
  const version = pkg.version;
  console.log(`Verifying release artifacts for v${version}\n`);

  const mcpbPath = join(rootDir, `qsv-mcp-server-${version}.mcpb`);
  const pluginPath = join(rootDir, `qsv-data-wrangling-${version}.plugin`);

  console.log('Artifacts present and non-trivial');
  const haveMcpb = check(`${mcpbPath.split('/').pop()} exists`, existsSync(mcpbPath));
  const havePlugin = check(`${pluginPath.split('/').pop()} exists`, existsSync(pluginPath));
  if (!haveMcpb || !havePlugin) {
    console.log('\nCannot continue without both artifacts. Run:');
    console.log('  npm run mcpb:package && npm run plugin:package');
    process.exit(1);
  }
  check('.mcpb size is plausible', statSync(mcpbPath).size >= MIN_MCPB_BYTES);
  check('.plugin size is plausible', statSync(pluginPath).size >= MIN_PLUGIN_BYTES);

  // Version agreement across every file that carries one. The packagers do no
  // cross-validation, so a partial version sweep otherwise ships a correctly
  // *named* bundle containing a stale manifest.
  console.log('\nVersion agreement across source manifests');
  const sourceVersions = {
    'package.json': version,
    'manifest.json': readJson(join(rootDir, 'manifest.json')).version,
    '.claude-plugin/plugin.json': readJson(join(rootDir, '.claude-plugin', 'plugin.json')).version,
  };
  const marketplace = readJson(join(repoRoot, '.claude-plugin', 'marketplace.json'));
  sourceVersions['marketplace.json (metadata)'] = marketplace.metadata?.version;
  sourceVersions['marketplace.json (plugins[0])'] = marketplace.plugins?.[0]?.version;
  for (const [file, found] of Object.entries(sourceVersions)) {
    check(`${file} == ${version}`, found === version, `found ${found}`);
  }

  const tmp = mkdtempSync(join(tmpdir(), 'qsv-verify-'));
  try {
    // ---- .mcpb ----
    console.log('\n.mcpb contents');
    const mcpbDir = join(tmp, 'mcpb');
    unzipTo(mcpbPath, mcpbDir);

    const bundledManifest = readJson(join(mcpbDir, 'manifest.json'));
    check('bundled manifest.json version matches', bundledManifest.version === version,
      `found ${bundledManifest.version}`);
    check('bundled package.json version matches',
      readJson(join(mcpbDir, 'package.json')).version === version);

    const minQsv = bundledManifest._meta?.['com.dathere.qsv']?.minimum_qsv_version;
    check('minimum_qsv_version is valid semver', SEMVER.test(minQsv ?? ''), `found ${minQsv}`);

    check('server/mcp-server.js present', existsSync(join(mcpbDir, 'server', 'mcp-server.js')));

    // Skill count is compared against the source tree rather than a hardcoded
    // number, so adding a skill never requires editing this script.
    const srcSkills = countSkillJsons(join(rootDir, 'qsv'));
    const bundledSkills = countSkillJsons(join(mcpbDir, 'qsv'));
    check(`bundled skill JSON count matches source (${srcSkills})`, srcSkills === bundledSkills,
      `bundle has ${bundledSkills}`);

    // ---- the check that actually matters ----
    console.log('\nServer starts from the extracted bundle');
    const started = await startsFromExtractedBundle(mcpbDir);
    check('server reached "Ready to accept requests"', started.ok, started.why);

    // ---- .plugin ----
    console.log('\n.plugin contents');
    const pluginDir = join(tmp, 'plugin');
    unzipTo(pluginPath, pluginDir);

    // Without manifest.json, cowork-setup.cjs silently falls back to a "0.0.0"
    // floor, disabling the qsv version check entirely.
    const havePluginManifest = check('manifest.json present (else version floor silently becomes 0.0.0)',
      existsSync(join(pluginDir, 'manifest.json')));
    if (havePluginManifest) {
      const pm = readJson(join(pluginDir, 'manifest.json'));
      check('bundled plugin manifest version matches', pm.version === version, `found ${pm.version}`);
      const pMin = pm._meta?.['com.dathere.qsv']?.minimum_qsv_version;
      check('plugin minimum_qsv_version is valid semver', SEMVER.test(pMin ?? ''), `found ${pMin}`);
    }
    check('.claude-plugin/plugin.json version matches',
      existsSync(join(pluginDir, '.claude-plugin', 'plugin.json')) &&
        readJson(join(pluginDir, '.claude-plugin', 'plugin.json')).version === version);
    check('hooks/hooks.json present', existsSync(join(pluginDir, 'hooks', 'hooks.json')));
    check('cowork-CLAUDE.md present', existsSync(join(pluginDir, 'cowork-CLAUDE.md')));
  } finally {
    rmSync(tmp, { recursive: true, force: true });
  }

  console.log('');
  if (failures.length) {
    console.log(`FAILED: ${failures.length} check(s)`);
    failures.forEach((f) => console.log(`  - ${f}`));
    process.exit(1);
  }
  console.log('All bundle checks passed.');
}

main().catch((err) => {
  console.error('verify-bundles crashed:', err);
  process.exit(1);
});
