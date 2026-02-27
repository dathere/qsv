/**
 * Unit tests for concurrency slot logic (acquireSlot / releaseSlot).
 * Validates that timed-out waiters are correctly skipped and
 * activeOperationCount stays consistent.
 */

import { test } from "node:test";
import assert from "node:assert";
import {
  _testConcurrency,
  getActiveOperationCount,
} from "../src/mcp-tools.js";
import { config } from "../src/config.js";

const { acquireSlot, releaseSlot, getSlotWaiterCount, setMaxConcurrent, reset } =
  _testConcurrency;

function setup(maxConcurrent: number): number {
  reset();
  const saved = config.maxConcurrentOperations;
  setMaxConcurrent(maxConcurrent);
  return saved;
}

function teardown(saved: number): void {
  setMaxConcurrent(saved);
  reset();
}

test("acquireSlot grants slot when under limit", async () => {
  const saved = setup(2);
  try {
    const ok = await acquireSlot(100);
    assert.strictEqual(ok, true);
    assert.strictEqual(getActiveOperationCount(), 1);

    const ok2 = await acquireSlot(100);
    assert.strictEqual(ok2, true);
    assert.strictEqual(getActiveOperationCount(), 2);
  } finally {
    teardown(saved);
  }
});

test("acquireSlot times out when all slots busy", async () => {
  const saved = setup(1);
  try {
    const ok = await acquireSlot(100);
    assert.strictEqual(ok, true);
    assert.strictEqual(getActiveOperationCount(), 1);

    // Second acquire should time out (slot is full)
    const ok2 = await acquireSlot(100);
    assert.strictEqual(ok2, false);
    assert.strictEqual(getActiveOperationCount(), 1);
  } finally {
    teardown(saved);
  }
});

test("releaseSlot hands off to a live waiter", async () => {
  const saved = setup(1);
  try {
    // Fill the slot
    await acquireSlot(100);
    assert.strictEqual(getActiveOperationCount(), 1);

    // Queue a waiter with a long timeout (will stay live)
    const waiterPromise = acquireSlot(5000);

    // Release the slot — should hand off to the queued waiter
    releaseSlot();

    const acquired = await waiterPromise;
    assert.strictEqual(acquired, true);
    // Count should still be 1 (handoff, not increment+decrement)
    assert.strictEqual(getActiveOperationCount(), 1);
    assert.strictEqual(getSlotWaiterCount(), 0);

    // Explicitly release the slot acquired by the waiter to mirror production usage.
    releaseSlot();
    assert.strictEqual(getActiveOperationCount(), 0);
  } finally {
    teardown(saved);
  }
});

test("releaseSlot skips timed-out waiters", async () => {
  const saved = setup(1);
  try {
    // Fill the slot
    await acquireSlot(100);
    assert.strictEqual(getActiveOperationCount(), 1);

    // Queue a waiter that will time out
    const timedOut = await acquireSlot(50);
    assert.strictEqual(timedOut, false);
    // Ensure timer callbacks have fired
    await new Promise((r) => setTimeout(r, 20));
    assert.strictEqual(getSlotWaiterCount(), 1); // still in queue

    // Release — should skip the timed-out waiter and decrement
    releaseSlot();
    assert.strictEqual(getActiveOperationCount(), 0);
    assert.strictEqual(getSlotWaiterCount(), 0);
  } finally {
    teardown(saved);
  }
});

test("releaseSlot skips multiple timed-out waiters then hands off to live one", async () => {
  const saved = setup(1);
  try {
    // Fill the slot
    await acquireSlot(100);

    // Queue two waiters that time out (each new acquireSlot prunes
    // all settled waiters from the array)
    const t1 = await acquireSlot(50);
    const t2 = await acquireSlot(50);
    assert.strictEqual(t1, false);
    assert.strictEqual(t2, false);
    // Ensure timer callbacks have fired
    await new Promise((r) => setTimeout(r, 20));
    assert.strictEqual(getSlotWaiterCount(), 1); // only last settled waiter remains

    // Queue a live waiter with long timeout (prunes all settled waiters first)
    const livePromise = acquireSlot(5000);
    assert.strictEqual(getSlotWaiterCount(), 1); // only the live waiter remains

    // Release — should hand off to the live waiter
    releaseSlot();

    const liveResult = await livePromise;
    assert.strictEqual(liveResult, true);
    assert.strictEqual(getActiveOperationCount(), 1);
    assert.strictEqual(getSlotWaiterCount(), 0);
  } finally {
    teardown(saved);
  }
});

test("releaseSlot decrements when all waiters timed out", async () => {
  const saved = setup(1);
  try {
    await acquireSlot(100);

    // Queue three waiters that all time out (each new acquireSlot prunes
    // all settled waiters from the array)
    await acquireSlot(50);
    await acquireSlot(50);
    await acquireSlot(50);
    // Ensure timer callbacks have fired
    await new Promise((r) => setTimeout(r, 20));
    assert.strictEqual(getSlotWaiterCount(), 1); // only last settled waiter remains

    releaseSlot();
    assert.strictEqual(getActiveOperationCount(), 0);
    assert.strictEqual(getSlotWaiterCount(), 0);
  } finally {
    teardown(saved);
  }
});

test("acquireSlot prunes settled waiters from middle of array, not just front", async () => {
  const saved = setup(1);
  try {
    // Fill the slot
    await acquireSlot(100);

    // Queue a live waiter with a long timeout (stays at front of queue)
    const livePromise = acquireSlot(5000);
    assert.strictEqual(getSlotWaiterCount(), 1);

    // Queue a waiter with a short timeout that will settle while the first stays live
    const shortResult = await acquireSlot(50);
    assert.strictEqual(shortResult, false);
    // Wait for the short-timeout waiter to settle
    await new Promise((r) => setTimeout(r, 70));

    // Now: waiter[0] is live (long timeout), waiter[1] is settled (timed out)
    // The settled waiter is at the end, not at the front.
    // A new acquireSlot call should prune the settled waiter from the middle/end.
    const another = acquireSlot(5000);
    // After pruning: the settled mid-array waiter is removed,
    // leaving the original live waiter + the new one = 2
    assert.strictEqual(getSlotWaiterCount(), 2);

    // Release the original slot — should hand off to the first live waiter
    releaseSlot();
    const liveResult = await livePromise;
    assert.strictEqual(liveResult, true);
    assert.strictEqual(getActiveOperationCount(), 1);

    // Release again — should hand off to the second live waiter
    releaseSlot();
    const anotherResult = await another;
    assert.strictEqual(anotherResult, true);
    assert.strictEqual(getActiveOperationCount(), 1);

    // Final cleanup
    releaseSlot();
    assert.strictEqual(getActiveOperationCount(), 0);
    assert.strictEqual(getSlotWaiterCount(), 0);
  } finally {
    teardown(saved);
  }
});
