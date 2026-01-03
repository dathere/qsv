#!/usr/bin/env node
/**
 * Test Examples Demo
 * Demonstrates loading CI test-based examples on-demand
 */

import { SkillLoader } from '../dist/index.js';

async function main() {
  console.log('QSV Skills - Test Examples Demo');
  console.log('================================\n');

  const loader = new SkillLoader();
  await loader.loadAll();

  // Example 1: Load test examples for dedup command
  console.log('Example 1: Loading test examples for qsv-dedup');
  console.log('-----------------------------------------------');

  const dedupExamples = await loader.loadTestExamples('qsv-dedup');

  if (dedupExamples) {
    console.log(`✅ Loaded ${dedupExamples.examples.length} test examples\n`);

    // Show first example
    const firstExample = dedupExamples.examples[0];
    console.log('First example:');
    console.log(`  Name: ${firstExample.name}`);
    console.log(`  Description: ${firstExample.description}`);
    console.log(`  Command: ${firstExample.command}`);
    console.log(`  Tags: ${firstExample.tags.join(', ')}`);

    if (firstExample.input?.data) {
      console.log(`\n  Input data (${firstExample.input.data.length} rows):`);
      firstExample.input.data.forEach(row => {
        console.log(`    ${row.join(', ')}`);
      });
    }

    if (firstExample.expected?.data) {
      console.log(`\n  Expected output (${firstExample.expected.data.length} rows):`);
      firstExample.expected.data.forEach(row => {
        console.log(`    ${row.join(', ')}`);
      });
    }
  } else {
    console.log('❌ No test examples available');
  }

  // Example 2: Show examples by tag
  console.log('\n\nExample 2: Filtering examples by tag');
  console.log('-------------------------------------');

  const statsExamples = await loader.loadTestExamples('qsv-stats');

  if (statsExamples) {
    console.log(`Total stats examples: ${statsExamples.examples.length}`);

    // Filter by tag
    const regressionTests = statsExamples.examples.filter(ex =>
      ex.tags && ex.tags.includes('regression')
    );

    console.log(`Regression tests: ${regressionTests.length}`);

    if (regressionTests.length > 0) {
      console.log('\nFirst regression test:');
      const example = regressionTests[0];
      console.log(`  ${example.name}: ${example.description}`);
      console.log(`  Command: ${example.command}`);
    }
  }

  // Example 3: Statistics about all test examples
  console.log('\n\nExample 3: Test Examples Statistics');
  console.log('------------------------------------');

  const skillsWithExamples = loader.getAll().filter(s => s.examples_ref);
  console.log(`Skills with test examples: ${skillsWithExamples.length}`);

  let totalTestExamples = 0;
  for (const skill of skillsWithExamples.slice(0, 10)) { // Just check first 10
    const examples = await loader.loadTestExamples(skill.name);
    if (examples) {
      totalTestExamples += examples.examples.length;
    }
  }

  console.log(`Total test examples (first 10 skills): ${totalTestExamples}`);

  // Example 4: Show example with options
  console.log('\n\nExample 4: Example with command options');
  console.log('----------------------------------------');

  if (dedupExamples) {
    // Find an example with options
    const exampleWithOptions = dedupExamples.examples.find(ex =>
      ex.options && Object.keys(ex.options).length > 0
    );

    if (exampleWithOptions) {
      console.log(`Name: ${exampleWithOptions.name}`);
      console.log(`Description: ${exampleWithOptions.description}`);
      console.log(`Command: ${exampleWithOptions.command}`);
      console.log(`Options:`, exampleWithOptions.options);
    } else {
      console.log('No examples with options found in dedup');
    }
  }

  console.log('\n✨ Demo complete!');
  console.log('\nKey Benefits:');
  console.log('  • Lightweight skill JSON files');
  console.log('  • Real, tested examples from CI');
  console.log('  • Input/output data for each example');
  console.log('  • Tagged for easy filtering');
  console.log('  • Loaded on-demand when needed');
}

main().catch(console.error);
