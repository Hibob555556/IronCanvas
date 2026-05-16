const canvas = document.querySelector("#canvas");
const ctx = canvas.getContext("2d");
const angleSelect = document.querySelector("#angle-select");
const rotateButton = document.querySelector("#rotate-button");
const resetButton = document.querySelector("#reset-button");
const verticesOutput = document.querySelector("#vertices-output");
const rotationLabel = document.querySelector("#rotation-label");

const topEdgeStartIndex = 2;
const topEdgeEndIndex = 5;
let currentVertices = [];
let rotation = 0;
let wasmExports = null;
let viewBounds = null;

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

function currentVerticesFromRust() {
  const ptr = wasmExports.current_rectangle_vertices_ptr();
  return verticesFromWasm(wasmExports, ptr);
}

function createViewBounds(vertices) {
  const xs = vertices.map((vertex) => vertex[0]);
  const ys = vertices.map((vertex) => vertex[1]);
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

function fitToCanvas() {
  const padding = 64;
  const width = viewBounds.maxX - viewBounds.minX;
  const height = viewBounds.maxY - viewBounds.minY;
  const scale = Math.min(
    (canvas.width - padding * 2) / width,
    (canvas.height - padding * 2) / height,
  );

  return (vertex) => [
    padding + (vertex[0] - viewBounds.minX) * scale,
    canvas.height - padding - (vertex[1] - viewBounds.minY) * scale,
  ];
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

function drawVertices(vertices) {
  const project = fitToCanvas();

  drawGrid();

  ctx.strokeStyle = "#7cf0cf";
  ctx.lineWidth = 5;
  ctx.lineJoin = "round";
  ctx.lineCap = "round";
  ctx.beginPath();

  vertices.forEach((vertex, index) => {
    const [x, y] = project(vertex);
    if (index === 0) {
      ctx.moveTo(x, y);
    } else {
      ctx.lineTo(x, y);
    }
  });

  ctx.stroke();

  vertices.forEach((vertex, index) => {
    const [x, y] = project(vertex);
    ctx.fillStyle = "#f3f7ff";
    ctx.beginPath();
    ctx.arc(x, y, 6, 0, Math.PI * 2);
    ctx.fill();
  });

  const [startX, startY] = project(vertices[topEdgeStartIndex]);
  const [endX, endY] = project(vertices[topEdgeEndIndex]);
  const labelX = (startX + endX) / 2;
  const labelY = (startY + endY) / 2;

  ctx.fillStyle = "#ffd279";
  ctx.font = "700 16px system-ui, sans-serif";
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText("TOP", labelX, labelY - 18);
}

function formatVertices(vertices) {
  return vertices
    .map((vertex, index) => {
      const [x, y, z] = vertex.map((value) => {
        const displayValue = Math.abs(value) < 0.000001 ? 0 : value;
        return displayValue.toFixed(2);
      });
      return `${String(index).padStart(2, "0")}: [${x}, ${y}, ${z}]`;
    })
    .join("\n");
}

function render() {
  drawVertices(currentVertices);
  verticesOutput.textContent = formatVertices(currentVertices);
  rotationLabel.textContent = `${rotation} degrees`;
}

async function loadWasm() {
  const response = await fetch("./iron_canvas.wasm");
  const bytes = await response.arrayBuffer();
  const wasm = await WebAssembly.instantiate(bytes);
  return wasm.instance.exports;
}

rotateButton.addEventListener("click", () => {
  const degrees = Number(angleSelect.value);
  wasmExports.rotate_current_rectangle_z(degrees);
  currentVertices = currentVerticesFromRust();
  rotation = (rotation + degrees) % 360;
  render();
});

resetButton.addEventListener("click", () => {
  wasmExports.reset_current_rectangle();
  currentVertices = currentVerticesFromRust();
  rotation = 0;
  render();
});

try {
  wasmExports = await loadWasm();
  wasmExports.reset_current_rectangle();
  currentVertices = currentVerticesFromRust();
  viewBounds = createViewBounds(currentVertices);
  render();
} catch (error) {
  verticesOutput.textContent = `Could not load WASM: ${error.message}`;
}
