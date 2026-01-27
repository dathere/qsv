/**
 * Tests for SkillLoader
 */

import test from "node:test";
import assert from "node:assert";
import { SkillLoader } from "../src/loader.js";

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
