/**
 * Tests for SkillLoader
 */

import test from "node:test";
import assert from "node:assert";
import { mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { SkillLoader } from "../src/loader.js";

// Minimal valid skill shape per isValidSkillShape() in loader.ts
const minimalSkillJson = (name: string) =>
  JSON.stringify({
    name,
    description: `${name} test skill`,
    version: "1.0.0",
    category: "test",
    command: { subcommand: name.replace(/^qsv-/, "") },
  });

test("SkillLoader initializes without errors", () => {
  const loader = new SkillLoader();
  assert.ok(loader, "SkillLoader should initialize");
});

test("isAllLoaded returns false before loading", () => {
  const loader = new SkillLoader();
  assert.strictEqual(
    loader.isAllLoaded(),
    false,
    "isAllLoaded should return false before any loading",
  );
});

test("loadByNames loads specific skills", async () => {
  const loader = new SkillLoader();

  // Load just a few common skills
  const skillNames = ["qsv-count", "qsv-headers", "qsv-stats"];
  const loaded = await loader.loadByNames(skillNames);

  assert.strictEqual(
    loaded.size,
    skillNames.length,
    `Should load ${skillNames.length} skills`,
  );

  for (const name of skillNames) {
    assert.ok(loaded.has(name), `Should have loaded ${name}`);
    const skill = loaded.get(name);
    assert.ok(skill, `Skill ${name} should exist`);
    assert.strictEqual(skill?.name, name, `Skill name should be ${name}`);
  }
});

test("loadByNames returns empty map for non-existent skills", async () => {
  const loader = new SkillLoader();

  const loaded = await loader.loadByNames([
    "qsv-nonexistent1",
    "qsv-nonexistent2",
  ]);

  assert.strictEqual(
    loaded.size,
    0,
    "Should return empty map for non-existent skills",
  );
});

test("loadByNames handles mix of valid and invalid skill names", async () => {
  const loader = new SkillLoader();

  const skillNames = ["qsv-count", "qsv-nonexistent", "qsv-headers"];
  const loaded = await loader.loadByNames(skillNames);

  assert.strictEqual(loaded.size, 2, "Should load 2 valid skills");
  assert.ok(loaded.has("qsv-count"), "Should have qsv-count");
  assert.ok(loaded.has("qsv-headers"), "Should have qsv-headers");
  assert.ok(!loaded.has("qsv-nonexistent"), "Should not have non-existent");
});

test("loadByNames caches loaded skills", async () => {
  const loader = new SkillLoader();

  // Load some skills
  await loader.loadByNames(["qsv-count", "qsv-headers"]);

  // Load again - should use cache
  const loaded = await loader.loadByNames(["qsv-count", "qsv-headers"]);

  assert.strictEqual(loaded.size, 2, "Should return cached skills");
});

test("isAllLoaded returns false after loadByNames", async () => {
  const loader = new SkillLoader();

  await loader.loadByNames(["qsv-count", "qsv-headers"]);

  assert.strictEqual(
    loader.isAllLoaded(),
    false,
    "isAllLoaded should return false after partial loading",
  );
});

test("loadAll sets isAllLoaded to true", async () => {
  const loader = new SkillLoader();

  await loader.loadAll();

  assert.strictEqual(
    loader.isAllLoaded(),
    true,
    "isAllLoaded should return true after loadAll",
  );
});

test("loadAll returns cached skills on second call", async () => {
  const loader = new SkillLoader();

  // First call loads all skills
  const first = await loader.loadAll();
  const firstSize = first.size;

  // Second call should return cached skills
  const second = await loader.loadAll();

  assert.strictEqual(
    second.size,
    firstSize,
    "Second loadAll should return same number of skills",
  );
  assert.strictEqual(first, second, "Should return same Map instance");
});

test("loadByNames works after loadAll", async () => {
  const loader = new SkillLoader();

  // Load all first
  await loader.loadAll();

  // loadByNames should still work (using cache)
  const loaded = await loader.loadByNames(["qsv-count", "qsv-headers"]);

  assert.strictEqual(loaded.size, 2, "Should load skills from cache");
});

test("load method uses cache from loadByNames", async () => {
  const loader = new SkillLoader();

  // Load via loadByNames
  await loader.loadByNames(["qsv-count"]);

  // Load same skill via load - should use cache
  const skill = await loader.load("qsv-count");

  assert.ok(skill, "Should return cached skill");
  assert.strictEqual(skill?.name, "qsv-count", "Skill name should match");
});

test("getAll returns loaded skills", async () => {
  const loader = new SkillLoader();

  await loader.loadByNames(["qsv-count", "qsv-headers", "qsv-stats"]);

  const all = loader.getAll();

  assert.strictEqual(all.length, 3, "Should return 3 loaded skills");
});

test("search works on loaded skills", async () => {
  const loader = new SkillLoader();

  await loader.loadByNames(["qsv-count", "qsv-headers", "qsv-stats"]);

  const results = loader.search("count");

  assert.ok(results.length >= 1, "Should find at least one matching skill");
  assert.ok(
    results.some((s) => s.name === "qsv-count"),
    "Should find qsv-count",
  );
});

test("loadByNames handles concurrent calls safely", async () => {
  const loader = new SkillLoader();

  const skillNames = ["qsv-count", "qsv-headers", "qsv-stats"];

  // Start multiple loadByNames calls simultaneously
  const concurrentLoads = [
    loader.loadByNames(skillNames),
    loader.loadByNames(skillNames),
    loader.loadByNames(skillNames),
    loader.loadByNames(skillNames),
  ];

  // All should complete successfully
  const results = await Promise.all(concurrentLoads);

  // Verify all calls returned successfully
  for (const loaded of results) {
    assert.strictEqual(
      loaded.size,
      skillNames.length,
      "Each concurrent call should return all skills",
    );
    for (const name of skillNames) {
      assert.ok(loaded.has(name), `Should have ${name}`);
    }
  }

  // Verify cache is correct after concurrent loads
  const cachedLoad = await loader.loadByNames(skillNames);
  assert.strictEqual(cachedLoad.size, skillNames.length, "Cache should work");
});

test("loadAll skips malformed JSON files and loads remaining valid skills", async () => {
  // Build a temp skills dir with several valid skills and one malformed file.
  // The malformed file would otherwise reject the Promise.all in doLoadAll.
  // BM25 consolidation requires a small minimum corpus, hence 3 valid skills.
  const tempDir = await mkdtemp(join(tmpdir(), "qsv-loader-test-"));
  try {
    await writeFile(join(tempDir, "qsv-count.json"), minimalSkillJson("qsv-count"));
    await writeFile(join(tempDir, "qsv-headers.json"), minimalSkillJson("qsv-headers"));
    await writeFile(join(tempDir, "qsv-stats.json"), minimalSkillJson("qsv-stats"));
    await writeFile(join(tempDir, "qsv-broken.json"), "{ this is not valid json");

    const loader = new SkillLoader(tempDir);
    const loaded = await loader.loadAll();

    assert.ok(loaded.has("qsv-count"), "Valid skill qsv-count should still load");
    assert.ok(loaded.has("qsv-headers"), "Valid skill qsv-headers should still load");
    assert.ok(loaded.has("qsv-stats"), "Valid skill qsv-stats should still load");
    assert.ok(!loaded.has("qsv-broken"), "Malformed file should be skipped, not crash");
    assert.strictEqual(loader.isAllLoaded(), true, "isAllLoaded should be true after partial success");
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
});

test("loadAll skips skill files missing required fields", async () => {
  // A syntactically-valid JSON file that doesn't match the QsvSkill shape
  // should be skipped without affecting other skills.
  const tempDir = await mkdtemp(join(tmpdir(), "qsv-loader-test-"));
  try {
    await writeFile(join(tempDir, "qsv-count.json"), minimalSkillJson("qsv-count"));
    await writeFile(join(tempDir, "qsv-headers.json"), minimalSkillJson("qsv-headers"));
    await writeFile(join(tempDir, "qsv-stats.json"), minimalSkillJson("qsv-stats"));
    await writeFile(
      join(tempDir, "qsv-bogus.json"),
      JSON.stringify({ not_a_skill: true }),
    );

    const loader = new SkillLoader(tempDir);
    const loaded = await loader.loadAll();

    assert.ok(loaded.has("qsv-count"), "Valid skill should still load");
    assert.ok(!loaded.has("qsv-bogus"), "Shape-invalid skill should be skipped");
  } finally {
    await rm(tempDir, { recursive: true, force: true });
  }
});

test("loadAll handles concurrent calls safely", async () => {
  const loader = new SkillLoader();

  // Start multiple loadAll calls simultaneously
  const concurrentLoads = [
    loader.loadAll(),
    loader.loadAll(),
    loader.loadAll(),
  ];

  // All should complete successfully
  const results = await Promise.all(concurrentLoads);

  // All should return the same Map instance (cached)
  const firstResult = results[0];
  for (const result of results) {
    assert.ok(result.size > 0, "Should load some skills");
    // After first load completes, subsequent calls return same instance
  }

  // Verify isAllLoaded is true
  assert.strictEqual(
    loader.isAllLoaded(),
    true,
    "isAllLoaded should be true after concurrent loads",
  );

  // Verify cache works correctly
  const cachedResult = await loader.loadAll();
  assert.strictEqual(
    cachedResult,
    firstResult,
    "Subsequent calls should return cached result",
  );
});
