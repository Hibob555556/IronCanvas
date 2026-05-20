import assert from "node:assert/strict";
import test from "node:test";

import {
  nextOrbitCamera,
  orbitMaxPitch,
  rotationDegreesForView,
} from "./orbit-controls.mjs";

test("orbit drag direction follows pointer movement", () => {
  const camera = { yaw: 0, pitch: 0 };

  const afterDragLeft = nextOrbitCamera(camera, -40, 0);
  assert.ok(afterDragLeft.yaw > camera.yaw);
  assert.equal(afterDragLeft.pitch, camera.pitch);

  const afterDragRight = nextOrbitCamera(camera, 40, 0);
  assert.ok(afterDragRight.yaw < camera.yaw);
  assert.equal(afterDragRight.pitch, camera.pitch);
});

test("orbit drag clamps pitch while preserving vertical direction", () => {
  const camera = { yaw: 0, pitch: 0 };

  const afterDragDown = nextOrbitCamera(camera, 0, 40);
  assert.ok(afterDragDown.pitch > camera.pitch);
  assert.equal(afterDragDown.yaw, camera.yaw);

  const afterLargeDragUp = nextOrbitCamera(camera, 0, -1000);
  assert.equal(afterLargeDragUp.pitch, -orbitMaxPitch);
});

test("view-aware rotation flips y only from the far side", () => {
  assert.equal(rotationDegreesForView(45, "y", false, { yaw: Math.PI, pitch: 0 }), 45);
  assert.equal(rotationDegreesForView(45, "y", true, { yaw: 0, pitch: 0 }), 45);
  assert.equal(rotationDegreesForView(45, "y", true, { yaw: Math.PI, pitch: 0 }), -45);
  assert.equal(rotationDegreesForView(45, "x", true, { yaw: Math.PI, pitch: 0 }), 45);
});
