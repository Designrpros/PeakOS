```markdown
# PeakUI Framework 🏔️
> **"SwiftUI for Rust" - The Native App Framework of PeakOS**

**PeakUI** is a high-level UI framework built on top of [Iced](https://github.com/iced-rs/iced) (v0.13), designed to enable rapid, beautiful, and consistent application development. It abstracts the complexity of raw widget composition into a declarative, modifier-based syntax inspired by SwiftUI.

**Build Once, Run Everywhere:**
* 🖥️ **Native Desktop:** macOS, Windows, Linux (wGPU accelerated).
* 🌐 **Web (WASM):** Runs directly in modern browsers via WebGL.
* 📱 **Mobile:** Adaptive layouts that switch between Sidebar and Stack navigation automatically.

---

## 🚀 Quick Start: The Showcase
The best way to learn PeakUI is to run the **Showcase Application**. This is a comprehensive "Component Lab" that demonstrates every widget, layout, and hook in the framework.

### Run Native (Desktop)
```bash
cargo run --example showcase

```

### Run on Web (WASM)

*Requires [Trunk*](https://trunkrs.dev/)

```bash
trunk serve peak-ui/index.html

```

---

## 1. Core Philosophy

1. **Declarative Syntax**: Write code that describes *what* the UI should look like, not *how* to draw it.
2. **Adaptive by Default**: Apps automatically switch between "Desktop Mode" (Sidebar + Content) and "Mobile Mode" (Navigation Stack) based on window width.
3. **Visual Consistency**: All apps inherit the PeakOS Design System (Glassmorphism, Typography, Spacing) automatically.

## 2. Architecture: "The One Codebase"

PeakUI enables a truly separated ecosystem where applications are libraries, not binaries.

| Layer | Crate | Responsibilities |
| --- | --- | --- |
| **Framework** | `crates/peak-ui` | The SDK. Widgets, layout logic, theming system, and adaptation rules. |
| **Showcase** | `examples/showcase.rs` | The living documentation and component laboratory. |
| **Applications** | `crates/peak-apps` | The core apps (Settings, Files). Pure logic & layout using `peak-ui`. |
| **Shells** | `modes/*` | Hosts apps in different contexts (Windowed, Fullscreen, Web). |

## 3. Example Usage

```rust
use peak_ui::prelude::*;

struct MyApp {
    show_sidebar: bool,
}

impl View for MyApp {
    fn body(&self) -> impl View {
        NavigationSplitView::new(
            // Master (Sidebar)
            List::new()
                .header("General")
                .item(Label::new("Profile", icon("person.circle")))
                .item(Label::new("Network", icon("wifi"))),
            
            // Detail (Content)
            VStack::new()
                .push(Text::new("Welcome to PeakOS").large_title())
                .push(Button::new("Click Me").style(style::Primary))
                .padding()
        )
        .force_sidebar_on_slim(self.show_sidebar)  // Stack navigation for mobile
        .on_back(Message::GoBack)                  // Automatic back button logic
    }
}

```

## 4. Design System Specs (Cupertino Theme)

PeakUI implements a premium visual language out of the box:

### A. Navigation

* **Sidebar**: Translucent glass background, rounded-rect selection.
* **Toolbar**: Unified with window chrome, large titles.

### B. Controls (Interactive Labs)

* **Toggle**: Green/Grey pill with shadowed knob.
* **Slider**: Thin track, blue fill, expanding knob on drag.
* **Button**: Supports `Variant` (Solid, Soft, Ghost) and `Intent` (Primary, Destructive).

### C. Layouts

* **NavigationSplitView**: The core adaptive container.
* **ZStack**: For overlaying content (like toasts or modals).
* **ResponsiveGrid**: Flows content (cards, images) based on available width.

## 5. Theming

PeakUI supports dynamic theme switching at runtime:

* **Tone**: Light / Dark / Auto.
* **Theme**: Heritage (Classic) / Cupertino (Modern Glass).

---

*To contribute, please see `PEAKUI_GUIDE.md`.*