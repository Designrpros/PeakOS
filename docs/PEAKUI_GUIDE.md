# PeakUI Developer Guide

PeakUI is a declarative, semantic UI framework built on top of `iced` 0.13, designed specifically for the PeakOS ecosystem. It facilitates building adaptive, theme-aware interfaces that work seamlessly across Desktop, Mobile, and TV.

## Core Concepts

### 1. Declarative Views
PeakUI uses a `View` trait instead of traditional widget composition. This allows for a more "SwiftUI-like" experience.

```rust
pub trait View<Message> {
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer>;
}
```

### 2. Context-Awareness
Every view receives a `Context` containing environmental data:
- **ThemeTokens**: Dynamic colors and metrics.
- **ShellMode**: The current OS mode (Desktop, Mobile, etc.).
- **DeviceType**: Categorized hardware type for layout branching.

---

## Basic Components

### Typography
Use the `Text` component for all textual content. It supports semantic modifiers.

```rust
Text::new("My Title").title()
Text::new("Body content").body()
Text::new("Subtle description").secondary()
```

### Layouts
Layout primitives handle spacing and alignment automatically.

- **VStack**: Vertical stack.
- **HStack**: Horizontal stack.
- **ZStack**: Overlapping stack (layers).

```rust
VStack::new()
    .spacing(10.0)
    .push(Text::new("Item 1"))
    .push(Text::new("Item 2"))
```

### Containers
- **Card**: A styled container with background and padding.
- **Section**: A labeled group of components.

```rust
Section::new("Profile", 
    Card::new(
        VStack::new()
            .push(Text::new("Name: Peak"))
            .push(Text::new("Status: Online"))
    )
)
```

---

## Advanced: Adaptive Layouts

The `NavigationSplitView` is the flagship adaptive component. It automatically switches between a two-pane layout on Desktop and a single-pane layout on Mobile/TV.

```rust
NavigationSplitView::new(sidebar_view, content_view)
```

### Stack Navigation (Mobile)
On mobile devices, `NavigationSplitView` can behave as a stack navigator with automatic back button:

```rust
NavigationSplitView::new(sidebar_view, content_view)
    .force_sidebar_on_slim(show_sidebar)  // Control which view shows
    .on_back(Message::GoBack)             // Handle back navigation
```

When `force_sidebar_on_slim` is `false`, a "Back" button with chevron icon is automatically rendered, allowing users to return to the sidebar.

### Buttons with Icons
Buttons now support optional icons that adapt to the button's style and theme:

```rust
Button::new("Back")
    .icon("chevron_left")              // Icon name from assets/icons/system/ui/
    .style(ButtonStyle::Ghost)
    .on_press(Message::GoBack)
```

Icons automatically receive proper theming:
- **Primary/Destructive**: White icons
- **Secondary/Ghost**: Theme text color

---

## Design System Integration

PeakUI is directly wired to `peak-theme`. When the OS theme changes (Dark/Light) or the mode changes (Console -> Desktop), all PeakUI components update automatically because they resolve their styles through the `Context`.

### Custom Styling in Views
If you need custom colors, access the theme via the context:

```rust
fn view(&self, context: &Context) -> Element<'static, Message, Theme, Renderer> {
    container(self.content.view(context))
        .style(|_theme| container::Style {
            background: Some(context.theme.card_bg.into()),
            ..Default::default()
        })
        .into()
}
```

---

## Running the Showcase
To see PeakUI in action, run the showcase application:

```bash
cargo run -p peak-ui --bin peak-ui-showcase
```
