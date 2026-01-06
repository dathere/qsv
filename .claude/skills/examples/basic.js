#!/usr/bin/env node
/**
 * Basic Skill Execution Example
 * Demonstrates loading and executing individual qsv skills
 */

import { SkillLoader, SkillExecutor } from '../dist/index.js';

async function main() {
  console.log('QSV Skills - Basic Execution Example');
  console.log('====================================\n');

  // Load skills
  const loader = new SkillLoader();
  await loader.loadAll();

  console.log(`Loaded ${loader.getAll().length} skills\n`);

  // Get skill statistics
  const stats = loader.getStats();
  console.log('Skill Statistics:');
  console.log(`  Total skills: ${stats.total}`);
  console.log(`  Total examples: ${stats.totalExamples}`);
  console.log(`  Total options: ${stats.totalOptions}`);
  console.log(`  Total arguments: ${stats.totalArgs}`);
  console.log('\nSkills by category:');
  Object.entries(stats.byCategory).forEach(([cat, count]) => {
    console.log(`  ${cat}: ${count}`);
  });
  console.log();

  // Search for skills
  console.log('Searching for "duplicate" skills:');
  const duplicateSkills = loader.search('duplicate');
  duplicateSkills.forEach(skill => {
    console.log(`  - ${skill.name}: ${skill.description.substring(0, 60)}...`);
  });
  console.log();

  // Load a specific skill
  const selectSkill = await loader.load('qsv-select');
  if (selectSkill) {
    console.log('qsv-select skill:');
    console.log(`  Description: ${selectSkill.description.substring(0, 100)}...`);
    console.log(`  Examples: ${selectSkill.examples.length}`);
    console.log(`  Options: ${selectSkill.command.options.length}`);
    console.log(`  Category: ${selectSkill.category}`);
    console.log();

    // Show first 3 examples
    console.log('First 3 examples:');
    selectSkill.examples.slice(0, 3).forEach((ex, i) => {
      console.log(`  ${i + 1}. ${ex.description}`);
      console.log(`     ${ex.command}`);
    });
    console.log();
  }

  // Execute a skill (if qsv is installed)
  console.log('Testing skill execution...');
  const executor = new SkillExecutor();

  // Create sample CSV data
  const csvData = `name,age,city
Alice,30,NYC
Bob,25,LA
Charlie,35,Chicago
Alice,30,NYC
David,28,Boston`;

  try {
    // Execute qsv-dedup skill
    const dedupSkill = await loader.load('qsv-dedup');
    if (dedupSkill) {
      console.log('Executing qsv-dedup...');
      const result = await executor.execute(dedupSkill, {
        stdin: csvData
      });

      console.log(`  Success: ${result.success}`);
      console.log(`  Command: ${result.metadata.command}`);
      console.log(`  Duration: ${result.metadata.duration}ms`);
      console.log(`  Output rows: ${result.output.split('\\n').length - 1}`);
      console.log('  Output:');
      console.log(result.output.split('\\n').slice(0, 5).map(l => `    ${l}`).join('\\n'));
    }
  } catch (error) {
    console.log(`  ⚠️  qsv not found or error: ${error.message}`);
    console.log('  Install qsv to run skill execution examples');
  }

  console.log('\n✨ Example complete!');
}

main().catch(console.error);