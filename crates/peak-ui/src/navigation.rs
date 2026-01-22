use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Padding};
use peak_core::icons;
use std::sync::Arc;

pub struct SidebarItem<Message> {
    pub label: String,
    pub icon_name: String,
    pub on_press: Message,
    pub is_selected: bool,
}

pub struct Sidebar<Message> {
    pub title: String,
    pub items: Vec<SidebarItem<Message>>,
    pub search_query: Option<String>,
    pub on_search: Option<Arc<dyn Fn(String) -> Message + Send + Sync + 'static>>,
    pub tokens: peak_theme::ThemeTokens,
}

impl<Message> Sidebar<Message>
where
    Message: Clone + 'static,
{
    pub fn new(title: impl Into<String>, tokens: peak_theme::ThemeTokens) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            search_query: None,
            on_search: None,
            tokens,
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

impl<Message: Clone + 'static> crate::core::View<Message> for Sidebar<Message> {
    fn view(
        &self,
        context: &crate::core::Context,
    ) -> Element<'static, Message, iced::Theme, iced::Renderer> {
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
                .style(move |_| container::Style {
                    background: Some({
                        let mut c = tokens.colors.text_primary;
                        c.a = 0.05;
                        c.into()
                    }),
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
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
                .style(move |_theme, status| {
                    let final_bg = if is_selected {
                        bg
                    } else if status == iced::widget::button::Status::Hovered {
                        let mut c = tokens.colors.text_primary;
                        c.a = 0.05;
                        c
                    } else {
                        Color::TRANSPARENT
                    };

                    iced::widget::button::Style {
                        background: Some(final_bg.into()),
                        text_color,
                        border: iced::Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                }),
            );
        }

        container(scrollable(content))
            .width(Length::Fixed(220.0))
            .height(Length::Fill)
            .padding(12)
            .style(move |_| container::Style {
                background: Some(tokens.colors.surface_variant.into()),
                ..Default::default()
            })
            .into()
    }
}
