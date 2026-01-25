# PeakUI Reference App (Showcase) Guide

This application serves as both the primary documentation viewer and the feature showcase for the PeakUI framework.

## Directory Structure

```text
crates/peak-ui/src/reference/
├── app.rs            # Main Application State & Update Loop
├── model.rs          # Shared Data Models (Page Enum)
├── mod.rs            # Module definitions
├── page.rs           # Page Trait & PageResult definition
├── pages/            # Content for specific pages
│   ├── components.rs # Showcase gallery for components
│   ├── overview.rs   # Documentation pages
│   └── ...
└── views/            # Structural Views
    ├── content_view.rs # Main Layout Orchestrator (Notch, Dock, Split)
    ├── sidebar.rs      # Navigation Sidebar
    ├── canvas.rs       # Page Router & Renderer
    └── tabbar.rs       # Bottom Tab Bar (Mobile)
```

## Architecture

The app follows The Elm Architecture, strictly implementing the **Model-View-Update (MVU)** pattern via `iced`.

### 1. State (Model)
Defined in `App` (`app.rs`).
- **`active_tab`**: A `Page` enum representing the current view.
- **`navigation_mode`**: High-level mode ("Guide", "Documentation", "Components") which filters available sidebar items.
- **`show_sidebar`**, **`show_inspector`**: boolean flags for UI panels.

### 2. Update
Defined in `App::update`.
- Handles `Message::SetTab(Page)` to switch pages.
- Handles toggles for search, sidebar, etc.
- **Note**: This is the only place where state modification occurs.

### 3. View
The view hierarchy is composed of "Structural Views":

- **`ContentView`**: The root container. It constructs the `SidebarView` and `CanvasView`, and orchestrates the "Notch" (toolbar) and "Dock" (tabbar).
- **`CanvasView`**: Acts as the router. It consumes `active_tab` (Page) and returns a `PageResult`.
- **`SidebarView`**: Renders the navigation tree based on `navigation_mode`. It emits `Message::SetTab`.

## Key Features

- **Responsive Design**: Uses `context.is_slim()` to switch between Sidebar (Desktop) and TabBar (Mobile/Slim).
- **Dynamic Notch**: The top toolbar ("Notch") dynamically updates based on the active page's `toolbar_items`.
- **Inspector**: A collateral view that can be toggled to show detailed info about the current context.

## Navigation Flow

1. User clicks Sidebar Item -> Emits `Message::SetTab(Page::Button)`.
2. `App::update` receives message -> Updates `self.active_tab = Page::Button`.
3. `App::view` -> `ContentView` -> `CanvasView` -> Matches `Page::Button` -> Renders `pages::component_detail::view(...)`.
