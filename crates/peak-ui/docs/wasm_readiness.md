# PeakUI WASM Compatibility Guide

This guide ensures that PeakUI remains "future-proof" for WebAssembly (WASM) environments.

## Core Rule: The `'static` Lifetime

All UI elements returned by the `View` trait must satisfy the `'static` lifetime. This is mandatory for `iced` builds targeting WASM.

### ❌ Problem: Reference Leaks
Capturing a reference in a style closure or using `&str` in long-lived view components breaks WASM compatibility.
```rust
// Fails on WASM
.style(move |theme, status| {
    let color = &theme.colors.primary; // Reference!
    container::Style { background: Some((*color).into()) }
})

// Also problematic: View components taking &str
pub fn view(title: &str) -> Element<'a, Message> // 'a is not 'static!
```

### ✅ Solution: Primitive Captures & Owned Strings
Force copies of primitive values (colors, floats) before the closure and use `String`.
```rust
// Works on WASM
.style({
    let color = theme.colors.primary; // Copy
    move |_, _| container::Style { background: Some(color.into()) }
})

pub fn view(title: String) -> Element<'static, Message> // Truly 'static
```

## Specific API Changes

The following components now require `String` instead of `&str`:
- `peak_ui::alert::SystemAlert::view`
- `peak_ui::window_chrome::view`
- `peak_ui::segmented_picker::SegmentedPicker::new`


## Rendering Safeguards

To prevent the "Create clip mask" panic in software renderers (Tiny-Skia):
1. **Neutralize Radii**: Use the `core::radius(f32)` helper which returns 0.0 on WASM.
2. **Neutralize Shadows**: Use the `core::shadow(...)` helper which returns a null shadow on WASM.
3. **Canvas Sizing**: Ensure your `index.html` has a canvas with non-zero dimensions.

## Rendering Strategy (Tiered)

To ensure stability across environments, PeakUI uses different rendering backends:

1. **Native Desktop**: `WGPU`. Full hardware acceleration with modern features (Shadows, Blurs, Antialiasing).
2. **Web (WASM)**: `WebGL`. Stable hardware acceleration. Although WebGPU is the future, WebGL is currently the "future-proof" stable standard for wide browser compatibility in 2025/2026.
3. **Software Fallback**: `tiny-skia` (Deprecated for WASM). Currently avoided due to masking panics on complex clipping paths.

### ⚠️ WASM Feature Flags
The `wasm` feature in `Cargo.toml` is currently pinned to `iced/webgl` to prevent initialization conflicts. Do not enable `iced/wgpu` for WASM builds unless WebGPU support is explicitly required and tested.
