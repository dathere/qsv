/**
 * Unit tests for the installer module
 */

import { test } from "node:test";
import assert from "node:assert";
import { getAssetSuffix, getInstallDir, getManualInstructions } from "../src/installer.js";

test("getAssetSuffix returns correct suffix for macOS ARM", { skip: process.platform !== "darwin" || process.arch !== "arm64" ? "not macOS ARM" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "aarch64-apple-darwin");
});

test("getAssetSuffix returns correct suffix for macOS x64", { skip: process.platform !== "darwin" || process.arch !== "x64" ? "not macOS x64" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "x86_64-apple-darwin");
});

test("getAssetSuffix returns correct suffix for Linux x64", { skip: process.platform !== "linux" || process.arch !== "x64" ? "not Linux x64" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "x86_64-unknown-linux-gnu");
});

test("getAssetSuffix returns correct suffix for Linux ARM64", { skip: process.platform !== "linux" || process.arch !== "arm64" ? "not Linux ARM64" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "aarch64-unknown-linux-gnu");
});

test("getAssetSuffix returns correct suffix for Windows x64", { skip: process.platform !== "win32" || process.arch !== "x64" ? "not Windows x64" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "x86_64-pc-windows-msvc");
});

test("getAssetSuffix returns correct suffix for Windows ARM64", { skip: process.platform !== "win32" || process.arch !== "arm64" ? "not Windows ARM64" : false }, () => {
  assert.strictEqual(getAssetSuffix(), "aarch64-pc-windows-msvc");
});

test("getAssetSuffix returns string or null", () => {
  const result = getAssetSuffix();
  assert.ok(
    result === null || typeof result === "string",
    `Expected string or null but got: ${typeof result}`,
  );
});

test("getInstallDir returns a non-empty string", () => {
  const result = getInstallDir();
  assert.ok(typeof result === "string" && result.length > 0, "should return a non-empty path");
});

test("getInstallDir returns platform-appropriate path", () => {
  const result = getInstallDir();
  if (process.platform === "win32") {
    assert.ok(result.includes("Programs") && result.includes("qsv"), "Windows path should include Programs\\qsv");
  } else {
    assert.ok(
      result === "/usr/local/bin" || result.includes(".local/bin"),
      "Unix path should be /usr/local/bin or ~/.local/bin",
    );
  }
});

test("getManualInstructions returns instructions for macOS", () => {
  const result = getManualInstructions("darwin");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("macOS"), "should mention macOS");
  assert.ok(
    result.instructions!.includes("github.com/dathere/qsv/releases"),
    "should include releases URL",
  );
  assert.ok(
    !result.instructions!.includes("Homebrew"),
    "should not mention Homebrew",
  );
});

test("getManualInstructions returns instructions for Windows", () => {
  const result = getManualInstructions("win32");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("Windows"), "should mention Windows");
  assert.ok(
    !result.instructions!.includes("Scoop"),
    "should not mention Scoop",
  );
});

test("getManualInstructions returns instructions for Linux", () => {
  const result = getManualInstructions("linux");
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions");
  assert.ok(result.instructions!.includes("Linux"), "should mention Linux");
  assert.ok(
    !result.instructions!.includes("Homebrew"),
    "should not mention Homebrew",
  );
});

test("getManualInstructions handles unknown platforms as Linux", () => {
  const result = getManualInstructions("freebsd" as NodeJS.Platform);
  assert.strictEqual(result.success, false);
  assert.strictEqual(result.method, "manual");
  assert.ok(result.instructions, "should have instructions for unknown platform");
  assert.ok(result.instructions!.includes("Linux"), "should fall back to Linux instructions");
});
