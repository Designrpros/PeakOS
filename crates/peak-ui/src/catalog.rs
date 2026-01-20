use crate::atoms::{Divider, Icon, Image, Rectangle};
use crate::core::{Context, View};
use crate::prelude::*;
use iced::{Length, Task};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    Foundations,
    Components,
    Layouts,
    Navigation,
    View,
}

impl Category {
    fn title(&self) -> &'static str {
        match self {
            Self::Foundations => "Foundations",
            Self::Components => "Components",
            Self::Layouts => "Layouts",
            Self::Navigation => "Navigation",
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
            _ => Task::none(),
        }
    }

    fn build_items() -> Vec<CatalogItem> {
        vec![
            CatalogItem {
                id: "typography",
                title: "Typography",
                description: "The type scale used throughout PeakOS.",
                category: Category::Foundations,
                render: render_typography,
            },
            CatalogItem {
                id: "colors",
                title: "Colors",
                description: "Semantic color palette.",
                category: Category::Foundations,
                render: render_colors,
            },
            CatalogItem {
                id: "controls",
                title: "Basic Controls",
                description: "Interactive elements like Toggles, Sliders, Steppers, and Pickers.",
                category: Category::Components,
                render: render_controls_stub,
            },
            CatalogItem {
                id: "inputs",
                title: "Text Inputs",
                description: "Fields for user text entry.",
                category: Category::Components,
                render: render_inputs_placeholder,
            },
            CatalogItem {
                id: "navigation",
                title: "Navigation",
                description: "Navigation Split View, Sidebar, and stacks.",
                category: Category::Navigation,
                render: render_navigation,
            },
            CatalogItem {
                id: "view",
                title: "Views & Alerts",
                description: "Sheets, Popovers, and Alerts.",
                category: Category::View,
                render: render_views,
            },
            CatalogItem {
                id: "stacks",
                title: "Stacks",
                description: "VStack, HStack, and ZStack layouts.",
                category: Category::Layouts,
                render: render_stacks,
            },
            CatalogItem {
                id: "grid",
                title: "Responsive Grid",
                description: "Grid that adapts columns to screen width.",
                category: Category::Layouts,
                render: render_grid,
            },
            CatalogItem {
                id: "scroll",
                title: "Scroll View",
                description: "Scrollable container for overflowing content.",
                category: Category::Layouts, // Or Collections, but Layouts fits for now
                render: render_scroll,
            },
        ]
    }
}

// Implement View for Catalog directly
impl View<CatalogMessage> for Catalog {
    fn view(
        &self,
        context: &Context,
    ) -> iced::Element<'static, CatalogMessage, iced::Theme, iced::Renderer> {
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
            .padding(16.0)
            .spacing(24.0)
            .push(
                VStack::new()
                    .spacing(4.0)
                    .push(Text::new("PeakUI").large_title())
                    .push(Text::new("Reference").secondary()),
            )
            .push(render_category(&items, selected_id, Category::Foundations))
            .push(render_category(&items, selected_id, Category::Components))
            .push(render_category(&items, selected_id, Category::Layouts));

        // Detail Content
        let detail_view: Box<dyn View<CatalogMessage>> = if let Some(sid) = selected_id {
            // Manual dispatch for interactive items
            let content: Box<dyn View<CatalogMessage>> = match sid {
                "typography" => render_typography(context),
                "colors" => render_colors(context),
                "controls" => render_controls(
                    context,
                    toggle_val,
                    slider_val,
                    stepper_val,
                    self.picker_value,
                ),
                "inputs" => Box::new(
                    VStack::new()
                        .spacing(24.0)
                        .padding(32.0)
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
                "content" => render_content(context),
                "stacks" => render_stacks(context),
                "grid" => render_grid(context),
                "scroll" => render_scroll(context),
                "navigation" => render_navigation(context),
                "view" => render_views(context),
                _ => {
                    // Fallback for static items
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
                                .padding(32.0)
                                .align_y(iced::Alignment::Center)
                                .push(
                                    VStack::new()
                                        .spacing(8.0)
                                        .width(Length::Fill)
                                        .push(Text::new(item.title).large_title())
                                        .push(Text::new(item.description).body().secondary()),
                                )
                                .push(
                                    Button::new("Inspector")
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

            Button::new(item.title)
                .width(Length::Fill)
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
        .spacing(8.0)
        .push(Text::new(category.title()).caption1().secondary())
        .push(list)
}

// Static Renderers
fn render_typography(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(24.0)
            .padding(32.0)
            .push(Text::new("Large Title").large_title())
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
            .spacing(16.0)
            .padding(32.0)
            .push(Text::new("Semantic Colors").title2())
            .push(
                ResponsiveGrid::new()
                    .spacing(12.0)
                    .push(color_swatch("Primary", colors.primary))
                    .push(color_swatch("Background", colors.background))
                    .push(color_swatch("Surface", colors.surface))
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

// Placeholder renderers for items that need state (handled in Catalog::view view dispatch for now)
fn render_controls_placeholder(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(Text::new("Controls").body())
}
fn render_inputs_placeholder(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(Text::new("Inputs").body())
}
fn render_content_placeholder(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(Text::new("Content").body())
}

fn render_content(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(24.0)
            .padding(32.0)
            .push(Text::new("Icons").title2())
            .push(
                HStack::new()
                    .spacing(16.0)
                    .push(Icon::new("settings").size(32.0))
                    .push(Icon::new("user").size(32.0))
                    .push(
                        Icon::new("home")
                            .size(32.0)
                            .color(iced::Color::from_rgb(1.0, 0.0, 0.5)),
                    ),
            )
            .push(Divider::new())
            .push(Text::new("Images").title2())
            .push(
                Image::new(std::path::PathBuf::from("assets/images/placeholder.png"))
                    .width(Length::Fixed(200.0))
                    .height(Length::Fixed(150.0))
                    .corner_radius(12.0),
            )
            .push(
                Text::new("Note: Image requires valid asset path")
                    .caption1()
                    .secondary(),
            ),
    )
}

fn render_stacks(_ctx: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(32.0)
            .padding(32.0)
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
    let mut grid = ResponsiveGrid::new().spacing(20.0);

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
            .padding(32.0)
            .spacing(24.0)
            .push(Text::new("Resize window to see columns adapt").secondary())
            .push(grid),
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
            .spacing(24.0)
            .padding(32.0)
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

fn render_navigation(_context: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(16.0)
            .padding(32.0)
            .push(Text::new("Navigation").large_title())
            .push(Text::new("You are currently using NavigationSplitView.").body())
            .push(
                Text::new("It supports:\n• 2-pane Desktop Layout\n• 3-pane Inspector Layout\n• Responsive Mobile Stack Navigation\n• Mobile Inspector Sheets")
                    .secondary()
                    .callout()
            )
    )
}

fn render_views(_context: &Context) -> Box<dyn View<CatalogMessage>> {
    Box::new(
        VStack::new()
            .spacing(16.0)
            .padding(32.0)
            .push(Text::new("System Views").large_title())
            .push(Text::new("Alerts").title2())
            .push(
                Button::new("Show Alert")
                    .intent(Intent::Primary)
                    .on_press(CatalogMessage::ToggleAlert(true)),
            )
            .push(Divider::new())
            .push(Text::new("Sheets & Popovers").title2())
            .push(Text::new("Coming soon...").secondary()),
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
            .padding(32.0)
            .spacing(24.0)
            .height(Length::Fill) // Important for ScrollView to work within
            .push(Text::new("Vertical Scroll").title2())
            .push(
                ScrollView::new(content).height(Length::Fixed(400.0)), // Constrain height to force scroll
            ),
    )
}
