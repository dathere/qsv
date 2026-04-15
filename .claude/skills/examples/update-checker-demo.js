#!/usr/bin/env node
/**
 * Update Checker Demo
 *
 * Demonstrates the auto-update checking functionality
 */

import { UpdateChecker, getUpdateConfigFromEnv } from '../dist/update-checker.js';

async function main() {
  console.log('QSV MCP Server - Update Checker Demo');
  console.log('=====================================\n');

  // Get qsv binary path from environment or use default
  const qsvBinPath = process.env.QSV_MCP_BIN_PATH || 'qsv';

  // Create update checker with default config
  const checker = new UpdateChecker(qsvBinPath, undefined, {
    autoRegenerateSkills: false, // Don't actually regenerate in demo
    checkForUpdatesOnStartup: true,
    notifyOnUpdatesAvailable: true
  });

  try {
    console.log('1️⃣  Checking qsv binary version...');
    const qsvVersion = await checker.getQsvBinaryVersion();
    console.log(`   ✅ qsv binary version: ${qsvVersion}\n`);

    console.log('2️⃣  Checking skills version...');
    const skillsVersion = checker.getSkillsVersion();
    console.log(`   ✅ Skills generated with: ${skillsVersion}\n`);

    console.log('3️⃣  Checking MCP server version...');
    const mcpVersion = checker.getMcpServerVersion();
    console.log(`   ✅ MCP server version: ${mcpVersion}\n`);

    console.log('4️⃣  Performing quick version check...');
    const quickCheck = await checker.quickCheck();
    if (quickCheck.skillsOutdated) {
      console.log('   ⚠️  Skills are outdated!');
      console.log(`   qsv binary: ${quickCheck.versions.qsvBinaryVersion}`);
      console.log(`   Skills: ${quickCheck.versions.skillsGeneratedWithVersion}\n`);
    } else {
      console.log('   ✅ Skills are up to date\n');
    }

    console.log('5️⃣  Performing full update check (includes GitHub API)...');
    console.log('   (This may take a few seconds...)\n');
    const fullCheck = await checker.checkForUpdates();

    if (fullCheck.recommendations.length > 0) {
      console.log('📦 Update Recommendations:');
      console.log('─────────────────────────');
      fullCheck.recommendations.forEach(rec => {
        console.log(rec);
      });
      console.log();
    } else {
      console.log('   ✅ Everything is up to date!\n');
    }

    console.log('6️⃣  Version tracking file:');
    const versionInfo = checker.loadVersionInfo();
    if (versionInfo) {
      console.log('   Stored version info:');
      console.log(`   - qsv binary: ${versionInfo.qsvBinaryVersion}`);
      console.log(`   - Skills: ${versionInfo.skillsGeneratedWithVersion}`);
      console.log(`   - MCP server: ${versionInfo.mcpServerVersion}`);
      console.log(`   - Last checked: ${versionInfo.lastChecked}\n`);
    } else {
      console.log('   No version info file found (this is normal on first run)\n');
    }

    console.log('✨ Demo complete!');
    console.log('\nTo enable auto-regeneration, set:');
    console.log('  QSV_MCP_AUTO_REGENERATE_SKILLS=true');
    console.log('\nSee AUTO_UPDATE.md for full documentation.');

  } catch (error) {
    console.error('❌ Error:', error.message);
    process.exit(1);
  }
}

main();
