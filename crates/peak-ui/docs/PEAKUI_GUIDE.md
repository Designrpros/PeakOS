```markdown
# PeakUI Developer Guide 📘

Welcome to PeakUI. This guide is designed to get you building beautiful, adaptive applications for PeakOS (and the web) immediately.

## 1. The Golden Rule
> **"Look at the Showcase."**

The most up-to-date, comprehensive documentation is the code itself. The **Showcase Application** is not just a demo; it is the reference implementation for every feature in the framework.

Open the following files to see how a real app is built:
* **Entry Point:** `peak-ui/examples/showcase.rs`
* **State Management:** `peak-ui/src/reference/app.rs`
* **Routing:** `peak-ui/src/reference/model.rs`

## 2. Project Structure

A typical PeakUI app follows The Elm Architecture (Model-View-Update), consisting of three main parts:

### A. The Model (`Page` & `State`)
Define your application state and navigation routes in a dedicated module (e.g., `model.rs`).

```rust
// From reference/model.rs
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Page {
    #[default]
    Home,
    Settings,
    // Deep linking support
    Profile(String), 
}

pub struct AppState {
    pub theme: ThemeTone,
    pub navigation_mode: String,
}

```

### B. The Update Loop (`Message`)

Handle events purely. The `update` function is the only place where state changes.

```rust
#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Page),
    ToggleSidebar,
    // Form inputs
    Search(String), 
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Navigate(page) => {
                self.active_page = page;
                Task::none()
            }
            _ => Task::none()
        }
    }
}

```

### C. The View (`NavigationSplitView`)

Describe your UI declaratively. Always start with a `NavigationSplitView` to get free mobile/desktop adaptation.

```rust
fn view(&self) -> Element<Message> {
    NavigationSplitView::new(
        // Master (Sidebar)
        Sidebar::new(&self.active_page),
        
        // Detail (Content)
        Content::new(&self.active_page)
    )
    .force_sidebar_on_slim(self.show_sidebar) // Handles mobile stack navigation
    .into()
}

```

## 3. Common Patterns

### Responsive Design

Do not check for screen width manually. Use the environment context or responsive widgets to adapt automatically.

* **`NavigationSplitView`**: Automatically switches between Sidebar (Desktop) and Stack (Mobile) layouts.
* **`ResponsiveGrid`**: Flows content (like cards or images) based on available width, perfect for galleries.

### Theming

PeakUI uses a token-based design system found in `peak-theme`.

* **Colors**: Use `style::Primary`, `style::Destructive`, or `style::Glass`.
* **Typography**: Use modifiers like `.large_title()`, `.caption()`, or `.monospaced()`.

### Interactive "Labs"

If you need to test a component (like a Button or Slider) with various configurations, look at the **Component Labs** in the Showcase (`reference/pages/button.rs`). These show how to bind UI controls to internal state for real-time adjustments.

## 4. Contributing

1. Make changes to the framework in `crates/peak-ui`.
2. Run `cargo run --example showcase` to verify your changes visually.
3. If adding a new component, add a new page to the Showcase in `reference/pages/` so others can learn how to use it.
