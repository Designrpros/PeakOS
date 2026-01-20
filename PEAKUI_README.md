# PeakUI Framework 🏔️
> **"SwiftUI for Rust" - The Native App Framework of PeakOS**

**PeakUI** is a high-level UI framework built on top of [Iced](https://github.com/iced-rs/iced) designed to enable rapid, beautiful, and consistent application development for PeakOS. It abstracts the complexity of raw widget composition into a declarative, modifier-based syntax inspired by SwiftUI.

## 1. Core Philosophy
1.  **Declarative Syntax**: Write code that describes *what* the UI should look like, not *how* to draw it.
2.  **Platform Agnostic**: Apps built with PeakUI run natively on Desktop (Windowed) and Mobile (Fullscreen) without code changes.
3.  **Visual Consistency**: All apps inherit the OS Theme (Glassmorphism, Typography, spacing) automatically.

## 2. Architecture: "The One Codebase"
PeakUI enables a separated ecosystem where apps are libraries, not binaries.

| Layer | Crate | Responsibilities |
| :--- | :--- | :--- |
| **Framework** | `crates/peak-ui` | The SDK. Widgets, layout logic, theming system, and adaptation rules. |
| **Applications** | `crates/peak-apps` | The core apps (Settings, Files). Pure logic & layout using `peak-ui`. |
| **Desktop Shell** | `crates/peak-desktop` | Hosts apps in floating *Windows*. Renders `NavigationSplitView` as Sidebar+Detail. |
| **Mobile Shell** | `crates/peak-mobile` | Hosts apps full-screen. Renders `NavigationSplitView` as NavigationStack. |

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
                .push(Toggle::new("Enable Wi-Fi", &self.wifi_enabled))
                .padding()
        )
        .force_sidebar_on_slim(self.show_sidebar)  // Stack navigation for mobile
        .on_back(Message::GoBack)                  // Automatic back button
    }
}
```

**Adaptive Behavior:**
- **Desktop (≥600px)**: Shows sidebar + content side-by-side
- **Mobile (<600px)**: Stack navigation with automatic back button
  - Starts at sidebar (category list)
  - Taps navigate to detail view with "< Back" button
- **Icons**: Buttons support `.icon(name)` for theme-aware graphics

## 4. Design System Specs (Cupertino Theme)
PeakUI implements a premium visual language out of the box.

### A. Navigation
- **Sidebar**: Translucent glass background, rounded-rect selection (blue).
- **Toolbar**: Unified with window chrome, large titles.

### B. Controls
- **Toggle**: Green/Grey pill (50x30px) with white shadowed knob.
- **Slider**: Thin track, blue fill, expanding knob on drag.
- **Button**: `style(.primary)` gives blue gradient, `style(.destructive)` gives red tint.

### C. Lists
- **Inset Grouped**: Rounded white "islands" on a light grey background (Settings style).
- **Separators**: Inset margins (doesn't touch edges).

### D. Overlay Scrollbars
- **Style**: Invisible track.
- **Handle**: Thin (4px) rounded pill floating over content. Expands on hover.

## 5. Theming
PeakUI supports dynamic theme switching at runtime:
- **Heritage**: The classic PeakOS look.
- **Cupertino**: The modern, glass-infused aesthetic.
- **High Contrast**: For accessibility.

---
*This document serves as the architectural blueprint for the PeakUI implementation.*
