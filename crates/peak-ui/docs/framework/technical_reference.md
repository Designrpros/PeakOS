# PeakUI Technical Reference

## Core Concepts

PeakUI is designed around a unified "View-Model-Backend" architecture that separates the *definition* of a UI from its *rendering*. This allows the same UI code to run on desktops (Iced), terminals (crossterm), and even be interpreted by AI agents.

### 1. The View Trait

The fundamental unit of PeakUI is the `View` trait. Unlike traditional frameworks where components render directly to pixels or DOM nodes, a PeakUI View is a *blueprint*.

```rust
pub trait View<Message: 'static, B: Backend = IcedBackend> {
    /// Renders the view into the backend's native format.
    fn view(&self, context: &Context) -> B::AnyView<Message>;

    /// Describes the semantic structure of the view for AI agents.
    fn describe(&self, context: &Context) -> SemanticNode;
}
```

- **`view()`**: Transforms the component into the backend's specific visual representation (e.g., an `iced::Element` for GUI, or a `String` for TUI).
- **`describe()`**: Returns a `SemanticNode` tree, which provides a high-level, structured description of the UI (buttons, lists, inputs) without visual noise. This is used by the generic `AIBackend`.

### 2. The Backend Trait

Reflecting the "write once, run anywhere" philosophy, the `Backend` trait abstracts the underlying platform capabilities.

```rust
pub trait Backend: Sized + Clone + 'static {
    type AnyView<Message: 'static>: 'static;

    fn vstack<Message>(...) -> Self::AnyView<Message>;
    fn hstack<Message>(...) -> Self::AnyView<Message>;
    fn text<Message>(...) -> Self::AnyView<Message>;
    fn button<Message>(...) -> Self::AnyView<Message>;
    // ... primitive components
}
```

Current implementations:
- **`IcedBackend`**: Prod-grade GUI using the Iced library (wgpu/OpenGL/Vulkan).
- **`TermBackend`**: Rich TUI renderer using ANSI escape codes and unicode block characters.
- **`AIBackend`**: A "headless" backend that constructs a semantic tree for LLM consumption.

### 3. Context & Theming

Every `view()` call receives a `Context` object. This is the source of truth for the environment.

```rust
pub struct Context {
    pub theme: ThemeTokens,
    pub mode: ShellMode,
    pub device: DeviceType, // Desktop, Mobile, TV
    pub size: Size,
}
```

- **`ThemeTokens`**: A standardized set of semantic colors, spacing, and sizing derived from the current `mode` (e.g., "Desktop Dark", "TV Light"). This ensures consistent styling across the entire OS.
- **`DeviceType`**: Enables responsive logic within components. A component can check `if context.is_slim()` to render a `VStack` instead of an `HStack`.

## Component Architecture

### Atoms vs Containers

- **Atoms**: Indivisible units like `Text`, `Icon`, `Button`. They handle their own rendering via the Backend.
- **Containers**: Layout providers like `VStack`, `HStack`, `ZStack`. They take a list of children (`Vec<Box<dyn View>>`) and arrange them.

### Modifiers

PeakUI uses a modifier pattern similar to SwiftUI:

```rust
Text::new("Hello")
    .color(Color::RED)
    .font(Font::MONOSPACE)
    .padding(20)
```

Implementation involves a `ViewExt` trait (for generic modifiers) or builder methods on the struct itself (for specific properties).

## State Management

PeakUI follows The Elm Architecture (via Iced) but adds a layer of abstraction for Navigation and Page Management:

1.  **Global App State**: Holds the source of truth (e.g., specific data models).
2.  **View State**: Transient state (like scroll position or text input) is often handled by the `Backend`'s native widget system or lifted up to the App State via `Message`.
3.  **Navigation**: The `Page` enum (refactored) strictly defines valid routes, replacing string-based navigation for type safety.

## Agentic capabilities

One of the unique features is the **Inspector** and **Semantic View**.
- **Inspector**: A side-panel that can show debug info or AI analysis of the current page.
- **Semantic Tree**: By calling `.describe()`, an Agent can "see" the UI structure:
    ```json
    {
      "role": "button",
      "label": "Submit",
      "content": null
    }
    ```
    This allows agents to navigate the UI programmatically without needing vision processing.
