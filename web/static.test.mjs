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

test("index.html presents the demo as a portfolio case study", async () => {
  const html = await readFile(new URL("./index.html", import.meta.url), "utf8");

  assert.match(html, /class="site-header"/);
  assert.match(html, /class="brand"/);
  assert.match(html, /class="site-nav"/);
  assert.match(html, /Project logic/);
  assert.match(html, /How a click becomes a transform/);
  assert.match(html, /Tested like a tiny production app/);
  assert.match(html, /Rust owns the math/);
  assert.match(html, /CI\/CD/);
  assert.match(html, /class="site-footer"/);
});

test("index.html includes SEO and social sharing metadata", async () => {
  const html = await readFile(new URL("./index.html", import.meta.url), "utf8");

  assert.match(html, /<title>Iron Canvas \| Rust WebAssembly Geometry Demo<\/title>/);
  assert.match(html, /name="robots"/);
  assert.match(html, /rel="canonical"/);
  assert.match(html, /property="og:title"/);
  assert.match(html, /property="og:image"/);
  assert.match(html, /name="twitter:card"/);
  assert.match(html, /application\/ld\+json/);
});

test("main.js delegates rotation to Rust instead of doing trig itself", async () => {
  const js = await readFile(new URL("./main.js", import.meta.url), "utf8");

  assert.match(js, /rotate_current_rectangle_z/);
  assert.doesNotMatch(js, /Math\.(sin|cos|tan)/);
  assert.doesNotMatch(js, /function\s+rotate/i);
});
