/**
 * Unit tests for pipeline execution
 */
import { test } from 'node:test';
import assert from 'node:assert';
import { executePipeline } from '../src/mcp-pipeline.js';
import { SkillLoader } from '../src/loader.js';
import { config } from '../src/config.js';
test('executePipeline rejects pipelines exceeding step limit', async () => {
    const loader = new SkillLoader();
    await loader.loadAll();
    // Create a pipeline with more steps than allowed
    const tooManySteps = Array(config.maxPipelineSteps + 1)
        .fill(null)
        .map((_, i) => ({
        command: 'select',
        params: { selection: '1' },
    }));
    const result = await executePipeline({
        input_file: 'nonexistent.csv', // Will fail later, but limit check should happen first
        steps: tooManySteps,
    }, loader);
    assert.strictEqual(result.isError, true);
    assert.ok(result.content[0].text?.includes('exceeds maximum step limit') ||
        result.content[0].text?.includes(String(config.maxPipelineSteps)));
});
test('executePipeline validates required parameters', async () => {
    const loader = new SkillLoader();
    await loader.loadAll();
    // Missing input_file
    const result1 = await executePipeline({ steps: [] }, loader);
    assert.strictEqual(result1.isError, true);
    assert.ok(result1.content[0].text?.includes('input_file'));
    // Missing steps
    const result2 = await executePipeline({ input_file: 'test.csv' }, loader);
    assert.strictEqual(result2.isError, true);
    assert.ok(result2.content[0].text?.includes('steps'));
    // Empty steps array
    const result3 = await executePipeline({ input_file: 'test.csv', steps: [] }, loader);
    assert.strictEqual(result3.isError, true);
    assert.ok(result3.content[0].text?.includes('non-empty array'));
});
test('executePipeline validates step structure', async () => {
    const loader = new SkillLoader();
    await loader.loadAll();
    // Step without command
    const result1 = await executePipeline({
        input_file: 'test.csv',
        steps: [{ params: {} }],
    }, loader);
    assert.strictEqual(result1.isError, true);
    assert.ok(result1.content[0].text?.includes('command'));
    // Step with invalid params (array instead of object)
    const result2 = await executePipeline({
        input_file: 'test.csv',
        steps: [{ command: 'select', params: [] }],
    }, loader);
    assert.strictEqual(result2.isError, true);
    assert.ok(result2.content[0].text?.includes('params'));
});
//# sourceMappingURL=mcp-pipeline.test.js.map