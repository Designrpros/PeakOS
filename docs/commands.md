# PeakOS Project Commands

This document provides a reference for the most common commands used to build, run, and test PeakOS components.

## 🖥️ Native Execution (Desktop)

To run the main PeakOS desktop environment:

```bash
cargo run -p peak-desktop
```

To run the UI component showcase as a native app:

```bash
cargo run -p peak-ui
```

## 🌐 Web Architecture (WASM)

PeakUI can be rendered in the browser using [Trunk](https://trunkrs.dev/).

### Development Server
Run the UI showcase with live reload (default port 8080):

```bash
cd crates/peak-ui
trunk serve --port 8080
```

### Production Build
Compile the WASM and glue code for static hosting:

```bash
cd crates/peak-ui
trunk build --release
```

### WASM Compatibility Check
Verify that the codebase compiles for the `wasm32-unknown-unknown` target:

```bash
cargo check --target wasm32-unknown-unknown --all-features
```

### WASM Development
- `trunk serve --port 8080` (Run the WASM Showcase)
- `./scripts/detect_wasm_leaks.sh` (Audit for non-'static captures)

## 📦 Deployment & ISO Generation

Scripts for creating bootable PeakOS images (Alpine-based).

### ARM Architecture (Raspberry Pi, Pine64, etc.)
```bash
./crates/peak-deploy/build.sh --arm
```

### Intel Architecture (x86_64)
```bash
./crates/peak-deploy/build.sh --intel
```

### Options
- `--native`: Force a native build on the host (requires Linux/Alpine).
- `--docker`: Force a Docker-based build (recommended for macOS).
- `--skip-check`: Skip pre-build environment validation.

## 🧪 Testing & Quality

Run the full project test suite:

```bash
cargo test --workspace
```

Run linting checks (Clippy):

```bash
cargo clippy --workspace -- -D warnings
```

Format the codebase:

```bash
cargo fmt --all
```
