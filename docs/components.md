# PeakUI: Comprehensive Component Reference

This document serves as the "wishlist" and specification for all components in the PeakUI Standard Library.

## 1. Atoms (Basic Elements)
| Component | Description | Status |
|-----------|-------------|--------|
| `Text` | Multi-style typographic element (Title, Body, Caption). | [x] |
| `Icon` | SVG-based system symbols. | [x] |
| `Divider` | Horizontal/Vertical separators. | [x] |
| `Button` | Interactive trigger with hover/press states. | [x] |
| `TextField` | Single-line text input. | [ ] |
| `Slider` | Continuous value selection. | [ ] |
| `Toggle` | Boolean switch (Cupertino style). | [ ] |
| `ProgressBar` | Visual progress indicator. | [ ] |
| `Badge` | Small status/count overlay. | [ ] |
| `Image` | Local or remote image rendering. | [ ] |

## 2. Containers & Layout
| Component | Description | Status |
|-----------|-------------|--------|
| `VStack` | Vertical arrangement with spacing/alignment. | [x] |
| `HStack` | Horizontal arrangement. | [x] |
| `ZStack` | Layered arrangement (Depth). | [ ] |
| `ScrollView` | Scrollable container (Fixed constraints). | [/] |
| `Spacer` | Flex-space for distribution. | [x] |
| `Card` | Rounded container with shadow/glass effect. | [ ] |
| `Grid` | 2D responsive masonry/grid layout. | [ ] |
| `Group` | Logical grouping without layout impact. | [x] |

## 3. Navigation & High-Level
| Component | Description | Status |
|-----------|-------------|--------|
| `NavigationSplitView` | Desktop sidebar + detail layout. | [x] |
| `NavigationStack` | Push/Pop mobile-style navigation. | [ ] |
| `TabView` | Tab-bar based navigation (Bottom or Top). | [ ] |
| `Sheet` | Modal overlay (Swipe-to-dismiss). | [ ] |
| `Alert` | System-level notification dialog. | [ ] |
| `SegmentedPicker` | Toggle-strip for multi-choice. | [ ] |

## 4. Advanced Components
| Component | Description | Status |
|-----------|-------------|--------|
| `ListView` | Lazy-loading performant lists. | [ ] |
| `DataTable` | Sorting/Filtering data representation. | [ ] |
| `Chart` | Basic Line/Bar/Pie visualizer. | [ ] |
| `RichTextEditor` | Markdown/Formatted text area. | [ ] |
| `ColorPicker` | System-native color selection. | [ ] |
| `DatePicker` | Calendar-based date selection. | [ ] |

## 5. Intelligence (AI-Native)
| Feature | Description | Status |
|-----------|-------------|--------|
| `.describe()` | Semantic text representation for LLMs. | [ ] |
| `AIPanel` | Built-in reflective view for AI agents. | [ ] |
| `SemanticGroup` | Logical metadata grouping for AI focus. | [ ] |
