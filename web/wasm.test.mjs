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
  assert.equal(typeof exports.current_cube_camera_vertices_ptr, "function");
  assert.equal(typeof exports.current_cube_camera_face_vertices_ptr, "function");
  assert.equal(typeof exports.set_current_cube_camera, "function");
  assert.equal(typeof exports.cube_face_color_count, "function");
  assert.equal(typeof exports.current_cube_face_colors_ptr, "function");
  assert.equal(typeof exports.reset_current_rectangle, "function");
  assert.equal(typeof exports.rotate_current_cube_x, "function");
  assert.equal(typeof exports.rotate_current_cube_y, "function");
  assert.equal(typeof exports.rotate_current_rectangle_z, "function");
  assert.equal(typeof exports.rotate_current_cube_xyz, "undefined");
  assert.equal(exports.rectangle_vertex_count(), 24);
  assert.equal(exports.cube_face_vertex_count(), 24);
  assert.equal(exports.cube_face_color_count(), 6);
});

test("wasm owns orbit camera vertex locations", async () => {
  const exports = await loadWasm();

  exports.reset_current_rectangle();
  exports.set_current_cube_camera(0, 0);
  let floats = new Float32Array(
    exports.memory.buffer,
    exports.current_cube_camera_vertices_ptr(),
    exports.rectangle_vertex_count() * 3,
  );

  assert.deepEqual(Array.from(floats.slice(0, 3)), [0, 0, 0]);

  exports.set_current_cube_camera(1, 0.5);
  floats = new Float32Array(
    exports.memory.buffer,
    exports.current_cube_camera_vertices_ptr(),
    exports.rectangle_vertex_count() * 3,
  );

  assert.notDeepEqual(Array.from(floats.slice(0, 3)), [0, 0, 0]);
  assert.equal(Array.from(floats).some((value) => Object.is(value, -0)), false);
});

test("wasm owns solid cube face colors", async () => {
  const exports = await loadWasm();

  exports.reset_current_rectangle();
  exports.set_current_cube_camera(0, 0);
  let colors = new Float32Array(
    exports.memory.buffer,
    exports.current_cube_face_colors_ptr(),
    exports.cube_face_color_count() * 3,
  );
  const firstCameraColors = Array.from(colors);

  assert.equal(firstCameraColors.length, 18);
  assert.ok(firstCameraColors.every((value) => value >= 0 && value <= 255));

  exports.set_current_cube_camera(1, 0.5);
  colors = new Float32Array(
    exports.memory.buffer,
    exports.current_cube_face_colors_ptr(),
    exports.cube_face_color_count() * 3,
  );

  assert.notDeepEqual(Array.from(colors), firstCameraColors);
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

test("wasm rotates cube around explicit X and Y axes", async () => {
  const exports = await loadWasm();

  exports.reset_current_rectangle();
  exports.rotate_current_cube_x(90);
  let vertices = verticesFromExports(exports);
  assertClose(vertices[0], 0);
  assertClose(vertices[1], 0);
  assertClose(vertices[2], 2);

  exports.reset_current_rectangle();
  exports.rotate_current_cube_y(90);
  vertices = verticesFromExports(exports);
  assertClose(vertices[0], 0);
  assertClose(vertices[1], 0);
  assertClose(vertices[2], 2);
});
