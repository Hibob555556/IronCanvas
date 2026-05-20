export const orbitDragSensitivity = 0.01;
export const orbitMaxPitch = Math.PI / 2 - 0.22;

const fullTurn = Math.PI * 2;

export function normalizeYaw(yaw) {
  return ((((yaw + Math.PI) % fullTurn) + fullTurn) % fullTurn) - Math.PI;
}

export function nextOrbitCamera(camera, deltaX, deltaY) {
  return {
    yaw: normalizeYaw(camera.yaw - deltaX * orbitDragSensitivity),
    pitch: Math.max(
      -orbitMaxPitch,
      Math.min(orbitMaxPitch, camera.pitch + deltaY * orbitDragSensitivity),
    ),
  };
}

export function rotationDegreesForView(degrees, axis, isOrbit, camera) {
  if (!isOrbit) {
    return degrees;
  }

  if (axis === "y" && Math.cos(camera.yaw) < 0) {
    return -degrees;
  }

  return degrees;
}
