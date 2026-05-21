import {
  nextOrbitCamera,
  rotationDegreesForView,
} from "./orbit-controls.mjs";

const canvas = document.querySelector("#canvas");
const ctx = canvas.getContext("2d");
const versionSelect = document.querySelector("#version-select");
const angleSelect = document.querySelector("#angle-select");
const axisSelect = document.querySelector("#axis-select");
const rotateButton = document.querySelector("#rotate-button");
const resetButton = document.querySelector("#reset-button");
const showHiddenVerticesInput = document.querySelector("#show-hidden-vertices");
const verticesOutput = document.querySelector("#vertices-output");
const rotationLabel = document.querySelector("#rotation-label");
const versionTitle = document.querySelector("#version-title");
const versionSummary = document.querySelector("#version-summary");
const changelogEntries = document.querySelectorAll(".changelog-entry");

let currentVertices = [];
let currentFaces = [];
let rotation = 0;
let wasmExports = null;
let loadedWasmExports = null;
let viewBounds = null;
let activeVersion = null;
let orbitCamera = {
  yaw: -0.72,
  pitch: -0.34,
};
let dragState = null;

const rectangleVertices = [
  [0.0, 0.0, 0.0],
  [0.0, 1.0, 0.0],
  [0.0, 2.0, 0.0],
  [1.0, 2.0, 0.0],
  [2.0, 2.0, 0.0],
  [3.0, 2.0, 0.0],
  [3.0, 1.0, 0.0],
  [3.0, 0.0, 0.0],
  [2.0, 0.0, 0.0],
  [1.0, 0.0, 0.0],
  [0.0, 0.0, 0.0],
];

const fallbackVertices = [
  [0.0, 0.0, 0.0],
  [0.0, 2.0, 0.0],
  [0.0, 2.0, 0.0],
  [2.0, 2.0, 0.0],
  [2.0, 2.0, 0.0],
  [2.0, 0.0, 0.0],
  [2.0, 0.0, 0.0],
  [0.0, 0.0, 0.0],
  [0.0, 0.0, 2.0],
  [0.0, 2.0, 2.0],
  [0.0, 2.0, 2.0],
  [2.0, 2.0, 2.0],
  [2.0, 2.0, 2.0],
  [2.0, 0.0, 2.0],
  [2.0, 0.0, 2.0],
  [0.0, 0.0, 2.0],
  [0.0, 0.0, 0.0],
  [0.0, 0.0, 2.0],
  [0.0, 2.0, 0.0],
  [0.0, 2.0, 2.0],
  [2.0, 2.0, 0.0],
  [2.0, 2.0, 2.0],
  [2.0, 0.0, 0.0],
  [2.0, 0.0, 2.0],
];

const fallbackFaces = [
  [0.0, 0.0, 0.0],
  [2.0, 0.0, 0.0],
  [2.0, 2.0, 0.0],
  [0.0, 2.0, 0.0],
  [2.0, 0.0, 0.0],
  [2.0, 0.0, 2.0],
  [2.0, 2.0, 2.0],
  [2.0, 2.0, 0.0],
  [0.0, 0.0, 2.0],
  [0.0, 2.0, 2.0],
  [2.0, 2.0, 2.0],
  [2.0, 0.0, 2.0],
  [0.0, 0.0, 0.0],
  [0.0, 2.0, 0.0],
  [0.0, 2.0, 2.0],
  [0.0, 0.0, 2.0],
  [0.0, 2.0, 0.0],
  [2.0, 2.0, 0.0],
  [2.0, 2.0, 2.0],
  [0.0, 2.0, 2.0],
  [0.0, 0.0, 0.0],
  [0.0, 0.0, 2.0],
  [2.0, 0.0, 2.0],
  [2.0, 0.0, 0.0],
];

const faceColors = [
  "rgba(124, 240, 207, 0.24)",
  "rgba(255, 210, 121, 0.24)",
  "rgba(112, 167, 255, 0.25)",
  "rgba(255, 140, 107, 0.22)",
  "rgba(180, 255, 205, 0.22)",
  "rgba(214, 172, 255, 0.2)",
];
const fallbackSolidFaceColor = "rgb(154, 163, 173)";
const axisGuides = {
  x: {
    label: "X",
    color: "#ff8c6b",
    start: [-0.7, 1.0, 1.0],
    end: [2.7, 1.0, 1.0],
  },
  y: {
    label: "Y",
    color: "#ffd279",
    start: [1.0, -0.7, 1.0],
    end: [1.0, 2.7, 1.0],
  },
  z: {
    label: "Z",
    color: "#70a7ff",
    start: [1.0, 1.0, -0.7],
    end: [1.0, 1.0, 2.7],
  },
};

const versions = {
  "0.4.1": {
    title: "v0.4.1 Orbit cube",
    summary:
      "Vertex visibility controls make the Rust-owned cube buffer easier to inspect while orbiting.",
    vertices: fallbackVertices,
    faces: fallbackFaces,
    center: [1.0, 1.0, 1.0],
    projection: "orbit",
    edgeMode: "pairs",
    axes: true,
    solidFaces: true,
    orbit: true,
    topLabel: null,
  },
  "0.3.2": {
    title: "v0.3.2 Shaded cube",
    summary:
      "Filled cube faces and visible axis guides make the X, Y, and Z rotations easier to read.",
    vertices: fallbackVertices,
    faces: fallbackFaces,
    center: [1.0, 1.0, 1.0],
    projection: "dimetric",
    edgeMode: "pairs",
    axes: true,
    topLabel: { start: 2, end: 3, text: "TOP" },
  },
  "0.2.0": {
    title: "v0.2.0 Wire cube",
    summary:
      "The rectangle becomes a 3D wire cube made from explicit edge pairs, with no filled faces yet.",
    vertices: fallbackVertices,
    faces: [],
    center: [1.0, 1.0, 1.0],
    projection: "dimetric",
    edgeMode: "pairs",
    axes: true,
    topLabel: { start: 2, end: 3, text: "TOP" },
  },
  "0.1.0": {
    title: "v0.1.0 Rectangle rotation",
    summary:
      "The original flat rectangle outline rotates clockwise around its Z-axis through Rust-owned vertex data.",
    vertices: rectangleVertices,
    faces: [],
    center: [1.5, 1.0, 0.0],
    projection: "flat",
    edgeMode: "path",
    axes: false,
    topLabel: { start: 2, end: 5, text: "TOP" },
  },
};

function createFallbackRuntime(loadError, version = versions["0.3.2"]) {
  if (loadError) {
    console.warn(loadError);
  }

  const sourceVertices = version.vertices;
  const sourceFaces = version.faces;
  const edgeFloatCount = sourceVertices.length * 3;
  const faceFloatCount = sourceFaces.length * 3;
  const faceByteOffset = edgeFloatCount * 4;
  const memory = {
    buffer: new ArrayBuffer((edgeFloatCount + faceFloatCount) * 4),
  };
  const edgeFloats = new Float32Array(memory.buffer, 0, edgeFloatCount);
  const faceFloats = new Float32Array(memory.buffer, faceByteOffset, faceFloatCount);
  const center = version.center;

  function writeVertices(target, vertices) {
    vertices.flat().forEach((value, index) => {
      target[index] = value;
    });
  }

  function angleValues(degrees) {
    const normalized = ((degrees % 360) + 360) % 360;
    const diagonal = 0.7071067811865476;

    if (normalized === 45) return [diagonal, -diagonal];
    if (normalized === 90) return [0.0, -1.0];
    if (normalized === 135) return [-diagonal, -diagonal];
    if (normalized === 180) return [-1.0, 0.0];
    if (normalized === 225) return [-diagonal, diagonal];
    if (normalized === 270) return [0.0, 1.0];
    if (normalized === 315) return [diagonal, diagonal];

    return [1.0, 0.0];
  }

  function rotateBuffer(target, degrees, axis) {
    const [c, s] = angleValues(degrees);

    for (let index = 0; index < target.length; index += 3) {
      const x = target[index] - center[0];
      const y = target[index + 1] - center[1];
      const z = target[index + 2] - center[2];

      if (axis === "x") {
        target[index + 1] = center[1] + y * c - z * s;
        target[index + 2] = center[2] + y * s + z * c;
      }

      if (axis === "y") {
        target[index] = center[0] + x * c - z * s;
        target[index + 2] = center[2] + x * s + z * c;
      }

      if (axis === "z") {
        target[index] = center[0] + x * c - y * s;
        target[index + 1] = center[1] + x * s + y * c;
      }
    }
  }

  function rotateAll(degrees, axis) {
    rotateBuffer(edgeFloats, degrees, axis);
    rotateBuffer(faceFloats, degrees, axis);
  }

  writeVertices(edgeFloats, sourceVertices);
  writeVertices(faceFloats, sourceFaces);

  return {
    memory,
    rectangle_vertex_count() {
      return sourceVertices.length;
    },
    current_rectangle_vertices_ptr() {
      return 0;
    },
    cube_face_vertex_count() {
      return sourceFaces.length;
    },
    current_cube_face_vertices_ptr() {
      return faceByteOffset;
    },
    reset_current_rectangle() {
      writeVertices(edgeFloats, sourceVertices);
      writeVertices(faceFloats, sourceFaces);
    },
    rotate_current_cube_x(degrees) {
      rotateAll(degrees, "x");
    },
    rotate_current_cube_y(degrees) {
      rotateAll(degrees, "y");
    },
    rotate_current_rectangle_z(degrees) {
      rotateAll(degrees, "z");
    },
  };
}

function verticesFromWasm(exports, ptr) {
  const count = exports.rectangle_vertex_count();
  const floats = new Float32Array(exports.memory.buffer, ptr, count * 3);
  const vertices = [];

  for (let index = 0; index < count; index += 1) {
    const offset = index * 3;
    vertices.push([
      floats[offset],
      floats[offset + 1],
      floats[offset + 2],
    ]);
  }

  return vertices;
}

function vertexBufferFromWasm(exports, ptr, count) {
  const floats = new Float32Array(exports.memory.buffer, ptr, count * 3);
  const vertices = [];

  for (let index = 0; index < count; index += 1) {
    const offset = index * 3;
    vertices.push([floats[offset], floats[offset + 1], floats[offset + 2]]);
  }

  return vertices;
}

function currentVerticesFromRust() {
  const ptr = wasmExports.current_rectangle_vertices_ptr();
  return verticesFromWasm(wasmExports, ptr);
}

function currentFacesFromRust() {
  if (
    typeof wasmExports.cube_face_vertex_count !== "function" ||
    typeof wasmExports.current_cube_face_vertices_ptr !== "function"
  ) {
    return [];
  }

  const count = wasmExports.cube_face_vertex_count();
  const ptr = wasmExports.current_cube_face_vertices_ptr();
  const floats = new Float32Array(wasmExports.memory.buffer, ptr, count * 3);
  const faces = [];

  for (let index = 0; index < count; index += 4) {
    const face = [];

    for (let corner = 0; corner < 4; corner += 1) {
      const offset = (index + corner) * 3;
      face.push([floats[offset], floats[offset + 1], floats[offset + 2]]);
    }

    faces.push(face);
  }

  return faces;
}

function currentCameraVerticesFromRust() {
  if (typeof wasmExports.current_cube_camera_vertices_ptr !== "function") {
    return currentVertices;
  }

  const ptr = wasmExports.current_cube_camera_vertices_ptr();
  return vertexBufferFromWasm(wasmExports, ptr, wasmExports.rectangle_vertex_count());
}

function currentCameraFacesFromRust() {
  if (
    typeof wasmExports.current_cube_camera_face_vertices_ptr !== "function" ||
    typeof wasmExports.cube_face_vertex_count !== "function"
  ) {
    return currentFaces;
  }

  const count = wasmExports.cube_face_vertex_count();
  const ptr = wasmExports.current_cube_camera_face_vertices_ptr();
  const vertices = vertexBufferFromWasm(wasmExports, ptr, count);
  const faces = [];

  for (let index = 0; index < vertices.length; index += 4) {
    faces.push(vertices.slice(index, index + 4));
  }

  return faces;
}

function currentFaceColorsFromRust() {
  if (
    typeof wasmExports.cube_face_color_count !== "function" ||
    typeof wasmExports.current_cube_face_colors_ptr !== "function"
  ) {
    return [];
  }

  const count = wasmExports.cube_face_color_count();
  const ptr = wasmExports.current_cube_face_colors_ptr();
  const floats = new Float32Array(wasmExports.memory.buffer, ptr, count * 3);
  const colors = [];

  for (let index = 0; index < count; index += 1) {
    const offset = index * 3;
    colors.push(
      `rgb(${Math.round(floats[offset])}, ${Math.round(floats[offset + 1])}, ${Math.round(floats[offset + 2])})`,
    );
  }

  return colors;
}

function projectVertex(vertex) {
  const [x, y, z] = vertex;

  if (activeVersion.projection === "flat") {
    return [x, y];
  }

  if (activeVersion.projection === "orbit") {
    return [x, y];
  }

  const centeredX = x - activeVersion.center[0];
  const centeredY = y - activeVersion.center[1];
  const centeredZ = z - activeVersion.center[2];

  return [
    activeVersion.center[0] + centeredX * 0.86 - centeredZ * 0.5,
    activeVersion.center[1] + centeredY * 0.95 + (centeredX + centeredZ) * 0.22,
  ];
}

function cameraDepth(vertex) {
  if (activeVersion.projection === "orbit") {
    return vertex[2];
  }

  const [x, y, z] = vertex;
  const centeredX = x - activeVersion.center[0];
  const centeredY = y - activeVersion.center[1];
  const centeredZ = z - activeVersion.center[2];

  return centeredX * 0.35 + centeredZ * 0.75 - centeredY * 0.1;
}

function createViewBounds(vertices, faces = []) {
  const faceVertices = faces.flat();
  const allVertices = [...vertices, ...faceVertices];
  const projected = allVertices.map(projectVertex);
  const xs = projected.map((vertex) => vertex[0]);
  const ys = projected.map((vertex) => vertex[1]);
  const minX = Math.min(...xs);
  const maxX = Math.max(...xs);
  const minY = Math.min(...ys);
  const maxY = Math.max(...ys);
  const centerX = (minX + maxX) / 2;
  const centerY = (minY + maxY) / 2;
  const width = maxX - minX;
  const height = maxY - minY;
  const span = Math.max(Math.hypot(width, height), 1);

  return {
    minX: centerX - span / 2,
    maxX: centerX + span / 2,
    minY: centerY - span / 2,
    maxY: centerY + span / 2,
  };
}

function createOrbitViewBounds() {
  const radius = 2.05;

  return {
    minX: activeVersion.center[0] - radius,
    maxX: activeVersion.center[0] + radius,
    minY: activeVersion.center[1] - radius,
    maxY: activeVersion.center[1] + radius,
  };
}

function fitToCanvas() {
  const padding = 64;
  const width = viewBounds.maxX - viewBounds.minX;
  const height = viewBounds.maxY - viewBounds.minY;
  const scale = Math.min(
    (canvas.width - padding * 2) / width,
    (canvas.height - padding * 2) / height,
  );
  const scaledWidth = width * scale;
  const scaledHeight = height * scale;
  const offsetX = (canvas.width - scaledWidth) / 2;
  const offsetY = (canvas.height - scaledHeight) / 2;

  return (vertex) => [
    offsetX + (projectVertex(vertex)[0] - viewBounds.minX) * scale,
    canvas.height - offsetY - (projectVertex(vertex)[1] - viewBounds.minY) * scale,
  ];
}

function isPointInCanvas([x, y], margin = 0) {
  return (
    x >= -margin &&
    x <= canvas.width + margin &&
    y >= -margin &&
    y <= canvas.height + margin
  );
}

function screenBounds(points) {
  const xs = points.map(([x]) => x);
  const ys = points.map(([, y]) => y);

  return {
    minX: Math.min(...xs),
    maxX: Math.max(...xs),
    minY: Math.min(...ys),
    maxY: Math.max(...ys),
  };
}

function boundsIntersectCanvas(bounds, margin = 0) {
  return (
    bounds.maxX >= -margin &&
    bounds.minX <= canvas.width + margin &&
    bounds.maxY >= -margin &&
    bounds.minY <= canvas.height + margin
  );
}

function projectedVerticesIntersectCanvas(vertices, project, margin = 0) {
  return boundsIntersectCanvas(screenBounds(vertices.map(project)), margin);
}

function nearlySameScreenPoint(a, b, radius = 0.75) {
  return Math.hypot(a[0] - b[0], a[1] - b[1]) <= radius;
}

function sign(point, start, end) {
  return (point[0] - end[0]) * (start[1] - end[1]) - (start[0] - end[0]) * (point[1] - end[1]);
}

function pointInTriangle(point, a, b, c) {
  const d1 = sign(point, a, b);
  const d2 = sign(point, b, c);
  const d3 = sign(point, c, a);
  const hasNegative = d1 < 0 || d2 < 0 || d3 < 0;
  const hasPositive = d1 > 0 || d2 > 0 || d3 > 0;

  return !(hasNegative && hasPositive);
}

function faceContainsProjectedPoint(face, point, project) {
  const projectedFace = face.map(project);

  return (
    pointInTriangle(point, projectedFace[0], projectedFace[1], projectedFace[2]) ||
    pointInTriangle(point, projectedFace[0], projectedFace[2], projectedFace[3])
  );
}

function faceDepth(face) {
  return face.reduce((total, vertex) => total + cameraDepth(vertex), 0) / face.length;
}

function isDepthInFront(a, b, margin = 0.02) {
  return a > b + margin;
}

function isVertexOccluded(vertex, vertexIndex, vertices, faces, project) {
  const point = project(vertex);
  const depth = cameraDepth(vertex);

  const hiddenBehindFace = faces.some((face) => {
    if (face.some((faceVertex) => nearlySameScreenPoint(project(faceVertex), point))) {
      return false;
    }

    return isDepthInFront(faceDepth(face), depth) && faceContainsProjectedPoint(face, point, project);
  });

  if (hiddenBehindFace) {
    return true;
  }

  return vertices.some((otherVertex, otherIndex) => {
    if (otherIndex === vertexIndex || !isDepthInFront(cameraDepth(otherVertex), depth)) {
      return false;
    }

    return nearlySameScreenPoint(project(otherVertex), point);
  });
}

function isDuplicateVisibleVertex(vertex, index, visibleEntries, project) {
  const point = project(vertex);
  const depth = cameraDepth(vertex);

  return visibleEntries.some(
    (entry) =>
      nearlySameScreenPoint(project(entry.vertex), point) &&
      Math.abs(cameraDepth(entry.vertex) - depth) <= 0.02 &&
      entry.index < index,
  );
}

function drawGrid() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "#07111f";
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  ctx.strokeStyle = "rgba(255, 255, 255, 0.08)";
  ctx.lineWidth = 1;

  for (let x = 40; x < canvas.width; x += 40) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, canvas.height);
    ctx.stroke();
  }

  for (let y = 40; y < canvas.height; y += 40) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }
}

function drawFaces(faces, project, solidColors = []) {
  faces
    .map((face, index) => ({
      face,
      index,
      depth: faceDepth(face),
    }))
    .filter(({ face }) => projectedVerticesIntersectCanvas(face, project, 2))
    .sort((a, b) => a.depth - b.depth)
    .forEach(({ face, index }) => {
      ctx.beginPath();

      face.forEach((vertex, corner) => {
        const [x, y] = project(vertex);

        if (corner === 0) {
          ctx.moveTo(x, y);
        } else {
          ctx.lineTo(x, y);
        }
      });

      ctx.closePath();
      ctx.fillStyle = activeVersion.solidFaces
        ? solidColors[index] ?? fallbackSolidFaceColor
        : faceColors[index % faceColors.length];
      ctx.strokeStyle = activeVersion.solidFaces
        ? "rgba(12, 17, 23, 0.72)"
        : "rgba(255, 255, 255, 0.16)";
      ctx.lineWidth = activeVersion.solidFaces ? 2 : 1;
      ctx.fill();
      ctx.stroke();
    });
}

function drawAxisGuides(project) {
  if (!activeVersion?.axes || activeVersion?.orbit) {
    return;
  }

  const guideKeys = [axisSelect.value];

  guideKeys.forEach((key) => {
    const guide = axisGuides[key];
    const [startX, startY] = project(guide.start);
    const [endX, endY] = project(guide.end);

    ctx.save();
    ctx.strokeStyle = guide.color;
    ctx.fillStyle = guide.color;
    ctx.lineWidth = 3;
    ctx.setLineDash([12, 8]);
    ctx.beginPath();
    ctx.moveTo(startX, startY);
    ctx.lineTo(endX, endY);
    ctx.stroke();
    ctx.setLineDash([]);

    ctx.beginPath();
    ctx.arc(endX, endY, 5, 0, Math.PI * 2);
    ctx.fill();

    ctx.font = "800 14px system-ui, sans-serif";
    ctx.textAlign = "center";
    ctx.textBaseline = "middle";
    ctx.fillText(guide.label, endX, endY - 18);
    ctx.restore();
  });
}

function drawVertices(vertices, faces, solidColors = []) {
  const project = fitToCanvas();

  drawGrid();
  drawFaces(faces, project, solidColors);
  drawAxisGuides(project);

  if (!activeVersion.solidFaces) {
    ctx.strokeStyle = "#7cf0cf";
    ctx.lineWidth = 4;
    ctx.lineJoin = "round";
    ctx.lineCap = "round";
    ctx.beginPath();

    if (activeVersion?.edgeMode === "path") {
      const projected = vertices.map(project);
      let hasVisiblePathPoint = false;

      projected.forEach(([x, y], index) => {
        if (!isPointInCanvas([x, y], 8)) {
          hasVisiblePathPoint = false;
          return;
        }

        if (!hasVisiblePathPoint) {
          ctx.moveTo(x, y);
          hasVisiblePathPoint = true;
        } else {
          ctx.lineTo(x, y);
        }
      });
    } else {
      for (let index = 0; index < vertices.length; index += 2) {
        if (!projectedVerticesIntersectCanvas([vertices[index], vertices[index + 1]], project, 8)) {
          continue;
        }

        const [startX, startY] = project(vertices[index]);
        const [endX, endY] = project(vertices[index + 1]);

        ctx.moveTo(startX, startY);
        ctx.lineTo(endX, endY);
      }
    }

    ctx.stroke();
  }

  if (!activeVersion.solidFaces) {
    vertices.forEach((vertex) => {
      const [x, y] = project(vertex);

      if (!isPointInCanvas([x, y], 8)) {
        return;
      }

      ctx.fillStyle = "#f3f7ff";
      ctx.beginPath();
      ctx.arc(x, y, 6, 0, Math.PI * 2);
      ctx.fill();
    });
  }

  const label = activeVersion?.topLabel;

  if (!label) {
    return;
  }

  const [startX, startY] = project(vertices[label.start]);
  const [endX, endY] = project(vertices[label.end]);

  if (
    !projectedVerticesIntersectCanvas(
      [vertices[label.start], vertices[label.end]],
      project,
      24,
    )
  ) {
    return;
  }

  const labelX = (startX + endX) / 2;
  const labelY = (startY + endY) / 2;

  ctx.fillStyle = "#ffd279";
  ctx.font = "700 16px system-ui, sans-serif";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(label.text, labelX, labelY - 18);
}

function vertexEntries(vertices) {
  return vertices.map((vertex, index) => ({ vertex, index }));
}

function visibleVertexEntries(vertices, faces) {
  const project = fitToCanvas();
  const visibleEntries = [];

  vertices.forEach((vertex, index) => {
    if (
      !isPointInCanvas(project(vertex), 8) ||
      isVertexOccluded(vertex, index, vertices, faces, project) ||
      isDuplicateVisibleVertex(vertex, index, visibleEntries, project)
    ) {
      return;
    }

    visibleEntries.push({ vertex, index });
  });

  return visibleEntries;
}

function formatVertices(vertexEntries) {
  const output = vertexEntries
    .map(({ vertex, index }) => {
      const [x, y, z] = vertex.map((value) => {
        const displayValue = Math.abs(value) < 0.000001 ? 0 : value;
        return displayValue.toFixed(2);
      });
      return `${String(index).padStart(2, "0")}: [${x}, ${y}, ${z}]`;
    })
    .join("\n");

  return output || "No vertices in view.";
}

function currentVertexOutputEntries(vertices, faces) {
  return showHiddenVerticesInput.checked
    ? vertexEntries(vertices)
    : visibleVertexEntries(vertices, faces);
}

function updateOrbitFromPointer(event) {
  if (!dragState || !activeVersion?.orbit) {
    return;
  }

  const deltaX = event.clientX - dragState.x;
  const deltaY = event.clientY - dragState.y;
  orbitCamera = nextOrbitCamera(orbitCamera, deltaX, deltaY);
  dragState.x = event.clientX;
  dragState.y = event.clientY;
  render();
}

function startOrbitDrag(event) {
  if (!activeVersion?.orbit) {
    return;
  }

  dragState = {
    pointerId: event.pointerId,
    x: event.clientX,
    y: event.clientY,
  };
  canvas.setPointerCapture(event.pointerId);
  canvas.classList.add("is-dragging");
}

function stopOrbitDrag(event) {
  if (!dragState || dragState.pointerId !== event.pointerId) {
    return;
  }

  dragState = null;
  canvas.classList.remove("is-dragging");
}

function syncOrbitAvailability() {
  canvas.classList.toggle("can-orbit", Boolean(activeVersion?.orbit));
}

function render() {
  let renderVertices = currentVertices;
  let renderFaces = currentFaces;
  let renderFaceColors = [];

  if (activeVersion?.orbit) {
    if (typeof wasmExports.set_current_cube_camera === "function") {
      wasmExports.set_current_cube_camera(orbitCamera.yaw, orbitCamera.pitch);
      renderVertices = currentCameraVerticesFromRust();
      renderFaces = currentCameraFacesFromRust();
      renderFaceColors = currentFaceColorsFromRust();
    }
  }

  drawVertices(renderVertices, renderFaces, renderFaceColors);
  verticesOutput.textContent = formatVertices(currentVertexOutputEntries(renderVertices, renderFaces));
  rotationLabel.textContent = `${activeVersion.title} | ${rotation} degrees | ${axisSelect.selectedOptions[0].textContent}`;
}

async function loadWasm() {
  const wasmUrl = new URL("./iron_canvas.wasm", import.meta.url);
  const response = await fetch(wasmUrl);

  if (!response.ok) {
    throw new Error(
      `Could not fetch ${wasmUrl.pathname}. Build the Rust module so web/iron_canvas.wasm exists.`,
    );
  }

  const bytes = await response.arrayBuffer();
  const signature = new Uint8Array(bytes, 0, Math.min(bytes.byteLength, 4));
  const isWasm =
    signature[0] === 0x00 &&
    signature[1] === 0x61 &&
    signature[2] === 0x73 &&
    signature[3] === 0x6d;

  if (!isWasm) {
    throw new Error(
      `Expected a WebAssembly binary at ${wasmUrl.pathname}, but the server returned another file. Build and serve web/iron_canvas.wasm next to index.html.`,
    );
  }

  const wasm = await WebAssembly.instantiate(bytes);
  return wasm.instance.exports;
}

function syncChangelog(versionKey) {
  versionTitle.textContent = activeVersion.title;
  versionSummary.textContent = activeVersion.summary;

  changelogEntries.forEach((entry) => {
    entry.classList.toggle("is-active", entry.dataset.version === versionKey);
  });
}

function resetCurrentDemo() {
  wasmExports.reset_current_rectangle();
  currentVertices = currentVerticesFromRust();
  currentFaces = currentFacesFromRust();
  viewBounds = activeVersion?.orbit
    ? createOrbitViewBounds()
    : createViewBounds(currentVertices, currentFaces);
  rotation = 0;
  render();
}

function selectVersion(versionKey) {
  activeVersion = versions[versionKey];
  const wasAxisDisabled = axisSelect.disabled;

  if ((versionKey === "0.4.1" || versionKey === "0.3.2") && loadedWasmExports) {
    wasmExports = loadedWasmExports;
  } else {
    wasmExports = createFallbackRuntime(null, activeVersion);
  }

  axisSelect.disabled = !activeVersion.axes;

  if (!activeVersion.axes) {
    axisSelect.value = "z";
  } else if (wasAxisDisabled) {
    axisSelect.value = "y";
  }

  syncChangelog(versionKey);
  syncOrbitAvailability();
  resetCurrentDemo();
}

function rotateCurrentDemo(degrees, axis) {
  const displayDegrees = rotationDegreesForView(
    degrees,
    axis,
    Boolean(activeVersion?.orbit),
    orbitCamera,
  );

  if (axis === "x" && typeof wasmExports.rotate_current_cube_x === "function") {
    wasmExports.rotate_current_cube_x(displayDegrees);
  } else if (axis === "y" && typeof wasmExports.rotate_current_cube_y === "function") {
    wasmExports.rotate_current_cube_y(displayDegrees);
  } else {
    wasmExports.rotate_current_rectangle_z(displayDegrees);
  }

  currentVertices = currentVerticesFromRust();
  currentFaces = currentFacesFromRust();
  rotation = (rotation + displayDegrees) % 360;
  render();
}

rotateButton.addEventListener("click", () => {
  rotateCurrentDemo(Number(angleSelect.value), axisSelect.value);
});

resetButton.addEventListener("click", () => {
  resetCurrentDemo();
});

versionSelect.addEventListener("change", () => {
  selectVersion(versionSelect.value);
});

axisSelect.addEventListener("change", () => {
  render();
});

showHiddenVerticesInput.addEventListener("change", () => {
  render();
});

canvas.addEventListener("pointerdown", startOrbitDrag);
canvas.addEventListener("pointermove", updateOrbitFromPointer);
canvas.addEventListener("pointerup", stopOrbitDrag);
canvas.addEventListener("pointercancel", stopOrbitDrag);

try {
  loadedWasmExports = await loadWasm();
} catch (error) {
  console.warn(error);
}

selectVersion(versionSelect.value);
