use crate::atoms::{Divider, Icon, Image, Rectangle};
use crate::core::{Context, View};
use crate::prelude::*;
use iced::{Element, Length, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Design,
    System,
    Layouts,
    Collections,
    Navigation,
    Content,
    Controls,
    View,
}

impl Category {
    fn title(&self) -> &'static str {
        match self {
            Self::Design => "Design",
            Self::System => "System",
            Self::Layouts => "Layouts",
            Self::Collections => "Collections",
            Self::Navigation => "Navigation",
            Self::Content => "Content",
            Self::Controls => "Controls",
            Self::View => "View",
        }
    }
}

#[derive(Clone)]
pub struct CatalogItem {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub category: Category,
    pub render: fn(&Context) -> Box<dyn View<CatalogMessage>>,
}

#[derive(Debug, Clone)]
pub enum CatalogMessage {
    ItemSelected(&'static str),
    GoBack,
    ToggleInspector,
    // Control States
    ToggleChanged(bool),
    SliderChanged(f32),
    StepperChanged(i32),
    PickerChanged(usize),
    TextChanged(String),
    ToggleAlert(bool),
    ThemeChanged(peak_theme::PeakTheme),
    ToneChanged(peak_theme::ThemeTone),
    None,
}

#[derive(Clone)]
pub struct Catalog {
    pub selected_id: Option<&'static str>,
    pub inspector_open: bool,
    pub items: Vec<CatalogItem>,
    // Interactive State
    pub toggle_value: bool,
    pub slider_value: f32,
    pub stepper_value: i32,
    pub picker_value: usize,
    pub text_value: String,
    pub show_alert: bool,
    // Theme State
    pub theme: peak_theme::PeakTheme,
    pub tone: peak_theme::ThemeTone,
}

impl Catalog {
    pub fn new() -> Self {
        Self {
            selected_id: Some("typography"),
            inspector_open: true,
            items: Self::build_items(),
            toggle_value: false,
            slider_value: 50.0,
            stepper_value: 1,
            picker_value: 0,
            text_value: String::new(),
            show_alert: false,
            theme: peak_theme::PeakTheme::Cupertino,
            tone: peak_theme::ThemeTone::Light,
        }
    }

    pub fn update(&mut self, message: CatalogMessage) -> Task<CatalogMessage> {
        match message {
            CatalogMessage::ItemSelected(id) => {
                self.selected_id = Some(id);
                Task::none()
            }
            CatalogMessage::GoBack => {
                self.selected_id = None;
                Task::none()
            }
            CatalogMessage::ToggleInspector => {
                self.inspector_open = !self.inspector_open;
                Task::none()
            }
            CatalogMessage::ToggleChanged(val) => {
                self.toggle_value = val;
                Task::none()
            }
            CatalogMessage::SliderChanged(val) => {
                self.slider_value = val;
                Task::none()
            }
            CatalogMessage::StepperChanged(val) => {
                self.stepper_value = val;
                Task::none()
            }
            CatalogMessage::PickerChanged(val) => {
                self.picker_value = val;
                Task::none()
            }
            CatalogMessage::TextChanged(val) => {
                self.text_value = val;
                Task::none()
            }
            CatalogMessage::ToggleAlert(show) => {
                self.show_alert = show;
                Task::none()
            }
            CatalogMessage::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            CatalogMessage::ToneChanged(tone) => {
                self.tone = tone;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn build_items() -> Vec<CatalogItem> {
        vec![
            // --- Design ---
            CatalogItem {
                id: "themes",
                title: "Themes",
                description: "Visual identity and dark mode support.",
                category: Category::Design,
                render: render_coming_soon, // Will be overridden in view dispatch
            },
            CatalogItem {
                id: "colors",
                title: "Colors",
                description: "System colors and semantic tokens.",
                category: Category::Design,
                render: render_colors,
            },
            CatalogItem {
                id: "gradient",
                title: "Gradient",
                description: "Linear, Radial, and Angular gradients.",
                category: Category::Design,
                render: render_coming_soon,
            },
            // --- System ---
            CatalogItem {
                id: "system",
                title: "System Info",
                description: "OS, Device, and Screen details.",
                category: Category::System,
                render: render_coming_soon,
            },
            // --- Layouts ---
            CatalogItem {
                id: "stacks",
                title: "Stacks",
                description: "VStack, HStack, and ZStack.",
                category: Category::Layouts,
                render: render_stacks,
            },
            CatalogItem {
                id: "spacer",
                title: "Spacer",
                description: "Flexible space between views.",
                category: Category::Layouts,
                render: render_coming_soon,
            },
            CatalogItem {
                id: "padding",
                title: "Padding",
                description: "Spacing around views.",
                category: Category::Layouts,
                render: render_coming_soon,
            },
            CatalogItem {
                id: "frame",
                title: "Frame",
                description: "Positioning and sizing constraints.",
                category: Category::Layouts,
                render: render_coming_soon,
            },
            CatalogItem {
                id: "geometry",
                title: "Geometry Reader",
                description: "Size-dependent layout logic.",
                category: Category::Layouts,
                render: render_coming_soon,
            },
            // --- Collections ---
            CatalogItem {
                id: "grid",
                title: "Grid",
                description: "Fixed and responsive grid layouts.",
                category: Category::Collections,
                render: render_grid,
            },
            CatalogItem {
                id: "lazy_grid",
                title: "Lazy Grids",
                description: "Scrollable large-dataset grids.",
                category: Category::Collections,
                render: render_coming_soon,
            },
            CatalogItem {
                id: "scroll",
                title: "Scroll View",
                description: "Scrollable content containers.",
                category: Category::Collections,
                render: render_scroll,
            },
            CatalogItem {
                id: "tab_view",
                title: "Tab View",
                description: "Tab-based navigation.",
                category: Category::Collections,
                render: render_coming_soon,
            },
            // --- Navigation ---
            CatalogItem {
                id: "nav_split",
                title: "Navigation Split View",
                description: "Two or three pane navigation.",
                category: Category::Navigation,
                render: render_navigation,
            },
            CatalogItem {
                id: "nav_stack",
                title: "Navigation Stack",
                description: "Push/Pop navigation.",
                category: Category::Navigation,
                render: render_coming_soon,
            },
            // --- Content ---
            CatalogItem {
                id: "typography",
                title: "Typography",
                description: "Font hierarchy and text styles.",
                category: Category::Content,
                render: render_typography,
            },
            CatalogItem {
                id: "icons",
                title: "Icons",
                description: "System icons and symbols.",
                category: Category::Content,
                render: render_icons,
            },
            CatalogItem {
                id: "text",
                title: "Text",
                description: "Text variants and labels.",
                category: Category::Content,
                render: render_content, // Reusing render_content for generic content demos
            },
            CatalogItem {
                id: "text_field",
                title: "Text Field",
                description: "Single line text input.",
                category: Category::Content,
                render: render_inputs_placeholder, // Specifically for inputs
            },
            CatalogItem {
                id: "image",
                title: "Image",
                description: "Asset and memory images.",
                category: Category::Content,
                render: render_content,
            },
            CatalogItem {
                id: "divider",
                title: "Divider",
                description: "Visual separators.",
                category: Category::Content,
                render: render_content,
            },
            CatalogItem {
                id: "shape",
                title: "Shape",
                description: "Rectangle, Circle, Capsule, etc.",
                category: Category::Content,
                render: render_coming_soon,
            },
            // --- Controls ---
            CatalogItem {
                id: "controls", // ID matching manual dispatch
                title: "Basic Controls",
                description: "Toggle, Slider, Stepper, Picker.",
                category: Category::Controls,
                render: render_controls_stub,
            },
            CatalogItem {
                id: "button",
                title: "Button",
                description: "Push buttons and styles.",
                category: Category::Controls,
                render: render_coming_soon,
            },
            CatalogItem {
                id: "menu",
                title: "Menu",
                description: "Dropdown and context menus.",
                category: Category::Controls,
                render: render_coming_soon,
            },
            // --- Views ---
            CatalogItem {
                id: "sheets",
                title: "Sheets & Alerts",
                description: "Modal views and popovers.",
                category: Category::View,
                render: render_views,
            },
            CatalogItem {
                id: "popover",
                title: "Popover",
                description: "Contextual info bubbles.",
                category: Category::View,
                render: render_coming_soon,
            },
        ]
    }

    pub fn view(&self, context: &Context) -> Element<'static, CatalogMessage> {
        // Apply Catalog's selected theme to the context
        let mut local_context = context.clone();
        let mut tokens = peak_theme::ThemeTokens::new(self.theme, self.tone);
        tokens.scaling = 0.85; // Global shrink for desktop fit
        local_context.theme = tokens;
        let context = &local_context;

        let items = self.items.clone();
        let selected_id = self.selected_id;
        let inspector_open = self.inspector_open;

        // Capture state for renderers that need it
        let toggle_val = self.toggle_value;
        let slider_val = self.slider_value;
        let stepper_val = self.stepper_value;
        let text_val = self.text_value.clone();

        // Sidebar Content
        let sidebar_content = VStack::new()
            .padding(12.0)
            .spacing(12.0)
            .push(
                VStack::new()
                    .spacing(2.0)
                    .push(Text::new("PeakUI").headline())
                    .push(Text::new("Reference").secondary().caption1()),
            )
            .push(render_category(&items, selected_id, Category::Design))
            .push(render_category(&items, selected_id, Category::Content))
            .push(render_category(&items, selected_id, Category::Controls))
            .push(render_category(&items, selected_id, Category::Collections))
            .push(render_category(&items, selected_id, Category::Layouts))
            .push(render_category(&items, selected_id, Category::Navigation))
            .push(render_category(&items, selected_id, Category::View))
            .push(render_category(&items, selected_id, Category::System));

        // Detail Content
        let detail_view: Box<dyn View<CatalogMessage>> = if let Some(sid) = selected_id {
            // Manual dispatch for interactive items that need specific STATE
            let content: Box<dyn View<CatalogMessage>> = match sid {
                "controls" => render_controls(
                    context,
                    toggle_val,
                    slider_val,
                    stepper_val,
                    self.picker_value,
                ),
                "themes" => render_themes(context, self.theme, self.tone),
                "text_field" | "inputs" => Box::new(
                    VStack::new()
                        .spacing(12.0)
                        .padding(16.0)
                        .push(Text::new("Text Fields").title2())
                        .push(
                            TextField::new(
                                "Username",
                                text_val.clone(),
                                CatalogMessage::TextChanged,
                            )
                            .placeholder("Enter username")
                            .width(Length::Fixed(300.0)),
                        )
                        .push(
                            TextField::new("Password", "", |_| CatalogMessage::None)
                                .placeholder("Enter password")
                                .secure(true)
                                .width(Length::Fixed(300.0)),
                        ),
                ),
                // For other items, we use the function pointer from the item itself
                // This reduces the big match statement
                _ => {
                    if let Some(item) = items.iter().find(|i| i.id == sid) {
                        (item.render)(context)
                    } else {
                        Box::new(Text::new("Item not found").large_title().center())
                    }
                }
            };

            if let Some(item) = items.iter().find(|i| i.id == sid) {
                Box::new(
                    VStack::new()
                        .width(Length::Fill)
                        .push(
                            // Header
                            HStack::new()
                                .padding(16.0)
                                .align_y(iced::Alignment::Center)
                                .push(
                                    VStack::new()
                                        .spacing(4.0)
                                        .width(Length::Fill)
                                        .push(Text::new(item.title).title2())
                                        .push(Text::new(item.description).caption1().secondary()),
                                )
                                .push(
                                    Button::label("Inspector")
                                        .icon("layout_sidebar_right")
                                        .variant(Variant::Ghost)
                                        .on_press(CatalogMessage::ToggleInspector),
                                ),
                        )
                        .push(content),
                )
            } else {
                Box::new(Text::new("Item not found").large_title().center())
            }
        } else {
            Box::new(Text::new("Select an item").large_title().center())
        };

        let mut nav = NavigationSplitView::new(sidebar_content, detail_view);

        if inspector_open {
            nav = nav.inspector(
                VStack::new()
                    .padding(16.0)
                    .push(Text::new("Inspector").headline())
                    .push(Text::new("Controls coming soon...").secondary().caption1()),
            );
        }

        // Handle recursive navigation for slim views
        nav.force_sidebar_on_slim(selected_id.is_none())
            .on_back(CatalogMessage::GoBack)
            .view(context)
    }
}

// Helpers
fn render_category(
    items: &[CatalogItem],
    selected_id: Option<&str>,
    category: Category,
) -> impl View<CatalogMessage> {
    let category_items: Vec<_> = items
        .iter()
        .filter(|i| i.category == category)
        .map(|item| {
            let is_selected = Some(item.id) == selected_id;

            Button::label(item.title)
                .width(Length::Fill)
                .size(Size::Small)
                .intent(if is_selected {
                    Intent::Primary
                } else {
                    Intent::Neutral
                })
                .variant(if is_selected {
                    Variant::Solid
                } else {
                    Variant::Ghost
                })
                .on_press(CatalogMessage::ItemSelected(item.id))
        })
        .collect();

    if category_items.is_empty() {
        return VStack::new();
    }

    let mut list = VStack::new().spacing(4.0);
    for item in category_items {
        list = list.push(item);
    }

    VStack::new()
        .spacing(4.0)
        .push(Text::new(category.title()).caption2().secondary())
        .push(list)
}

// --- Render Functions ---

fn render_coming_soon(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(crate::atoms::Text::new("Coming Soon").title2())
            .push(
                crate::atoms::Text::new("This component is not yet implemented.")
                    .secondary()
                    .caption1(),
            ),
    )
}

fn render_typography(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(Text::new("Title 1").title1())
            .push(Text::new("Title 2").title2())
            .push(Text::new("Title 3").title3())
            .push(Text::new("Headline").headline())
            .push(Text::new("Body").body())
            .push(Text::new("Callout").callout())
            .push(Text::new("Subheadline").subheadline())
            .push(Text::new("Footnote").footnote())
            .push(Text::new("Caption 1").caption1())
            .push(Text::new("Caption 2").caption2()),
    )
}

fn render_colors(ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    let colors = ctx.theme.colors;
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(Text::new("Semantic Colors").title2())
            .push(
                ResponsiveGrid::new()
                    .spacing(12.0)
                    .push(color_swatch("Primary", colors.primary))
                    .push(color_swatch("Secondary", colors.secondary))
                    .push(color_swatch("Accent", colors.accent))
                    .push(color_swatch("Success", colors.success))
                    .push(color_swatch("Warning", colors.warning))
                    .push(color_swatch("Danger", colors.danger))
                    .push(color_swatch("Background", colors.background))
                    .push(color_swatch("Surface", colors.surface))
                    .push(color_swatch("Border", colors.border))
                    .push(color_swatch("Text Primary", colors.text_primary)),
            ),
    )
}

fn color_swatch(name: &str, color: iced::Color) -> impl View<CatalogMessage> {
    VStack::new()
        .spacing(8.0)
        .push(
            Rectangle::new(Length::Fill, Length::Fixed(80.0))
                .color(color)
                .corner_radius(12.0),
        )
        .push(Text::new(name).caption1().center())
}

fn render_icons(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(Text::new("Icons").title2())
            .push(
                HStack::new()
                    .spacing(16.0)
                    .push(Icon::new("settings").size(32.0))
                    .push(Icon::new("user").size(32.0))
                    .push(
                        Icon::new("home")
                            .size(32.0)
                            .color(iced::Color::from_rgb(0.0, 0.5, 1.0)),
                    ),
            ),
    )
}

fn render_controls_stub(_: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(crate::atoms::Space::new(Length::Shrink, Length::Shrink))
}

fn render_controls(
    _ctx: &Context,
    toggle_val: bool,
    slider_val: f32,
    stepper_val: i32,
    picker_val: usize,
) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(16.0)
            .padding(16.0)
            .push(Text::new("Toggles").title2())
            .push(Toggle::new(
                "Airplane Mode",
                toggle_val,
                CatalogMessage::ToggleChanged,
            ))
            .push(Text::new("Sliders").title2())
            .push(
                Slider::new(0.0..=100.0, slider_val, CatalogMessage::SliderChanged)
                    .width(Length::Fixed(200.0)),
            )
            .push(Text::new("Steppers").title2())
            .push(Stepper::new(
                "Quantity",
                stepper_val,
                CatalogMessage::StepperChanged,
            ))
            .push(Text::new("Segmented Picker").title2())
            .push(
                crate::segmented_picker::SegmentedPicker::<CatalogMessage, iced::Theme>::new(
                    vec![
                        ("Daily", CatalogMessage::PickerChanged(0)),
                        ("Weekly", CatalogMessage::PickerChanged(1)),
                        ("Monthly", CatalogMessage::PickerChanged(2)),
                        ("Yearly", CatalogMessage::PickerChanged(3)),
                    ],
                    picker_val,
                )
                .width(Length::Fixed(400.0)),
            ),
    )
}

fn render_inputs_placeholder(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    // This is actually stubbed in the Catalog::view manual match for now to pass state
    Box::new(Text::new("Inputs").body())
}

fn render_content(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(16.0)
            .padding(16.0)
            .push(Text::new("Layout & Separators").title2())
            .push(Text::new("Below is a divider").body())
            .push(Divider::new())
            .push(Text::new("Above is a divider").body())
            .push(Text::new("Multimedia").title2())
            .push(
                HStack::new()
                    .spacing(16.0)
                    .push(
                        VStack::new()
                            .spacing(8.0)
                            .push(Text::new("Image Asset").headline())
                            .push(
                                Image::new(std::path::PathBuf::from("assets/logo.png"))
                                    .width(Length::Fixed(100.0))
                                    .height(Length::Fixed(100.0))
                                    .corner_radius(12.0),
                            ),
                    )
                    .push(
                        VStack::new()
                            .spacing(8.0)
                            .push(Text::new("Placeholder").headline())
                            .push(
                                Rectangle::new(Length::Fixed(100.0), Length::Fixed(100.0))
                                    .color(iced::Color::from_rgb(0.8, 0.8, 0.8))
                                    .corner_radius(12.0),
                            ),
                    ),
            ),
    )
}

fn render_stacks(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(16.0)
            .padding(16.0)
            .push(Text::new("VStack & HStack").title2())
            .push(
                HStack::new()
                    .spacing(16.0)
                    .push(Card::new(Text::new("Item 1").body()).width(Length::Fixed(100.0)))
                    .push(Card::new(Text::new("Item 2").body()).width(Length::Fixed(100.0)))
                    .push(Card::new(Text::new("Item 3").body()).width(Length::Fixed(100.0))),
            )
            .push(Text::new("ZStack (Overlay)").title2())
            .push(
                ZStack::new()
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(200.0))
                    .push(
                        Rectangle::new(Length::Fill, Length::Fill)
                            .color(iced::Color::from_rgb(0.9, 0.9, 0.9))
                            .corner_radius(12.0),
                    )
                    .push(Text::new("Bottom Layer").secondary().center())
                    .push(
                        VStack::new()
                            .width(Length::Fill)
                            .height(Length::Fill)
                            .align_x(iced::Alignment::Center)
                            .push(
                                Card::new(Text::new("Top Layer").body())
                                    .width(Length::Fixed(120.0)),
                            ),
                    ),
            ),
    )
}

fn render_grid(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    let mut grid = ResponsiveGrid::new().spacing(12.0);

    for i in 1..=12 {
        grid = grid.push(
            Card::new(
                VStack::new()
                    .spacing(8.0)
                    .push(Text::new(format!("Item {}", i)).headline())
                    .push(
                        Text::new("Description text here that wraps.")
                            .caption1()
                            .secondary(),
                    ),
            )
            .width(Length::Fill),
        );
    }

    Box::new(
        VStack::new()
            .padding(16.0)
            .spacing(12.0)
            .push(Text::new("Responsive Grid").title2())
            .push(
                Text::new("Resize window to see columns adapt")
                    .caption1()
                    .secondary(),
            )
            .push(grid),
    )
}

fn render_scroll(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    // Generate enough content to force scrolling
    let mut content = VStack::new().spacing(16.0);
    for i in 1..=50 {
        content = content.push(
            HStack::new()
                .spacing(16.0)
                .push(Text::new(format!("{:02}", i)).secondary())
                .push(Text::new(format!("Scrollable Item Row {}", i)).body()),
        );
    }

    Box::new(
        VStack::new()
            .padding(16.0)
            .spacing(12.0)
            .height(Length::Fill) // Important for ScrollView to work within
            .push(Text::new("Vertical Scroll").title2())
            .push(
                ScrollView::new(content).height(Length::Fixed(400.0)), // Constrain height to force scroll
            ),
    )
}

fn render_navigation(_context: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(Text::new("Navigation").title1())
            .push(Text::new("You are currently using NavigationSplitView.").body())
            .push(
                Text::new("It supports:\n• 2-pane Desktop Layout\n• 3-pane Inspector Layout\n• Responsive Mobile Stack Navigation\n• Mobile Inspector Sheets")
                    .secondary()
                    .caption1()
            )
    )
}

fn render_views(_context: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(Text::new("System Views").title1())
            .push(Text::new("Alerts").title2())
            .push(
                Button::label("Show Alert")
                    .intent(Intent::Primary)
                    .on_press(CatalogMessage::ToggleAlert(true)),
            )
            .push(Divider::new())
            .push(Text::new("Sheets & Popovers").title2())
            .push(Text::new("Coming soon...").secondary()),
    )
}

fn render_themes(
    _ctx: &Context,
    current_theme: peak_theme::PeakTheme,
    current_tone: peak_theme::ThemeTone,
) -> Box<dyn View<CatalogMessage>> {
    let mut grid = ResponsiveGrid::new().spacing(12.0);

    for theme in peak_theme::PeakTheme::all() {
        let is_selected = *theme == current_theme;
        let colors = theme.colors(current_tone);

        grid = grid.push(
            Card::new(
                VStack::new()
                    .spacing(8.0)
                    .push(
                        Rectangle::new(Length::Fill, Length::Fixed(30.0))
                            .color(colors.primary)
                            .corner_radius(6.0),
                    )
                    .push(
                        HStack::new()
                            .spacing(4.0)
                            .push(
                                Rectangle::new(Length::Fixed(16.0), Length::Fixed(16.0))
                                    .color(colors.background)
                                    .corner_radius(4.0),
                            )
                            .push(
                                Rectangle::new(Length::Fixed(16.0), Length::Fixed(16.0))
                                    .color(colors.surface)
                                    .corner_radius(4.0),
                            )
                            .push(
                                Rectangle::new(Length::Fixed(16.0), Length::Fixed(16.0))
                                    .color(colors.secondary)
                                    .corner_radius(4.0),
                            ),
                    )
                    .push(
                        Button::label(theme.display_name())
                            .width(Length::Fill)
                            .intent(if is_selected {
                                Intent::Primary
                            } else {
                                Intent::Neutral
                            })
                            .variant(if is_selected {
                                Variant::Solid
                            } else {
                                Variant::Outline
                            })
                            .on_press(CatalogMessage::ThemeChanged(*theme)),
                    ),
            )
            .width(Length::Fill),
        );
    }

    Box::new(
        VStack::new()
            .spacing(12.0)
            .padding(16.0)
            .push(
                HStack::new()
                    .align_y(iced::Alignment::Center)
                    .push(Text::new("Appearance").title2())
                    .push(crate::atoms::Space::new(Length::Fill, Length::Shrink))
                    .push(
                        crate::segmented_picker::SegmentedPicker::<CatalogMessage, iced::Theme>::new(
                            vec![
                                (
                                    "Light",
                                    CatalogMessage::ToneChanged(peak_theme::ThemeTone::Light),
                                ),
                                (
                                    "Dark",
                                    CatalogMessage::ToneChanged(peak_theme::ThemeTone::Dark),
                                ),
                            ],
                            match current_tone {
                                peak_theme::ThemeTone::Light => 0,
                                peak_theme::ThemeTone::Dark => 1,
                            },
                        )
                        .width(Length::Fixed(200.0)),
                    ),
            )
            .push(Divider::new())
            .push(Text::new("Peak Themes").title2())
            .push(Text::new("Choose a theme to transform the entire OS interface.").secondary())
            .push(grid),
    )
}
