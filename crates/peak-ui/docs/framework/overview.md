# PeakUI Framework Overview

PeakUI is a production-grade, universal UI framework written in Rust. It is designed to bridge the gap between high-performance systems programming and the legendary developer ergonomics of declarative UI frameworks like SwiftUI.

## Core Architecture

PeakUI is built on a "Multi-Backend" philosophy, allowing a single view tree to be rendered across vastly different environments:

1. **IcedBackend (GUI)**: Renders beautiful web and desktop applications with glassmorphism, GPU-accelerated shadows, and premium typography.
2. **TermBackend (TUI)**: Renders high-fidelity terminal interfaces with ASCII/Unicode patterns.
3. **AIBackend (Semantic)**: Renders UI trees into semantic metadata for LLM extraction and agentic introspection.

## The Semantic Core

At the heart of PeakUI is the `View` trait:

```rust
pub trait View<Message, B: Backend> {
    fn view(&self, context: &Context) -> B::AnyView<Message>;
    fn describe(&self, context: &Context) -> SemanticNode;
}
```

This allows every component to be self-describing, enabling AI agents to navigate and interact with the UI with 100% precision.

## SwiftUI-Style Modifiers (`ViewExt`)

PeakUI provides a chainable API for contextual UI features:

```rust
VStack::new(...)
    .padding(20)
    .searchable(query, "Search...", |s| Message::Search(s))
    .sidebar_toggle(Message::ToggleSidebar)
    .toolbar(ToolbarItem::new().icon("plus"))
```

## Theme Engine

The framework is powered by `ThemeTokens`, a standardized design system that controls:
- **Colors**: Primary, Secondary, Surface, Divider, and Intent-based colors.
- **Scaling**: Resolution-independent sizing for consistent cross-device rendering.
- **Radius & Shadows**: GPU-optimized visual effects with platform-aware fallbacks.

## Goal
To become the definitive standard for building intelligent, universal, and visually stunning software in the agentic era.
