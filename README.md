# Iron Canvas

Iron Canvas is a small Rust-to-WebAssembly geometry demo that renders and rotates a rectangle in the browser. The project is intentionally compact, but it is built like a real frontend systems project: Rust owns the geometry, WebAssembly exposes a narrow runtime API, and JavaScript handles only browser rendering and UI orchestration.

The result is a visual playground for understanding how typed Rust data can move through WASM memory and become an interactive canvas experience.

## Highlights

- Rust geometry engine compiled to WebAssembly
- Canvas UI with rotate/reset controls
- Runtime `rotate_z` transformation with 45-degree and 90-degree options
- JavaScript does not perform rotation math
- Original vertex list remains the single source of truth
- Rust unit tests, Rust integration tests, web static tests, and WASM runtime tests
- GitHub Actions CI and GitHub Pages deployment workflow
- Husky pre-commit hook for local quality checks

## Demo Behavior

The app starts with a closed rectangle made from 11 vertices. The browser reads the vertex buffer from WASM memory, draws it to a canvas, and prints the current coordinates beside it.

When the user selects an angle and clicks `Rotate`, JavaScript calls the Rust/WASM API:

```js
wasmExports.rotate_current_rectangle_z(degrees);
```

Rust mutates the current vertex buffer, JavaScript reads the updated buffer, and the canvas redraws. The `TOP` label marks the rectangle's original top edge so orientation is easy to track after rotation.

## Architecture

```text
IronCanvas/
  src/
    lib.rs              Rust geometry and WASM exports
    rectangle.rs        Native CLI demo
  web/
    index.html          Browser UI shell
    main.js             Canvas rendering and WASM calls
    styles.css          Portfolio-style visual design
    *.test.mjs          Web and WASM tests
  tests/
    rectangle_api.rs    Rust integration tests
  scripts/
    build-wasm.ps1      Builds and copies WASM into web/
    test.ps1            Full local/CI test suite
    test-web.ps1        Web/WASM test suite
```

## Rust/WASM Boundary

The Rust library exports a small C-compatible API:

```rust
rectangle_vertex_count()
rectangle_vertices_ptr()
current_rectangle_vertices_ptr()
reset_current_rectangle()
rotate_current_rectangle_z(degrees)
rotate_current_rectangle_90_clockwise()
```

`VERTICES` is the single source of truth. Rotated geometry is not precomputed or stored as a separate constant. The current runtime buffer is reset from `VERTICES`, then transformed by iterating over each vertex and calling:

```rust
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
cargo wasm
```

## Test Coverage

The test suite checks the project at several layers:

- Rust unit tests validate rotation behavior, center stability, and zero clamping.
- Rust integration tests validate the public rectangle API, bounds, closure, and repeated rotations.
- Web static tests validate the HTML/JS contract and verify rotation math is not implemented in JavaScript.
- WASM runtime tests instantiate the compiled module, inspect exports, reset vertices, rotate vertices, and verify negative zero does not leak into output.

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
- automated test coverage across native and WASM targets
- CI/CD deployment for a static WebAssembly demo

It is a good foundation for growing into richer geometry tools, drawing systems, collision demos, or a small Rust-powered graphics engine.
