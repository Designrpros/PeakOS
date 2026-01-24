use crate::core::{Backend, Context, IcedBackend, TermBackend, View};
use crate::toolbar::ToolbarItem;
use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Padding};
use peak_core::icons;
use std::sync::Arc;

/// Configuration for search behavior in a page
pub struct SearchConfig<Message: 'static> {
    /// The current search query
    pub query: String,
    /// Placeholder text for the search field
    pub placeholder: String,
    /// Callback triggered when the search query changes
    pub on_change: Arc<dyn Fn(String) -> Message + Send + Sync + 'static>,
}

/// Result of a page rendering, including contextual UI elements
pub struct PageResult<Message: 'static, B: Backend = IcedBackend> {
    /// The main content of the page
    pub view: Box<dyn View<Message, B>>,
    /// Contextual toolbar items contributed by the page
    pub toolbar_items: Vec<ToolbarItem<Message>>,
    /// Optional contextual inspector view provided by the page
    pub inspector: Option<Box<dyn View<Message, B>>>,
    /// Optional search configuration
    pub search_config: Option<SearchConfig<Message>>,
    /// Optional message to toggle the sidebar (used for burger menu on mobile)
    pub sidebar_toggle: Option<Message>,
}

impl<Message: 'static, B: Backend> PageResult<Message, B> {
    pub fn new(view: impl View<Message, B> + 'static) -> Self {
        Self {
            view: Box::new(view),
            toolbar_items: Vec::new(),
            inspector: None,
            search_config: None,
            sidebar_toggle: None,
        }
    }

    pub fn toolbar(mut self, item: ToolbarItem<Message>) -> Self {
        self.toolbar_items.push(item);
        self
    }

    pub fn inspector(mut self, inspector: impl View<Message, B> + 'static) -> Self {
        self.inspector = Some(Box::new(inspector));
        self
    }

    pub fn searchable(
        mut self,
        query: impl Into<String>,
        placeholder: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.search_config = Some(SearchConfig {
            query: query.into(),
            placeholder: placeholder.into(),
            on_change: Arc::new(on_change),
        });
        self
    }

    pub fn sidebar_toggle(mut self, msg: Message) -> Self {
        self.sidebar_toggle = Some(msg);
        self
    }
}

/// A Page is a top-level view that can contribute contextual UI elements like toolbars and inspectors.
pub trait Page<Message: 'static, B: Backend = IcedBackend>: View<Message, B> {
    /// Returns the contextual toolbar items for this page.
    fn page_toolbar_items(&self) -> Vec<ToolbarItem<Message>> {
        Vec::new()
    }

    /// Returns the optional contextual inspector for this page.
    fn page_inspector(&self) -> Option<Box<dyn View<Message, B>>> {
        None
    }

    /// Returns the optional search configuration for this page.
    fn page_search_config(&self) -> Option<SearchConfig<Message>> {
        None
    }

    /// Returns the optional sidebar toggle message for this page.
    fn page_sidebar_toggle(&self) -> Option<Message> {
        None
    }

    /// Wraps this page into a PageResult.
    fn into_page_result(self) -> PageResult<Message, B>
    where
        Self: Sized + 'static,
    {
        let toolbar_items = self.page_toolbar_items();
        let inspector = self.page_inspector();
        let search_config = self.page_search_config();
        let sidebar_toggle = self.page_sidebar_toggle();
        PageResult {
            view: Box::new(self),
            toolbar_items,
            inspector,
            search_config,
            sidebar_toggle,
        }
    }
}

/// Extensions to the View trait for ergonomic contextual UI building.
pub trait ViewExt<Message: 'static, B: Backend = IcedBackend>: View<Message, B> + Sized {
    /// Adds a contextual toolbar item to this view, turning it into a PageResult.
    fn toolbar(self, item: ToolbarItem<Message>) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).toolbar(item)
    }

    /// Adds a contextual inspector to this view, turning it into a PageResult.
    fn inspector(self, inspector: impl View<Message, B> + 'static) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).inspector(inspector)
    }

    /// Adds a search configuration to this view, turning it into a PageResult.
    fn searchable(
        self,
        query: impl Into<String>,
        placeholder: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).searchable(query, placeholder, on_change)
    }

    /// Adds a sidebar toggle message to this view, turning it into a PageResult.
    fn sidebar_toggle(self, msg: Message) -> PageResult<Message, B>
    where
        Self: 'static,
    {
        PageResult::new(self).sidebar_toggle(msg)
    }
}

impl<Message: 'static, B: Backend, T: View<Message, B>> ViewExt<Message, B> for T {}

pub struct SidebarItem<Message> {
    pub label: String,
    pub icon_name: String,
    pub on_press: Message,
    pub is_selected: bool,
}

pub struct Sidebar<Message: 'static, B: Backend = IcedBackend> {
    pub title: String,
    pub items: Vec<SidebarItem<Message>>,
    pub search_query: Option<String>,
    pub on_search: Option<Arc<dyn Fn(String) -> Message + Send + Sync + 'static>>,
    pub tokens: peak_theme::ThemeTokens,
    _phantom: std::marker::PhantomData<B>,
}

impl<Message: Clone + 'static> Sidebar<Message, IcedBackend> {
    pub fn new(title: impl Into<String>, tokens: peak_theme::ThemeTokens) -> Self {
        Self::new_generic(title, tokens)
    }
}

impl<Message: Clone + 'static> Sidebar<Message, TermBackend> {
    pub fn new_tui(title: impl Into<String>, tokens: peak_theme::ThemeTokens) -> Self {
        Self::new_generic(title, tokens)
    }
}

impl<Message: 'static, B: Backend> Sidebar<Message, B>
where
    Message: Clone + 'static,
{
    pub fn new_generic(title: impl Into<String>, tokens: peak_theme::ThemeTokens) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            search_query: None,
            on_search: None,
            tokens,
            _phantom: std::marker::PhantomData,
        }
    }

    pub fn item(
        mut self,
        label: impl Into<String>,
        icon_name: impl Into<String>,
        on_press: Message,
        is_selected: bool,
    ) -> Self {
        self.items.push(SidebarItem {
            label: label.into(),
            icon_name: icon_name.into(),
            on_press,
            is_selected,
        });
        self
    }

    pub fn with_search(
        mut self,
        query: impl Into<String>,
        on_change: impl Fn(String) -> Message + Send + Sync + 'static,
    ) -> Self {
        self.search_query = Some(query.into());
        self.on_search = Some(Arc::new(on_change));
        self
    }
}

impl<Message: Clone + 'static> View<Message, IcedBackend> for Sidebar<Message, IcedBackend> {
    fn view(&self, context: &Context) -> Element<'static, Message, iced::Theme, iced::Renderer> {
        let tokens = context.theme;
        let hex_text = format!(
            "#{:02X}{:02X}{:02X}",
            (tokens.colors.text_primary.r * 255.0) as u8,
            (tokens.colors.text_primary.g * 255.0) as u8,
            (tokens.colors.text_primary.b * 255.0) as u8
        );
        let mut content = column![].spacing(4);

        if let (Some(query), Some(on_search)) = (&self.search_query, &self.on_search) {
            content = content.push(
                container(
                    row![
                        iced::widget::svg(icons::get_ui_icon("search", &hex_text))
                            .width(16)
                            .height(16),
                        iced::widget::text_input("Search", query)
                            .on_input({
                                let on_search = on_search.clone();
                                move |s| on_search(s)
                            })
                            .size(13)
                            .padding(0)
                            .style(move |_theme, _status| iced::widget::text_input::Style {
                                background: Color::TRANSPARENT.into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT,
                                placeholder: iced::Color {
                                    a: 0.5,
                                    ..tokens.colors.text_primary
                                },
                                value: tokens.colors.text_primary,
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            }),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .padding(8),
                )
                .style({
                    let bg_color = {
                        let mut c = tokens.colors.text_primary;
                        c.a = 0.05;
                        c
                    };
                    let radius_val = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        8.0
                    };
                    move |_| container::Style {
                        background: Some(bg_color.into()),
                        border: iced::Border {
                            radius: radius_val.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }),
            );
            content = content.push(iced::widget::vertical_space().height(16));
        }
        for item in &self.items {
            let bg = if item.is_selected {
                Color::from_rgb8(0, 122, 255)
            } else {
                Color::TRANSPARENT
            };

            let text_color = if item.is_selected {
                Color::WHITE
            } else {
                tokens.colors.text_primary
            };

            let icon_color_hex = if item.is_selected {
                "#FFFFFF"
            } else {
                &hex_text
            };

            let is_selected = item.is_selected;
            let on_press = item.on_press.clone();
            let label = item.label.clone();
            let icon_handle = icons::get_ui_icon(&item.icon_name, icon_color_hex);

            content = content.push(
                iced::widget::button(
                    row![
                        iced::widget::svg(icon_handle).width(16).height(16),
                        text(label).size(14),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center),
                )
                .on_press(on_press)
                .padding(Padding::from([8, 12]))
                .width(Length::Fill)
                .style({
                    let bg_selected = bg;
                    let text_color = text_color;
                    let text_primary = tokens.colors.text_primary;
                    let radius_val = if cfg!(target_arch = "wasm32") {
                        0.0
                    } else {
                        8.0
                    };
                    move |_theme, status| {
                        let final_bg = if is_selected {
                            bg_selected
                        } else if status == iced::widget::button::Status::Hovered {
                            let mut c = text_primary;
                            c.a = 0.05;
                            c
                        } else {
                            Color::TRANSPARENT
                        };

                        iced::widget::button::Style {
                            background: Some(final_bg.into()),
                            text_color,
                            border: iced::Border {
                                radius: radius_val.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    }
                }),
            );
        }

        container(scrollable(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(12)
            .style(move |_| container::Style {
                background: Some(tokens.colors.surface_variant.into()),
                ..Default::default()
            })
            .into()
    }
}

impl<Message: Clone + 'static> View<Message, TermBackend> for Sidebar<Message, TermBackend> {
    fn view(&self, _context: &Context) -> String {
        let mut out = format!("=== {} ===\n", self.title);
        for item in &self.items {
            let prefix = if item.is_selected { "> " } else { "  " };
            out.push_str(&format!("{}{} ({})\n", prefix, item.label, item.icon_name));
        }
        out
    }
}
