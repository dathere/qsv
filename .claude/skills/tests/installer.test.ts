/**
 * Unit tests for the installer module
 */

import { test } from "node:test";
import assert from "node:assert";
import { detectPackageManager, getManualInstructions } from "../src/installer.js";

test("detectPackageManager returns a valid package manager type", () => {
  const result = detectPackageManager();
  assert.ok(
    result === "homebrew" || result === "scoop" || result === "none",
    `Expected homebrew, scoop, or none but got: ${result}`,
  );
});

test("detectPackageManager returns homebrew or none on non-Windows", () => {
  if (process.platform !== "win32") {
    const result = detectPackageManager();
    assert.notStrictEqual(result, "scoop", "scoop should not be detected on non-Windows");
  }
});

test("getManualInstructions returns instructions for macOS", () => {
  const result = getManualInstructions("darwin");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("macOS"), "should mention macOS");
  assert.ok(result.instructions!.includes("Homebrew"), "should mention Homebrew");
  assert.ok(
    result.instructions!.includes("github.com/dathere/qsv/releases"),
    "should include releases URL",
  );
});

test("getManualInstructions returns instructions for Windows", () => {
  const result = getManualInstructions("win32");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("Windows"), "should mention Windows");
  assert.ok(result.instructions!.includes("Scoop"), "should mention Scoop");
});

test("getManualInstructions returns instructions for Linux", () => {
  const result = getManualInstructions("linux");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("Linux"), "should mention Linux");
  assert.ok(
    result.instructions!.includes("Homebrew on Linux"),
    "should include Homebrew on Linux option",
  );
  assert.ok(
    !result.instructions!.includes("cargo install"),
    "should not reference cargo install",
  );
});

test("getManualInstructions handles unknown platforms as Linux", () => {
  const result = getManualInstructions("freebsd" as NodeJS.Platform);
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions for unknown platform");
  assert.ok(result.instructions!.includes("Linux"), "should fall back to Linux instructions");
});
