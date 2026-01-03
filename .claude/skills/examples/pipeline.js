#!/usr/bin/env node
/**
 * Pipeline Composition Example
 * Demonstrates chaining multiple qsv skills into a data processing pipeline
 */

import { SkillLoader, QsvPipeline } from '../dist/index.js';
import { readFile, writeFile } from 'fs/promises';

async function main() {
  console.log('QSV Skills - Pipeline Composition Example');
  console.log('=========================================\n');

  // Load skills
  const loader = new SkillLoader();
  await loader.loadAll();

  // Create sample customer data
  const customerData = `customer_id,name,email,revenue,signup_date,test_account
1,Alice Smith,alice@example.com,15000,2023-01-15,false
2,Bob Test,bob.test@example.com,500,2023-02-20,true
3,Charlie Brown,charlie@invalid,8000,2023-03-10,false
4,Alice Smith,alice@example.com,15000,2023-01-15,false
5,David Lee,david@example.com,25000,2023-04-05,false
6,Test User,test@example.com,100,2023-05-01,true
7,Eve Wilson,eve@example.com,12000,2023-06-15,false
8,Frank Miller,frank@example.com,18000,2023-07-20,false`;

  console.log('Input data (8 rows):\n');
  console.log(customerData.split('\\n').slice(0, 4).join('\\n'));
  console.log('...\n');

  // Example 1: Data Cleaning Pipeline
  console.log('Example 1: Data Cleaning Pipeline');
  console.log('----------------------------------');
  console.log('Steps:');
  console.log('  1. Remove test accounts (test_account = true)');
  console.log('  2. Remove duplicate rows');
  console.log('  3. Filter invalid emails');
  console.log('  4. Select relevant columns');
  console.log('  5. Sort by revenue descending\n');

  const cleaningPipeline = new QsvPipeline(loader)
    .search('^false$', 'test_account')      // Keep only non-test accounts
    .dedup()                                 // Remove duplicates
    .search('^[^@]+@[^@]+\\.[^@]+$', 'email') // Valid emails only
    .select('customer_id,name,email,revenue') // Select columns
    .sortBy('revenue', { reverse: true });   // Sort by revenue desc

  // Generate shell script equivalent
  const shellScript = await cleaningPipeline.toShellScript();
  console.log('Equivalent shell command:');
  console.log(shellScript);
  console.log();

  try {
    const result = await cleaningPipeline.execute(customerData);

    console.log('Pipeline execution results:');
    console.log(`  Total duration: ${result.totalDuration}ms`);
    console.log(`  Steps executed: ${result.steps.length}`);
    console.log();

    result.steps.forEach((step, i) => {
      console.log(`  Step ${i + 1}: ${step.metadata.command}`);
      console.log(`    Duration: ${step.metadata.duration}ms`);
      console.log(`    Success: ${step.success}`);
    });

    console.log('\nFinal output:');
    console.log(result.output.toString());
  } catch (error) {
    console.log(`⚠️  Pipeline execution failed: ${error.message}`);
    console.log('Make sure qsv is installed and in your PATH');
  }

  // Example 2: Analytics Pipeline
  console.log('\nExample 2: Analytics Pipeline');
  console.log('-----------------------------');
  console.log('Steps:');
  console.log('  1. Select numeric columns');
  console.log('  2. Compute comprehensive statistics\n');

  const analyticsPipeline = new QsvPipeline(loader)
    .select('customer_id,revenue')
    .stats({ everything: true });

  console.log('Shell equivalent:');
  console.log(await analyticsPipeline.toShellScript());
  console.log();

  try {
    const result = await analyticsPipeline.execute(customerData);
    console.log('Statistics computed:');
    console.log(result.output.toString().split('\\n').slice(0, 5).join('\\n'));
    console.log('...');
  } catch (error) {
    console.log(`⚠️  ${error.message}`);
  }

  // Example 3: Fluent API Demonstration
  console.log('\nExample 3: Complex Data Transformation');
  console.log('--------------------------------------');

  const complexPipeline = new QsvPipeline(loader)
    .select('!test_account')                // Remove test_account column
    .dedup()                                 // Remove duplicates
    .filter('^[^@]+@[^@]+\\.[^@]+$', 'email') // Valid emails
    .sortBy('revenue', { reverse: true })    // Sort by revenue
    .slice(0, 5);                            // Top 5 customers

  console.log('Pipeline steps:', complexPipeline.getSteps().length);
  console.log('Shell script:');
  console.log(await complexPipeline.toShellScript());
  console.log();

  // Example 4: Frequency Analysis
  console.log('\nExample 4: Frequency Distribution');
  console.log('----------------------------------');

  const freqPipeline = new QsvPipeline(loader)
    .select('signup_date')
    .frequency({ limit: 10 });

  console.log('Analyzing signup date distribution...');
  console.log('Shell: ' + await freqPipeline.toShellScript());

  console.log('\n✨ Pipeline examples complete!');
  console.log('\nNext steps:');
  console.log('  - Try creating your own pipelines');
  console.log('  - Combine with file I/O for real data processing');
  console.log('  - Integrate with Claude Agent SDK for AI-powered workflows');
}

main().catch(console.error);
