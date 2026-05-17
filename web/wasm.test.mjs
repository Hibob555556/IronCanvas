import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import test from "node:test";

async function loadWasm() {
  const bytes = await readFile(new URL("./iron_canvas.wasm", import.meta.url));
  const { instance } = await WebAssembly.instantiate(bytes);
  return instance.exports;
}

function verticesFromExports(exports) {
  const count = exports.rectangle_vertex_count();
  const floats = new Float32Array(
    exports.memory.buffer,
    exports.current_rectangle_vertices_ptr(),
    count * 3,
  );

  return Array.from(floats);
}

function assertClose(actual, expected) {
  assert.ok(
    Math.abs(actual - expected) < 0.00001,
    `expected ${expected}, got ${actual}`,
  );
}

test("wasm exports the cube runtime API with compatibility names", async () => {
  const exports = await loadWasm();

  assert.equal(typeof exports.memory, "object");
  assert.equal(typeof exports.rectangle_vertex_count, "function");
  assert.equal(typeof exports.current_rectangle_vertices_ptr, "function");
  assert.equal(typeof exports.cube_face_vertex_count, "function");
  assert.equal(typeof exports.current_cube_face_vertices_ptr, "function");
  assert.equal(typeof exports.reset_current_rectangle, "function");
  assert.equal(typeof exports.rotate_current_cube_x, "function");
  assert.equal(typeof exports.rotate_current_cube_y, "function");
  assert.equal(typeof exports.rotate_current_rectangle_z, "function");
  assert.equal(typeof exports.rotate_current_cube_xyz, "function");
  assert.equal(exports.rectangle_vertex_count(), 24);
  assert.equal(exports.cube_face_vertex_count(), 24);
});

test("wasm reset restores the original cube edge buffer", async () => {
  const exports = await loadWasm();

  exports.reset_current_rectangle();
  const vertices = verticesFromExports(exports);

  assert.deepEqual(vertices.slice(0, 6), [0, 0, 0, 0, 2, 0]);
  assert.deepEqual(vertices.slice(-3), [2, 0, 2]);
});

test("wasm rotates cube edges clockwise and avoids negative zero", async () => {
  const exports = await loadWasm();

  exports.reset_current_rectangle();
  exports.rotate_current_rectangle_z(90);
  const vertices = verticesFromExports(exports);

  assertClose(vertices[0], 0);
  assertClose(vertices[1], 2);
  assert.equal(vertices.some((value) => Object.is(value, -0)), false);
});
