import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

test("index.html has the controls and output hooks used by main.js", async () => {
  const html = await readFile(new URL("./index.html", import.meta.url), "utf8");

  for (const id of [
    "angle-select",
    "rotate-button",
    "reset-button",
    "canvas",
    "rotation-label",
    "vertices-output",
  ]) {
    assert.match(html, new RegExp(`id="${id}"`));
  }
});

test("main.js delegates rotation to Rust instead of doing trig itself", async () => {
  const js = await readFile(new URL("./main.js", import.meta.url), "utf8");

  assert.match(js, /rotate_current_rectangle_z/);
  assert.doesNotMatch(js, /Math\.(sin|cos|tan)/);
  assert.doesNotMatch(js, /function\s+rotate/i);
});
