# PeakUI Reference App Structure

The PeakUI Reference app is a modular, multi-page application designed to showcase the framework’s capabilities across different navigation modes and device types.

## Architecture

The app follows a strict separation of concerns:

- `app.rs`: The central message hub and orchestration layer.
- `views/`: Reusable shell components (Sidebar, Notch, Dock, Canvas).
- `pages/`: Independent, contextual view modules.

## Orchestration Layer (`ContentView`)

The `ContentView` acts as the intelligent hub. It automatically observes the `PageResult` metadata to:
- **Render Dynamic Overlays**: Show the search icon only if `.searchable()` is used.
- **Automate Mobile UI**: Add a burger menu icon only if `.sidebar_toggle()` is present and the device is mobile.
- **Contextual Sidebars**: Inject an inspector view only if `.inspector()` is configured.

## Page Design

Each page is an independent function that returns a `PageResult`. This allows pages to be highly specialized while still participating in the global navigation and search systems.

```rust
pub fn view(context: &Context, is_mobile: bool) -> PageResult {
    VStack::new(...)
        .push(Text::new("Hello PeakUI"))
        .sidebar_toggle(Message::ToggleSidebar)
}
```

## Navigation Modes

The app supports multiple navigation philosophies:
- **Guide**: Narrative, book-style documentation.
- **Documentation**: Technical API references.
- **Components**: A visual gallery of atoms and controls.
- **Hooks**: State management and lifecycle demonstrations.
- **Settings**: Dynamic theme and scaling customization.

## Key Visual Systems
1. **Dynamic Notch**: A floating pill that adapts its width and content to situational needs.
2. **Floating Dock**: A persistent, theme-aware navigation bar for high-frequency mode switching.
3. **Adaptive Sidebar**: A responsive navigation component that transitions into a full-screen overlay on mobile.
