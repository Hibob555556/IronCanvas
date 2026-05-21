# Iron Canvas

Current demo version: `v0.4.1`

[![CI](https://github.com/Hibob555556/IronCanvas/actions/workflows/ci.yml/badge.svg)](https://github.com/Hibob555556/IronCanvas/actions/workflows/ci.yml)
[![Deploy Web Demo](https://github.com/Hibob555556/IronCanvas/actions/workflows/pages.yml/badge.svg)](https://github.com/Hibob555556/IronCanvas/actions/workflows/pages.yml)

Iron Canvas is a small Rust-to-WebAssembly geometry demo that renders versioned geometry in the browser. It began as a flat rectangle rotation demo and now includes a wireframe cube, shaded cube faces, an orbit camera, lighting, and a changelog-backed version switcher.

The project stays intentionally compact, but it is built like a real frontend systems project: Rust owns the live geometry buffers, transforms, camera-space vertices, and face shading; WebAssembly exposes a narrow runtime API; and JavaScript handles browser controls, canvas drawing, and version presentation.

Repository: [github.com/Hibob555556/IronCanvas](https://github.com/Hibob555556/IronCanvas)

## Highlights

- Rust geometry engine compiled to WebAssembly
- Version switcher for `v0.1.0` rectangle, `v0.2.0` wire cube, `v0.3.2` shaded cube, and `v0.4.1` orbit cube
- Canvas UI with version, angle, axis, vertex visibility, rotate, and reset controls
- Cube edge vertices plus Rust-owned face vertices for shaded rendering
- X, Y, and Z rotation options
- Axis guide overlays for legacy cube views and drag-to-orbit controls for `v0.4.1`
- Rust owns rotation, orbit camera vertex transforms, lighting, shading, and solid face colors in the primary WASM path
- Rust unit tests, Rust integration tests, web static tests, and WASM runtime tests
- GitHub Actions CI and GitHub Pages deployment workflow

## Demo Behavior

The latest version starts with a cube represented as 12 explicit edge pairs and six quad faces. The browser reads Rust-owned edge, face, camera, and face-color buffers from WASM memory, draws solid lit faces, and prints the current camera-space vertex coordinates beside it. A vertex visibility toggle can switch the coordinate readout between the full vertex buffer and vertices that are not hidden behind nearer faces or overlapping vertices.

When the user selects an angle and axis, then clicks `Rotate`, JavaScript calls the matching Rust/WASM export:

```js
wasmExports.rotate_current_cube_x(degrees);
wasmExports.rotate_current_cube_y(degrees);
wasmExports.rotate_current_rectangle_z(degrees);
```

The `rectangle_*` names remain for compatibility with the original demo, but the current default geometry is a cube. Rust mutates the runtime buffers, JavaScript reads the updated WASM memory, and the canvas redraws. Older cube versions keep the `TOP` label and colored axis guides; `v0.4.1` uses solid walls, Rust-owned lighting, drag-to-orbit inspection, and a toggleable hidden-vertex readout.

If `web/iron_canvas.wasm` is missing during local development, the page falls back to a browser-side runtime for the older geometry demos. Rebuilding the WASM file restores the full Rust-backed `v0.4.1` camera and shading path.

## Versions

| Version | Demo | Notes |
| --- | --- | --- |
| `v0.4.1` | Orbit cube | Adds a hidden-vertex toggle for switching the coordinate readout between visible vertices and the full vertex buffer. |
| `v0.4.0` | Orbit cube | Adds solid lit faces and Rust-owned camera vertex transforms for drag-to-orbit inspection. |
| `v0.3.2` | Shaded cube | Improves 45-degree readability with a steadier canvas camera and camera-depth face ordering. |
| `v0.3.1` | Shaded cube | Removes the whole-cube shortcut, keeps explicit X/Y/Z rotation, and stabilizes rotation framing. |
| `v0.3.0` | Shaded cube | Adds Rust-owned face vertices, translucent face rendering, selected-axis guides, and X/Y/Z rotation controls. |
| `v0.2.0` | Wire cube | Replaces the flat rectangle with 12 cube edge pairs and a projected 3D view. |
| `v0.1.0` | Rectangle rotation | Preserves the original flat rectangle outline and clockwise Z-axis rotation behavior. |

## Architecture

```text
IronCanvas/
  src/
    lib.rs              Rust geometry buffers, transforms, and WASM exports
    rectangle.rs        Native CLI demo
  web/
    index.html          Browser UI shell, version selector, and changelog
    main.js             Canvas rendering, WASM calls, and dev fallback runtime
    styles.css          Portfolio-style visual design
    *.test.mjs          Web static and WASM runtime tests
  tests/
    rectangle_api.rs    Rust integration tests for the compatibility API
  scripts/
    build-wasm.ps1      Builds and copies WASM into web/
    test.ps1            Full local/CI test suite
    test-web.ps1        Web/WASM test suite
```

## Rust/WASM Boundary

The Rust library exports a small C-compatible API. Some names still reference `rectangle` because they are the original public boundary:

```rust
rectangle_vertex_count()
rectangle_vertices_ptr()
current_rectangle_vertices_ptr()
cube_face_vertex_count()
current_cube_face_vertices_ptr()
current_cube_camera_vertices_ptr()
current_cube_camera_face_vertices_ptr()
cube_face_color_count()
current_cube_face_colors_ptr()
reset_current_rectangle()
set_current_cube_camera(yaw, pitch)
rotate_current_rectangle_z(degrees)
rotate_current_rectangle_90_clockwise()
rotate_current_cube_x(degrees)
rotate_current_cube_y(degrees)
```

`VERTICES` stores the cube wireframe as edge pairs. `FACE_VERTICES` stores six quad faces. Runtime buffers are reset from those constants, then transformed by iterating over each vertex and calling axis-specific Rust helpers:

```rust
pub fn rotate_x(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3]
pub fn rotate_y(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3]
pub fn rotate_z(position: [f32; 3], center: [f32; 3], degrees: f32) -> [f32; 3]
```

Positive degree values rotate clockwise for the browser UI.

## Running Locally

Install the Rust WASM target:

```powershell
rustup target add wasm32-unknown-unknown
```

Install Node dependencies:

```powershell
npm install
```

Build the WASM file into `web/`:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\build-wasm.ps1
```

Serve the web folder:

```powershell
cd web
python -m http.server 8080
```

Open:

```text
http://localhost:8080
```

## Useful Commands

Run the full CI-style check:

```powershell
npm run check
```

Run all tests:

```powershell
npm test
```

Run only Rust tests:

```powershell
npm run test:rust
```

Run only web/WASM tests:

```powershell
npm run test:web
```

Build WASM directly through Cargo:

```powershell
cargo build --target wasm32-unknown-unknown --release --lib
```

## Test Coverage

The test suite checks the project at several layers:

- Rust unit tests validate rotation behavior, center stability, camera transforms, face color buffers, and zero clamping.
- Rust integration tests validate the compatibility API, cube bounds, face counts, axis rotation, camera distance stability, and repeated rotations.
- Web static tests validate the HTML/JS contract, version controls, changelog content, orbit helper usage, and WASM delegation hooks.
- Web behavior tests validate orbit drag direction, pitch clamping, and view-aware rotation signs.
- WASM runtime tests instantiate the compiled module, inspect exports, reset vertices, rotate vertices, validate camera-owned vertex/color buffers, and verify negative zero does not leak into output.

The local Husky pre-commit hook runs:

```powershell
npm run check
```

## CI/CD

GitHub Actions are configured in `.github/workflows/`.

`ci.yml` runs on pushes, pull requests, and manual dispatch:

- sets up Rust and Node
- installs the WASM target
- checks Rust formatting
- runs the full test suite
- uploads the generated WASM artifact

`pages.yml` builds and tests the project, then publishes the `web/` directory to GitHub Pages.

For deployment, configure GitHub Pages to use **GitHub Actions** as the source.

## Why This Project Matters

Iron Canvas is deliberately modest in scope, but it demonstrates a production-minded workflow around a low-level browser feature:

- memory layout with `#[repr(C)]`
- WASM export design
- Rust-owned transformation logic
- browser canvas rendering
- versioned UI behavior and changelog presentation
- automated test coverage across native and WASM targets
- CI/CD deployment for a static WebAssembly demo

It is a good foundation for growing into richer geometry tools, drawing systems, collision demos, or a small Rust-powered graphics engine.
