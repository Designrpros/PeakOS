use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length, Padding};
use peak_core::icons;

pub struct SidebarItem<Message> {
    pub label: String,
    pub icon_name: String,
    pub on_press: Message,
    pub is_selected: bool,
}

pub struct Sidebar<'a, Message> {
    pub title: String,
    pub items: Vec<SidebarItem<Message>>,
    pub search_query: Option<String>,
    pub on_search: Option<Box<dyn Fn(String) -> Message + 'a>>,
    pub tokens: peak_theme::ThemeTokens,
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, Message> Sidebar<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(title: impl Into<String>, tokens: peak_theme::ThemeTokens) -> Self {
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
        on_change: impl Fn(String) -> Message + 'a,
    ) -> Self {
        self.search_query = Some(query.into());
        self.on_search = Some(Box::new(on_change));
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let tokens = self.tokens;
        let hex_text = format!(
            "#{:02X}{:02X}{:02X}",
            (tokens.text.r * 255.0) as u8,
            (tokens.text.g * 255.0) as u8,
            (tokens.text.b * 255.0) as u8
        );
        let mut content = column![].spacing(4);

        if let (Some(query), Some(on_search)) = (self.search_query, self.on_search) {
            content = content.push(
                container(
                    row![
                        iced::widget::svg(icons::get_ui_icon("search", &hex_text))
                            .width(16)
                            .height(16),
                        iced::widget::text_input("Search", &query)
                            .on_input(on_search)
                            .size(13)
                            .padding(0)
                            .style(move |_theme, _status| iced::widget::text_input::Style {
                                background: Color::TRANSPARENT.into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT,
                                placeholder: iced::Color {
                                    a: 0.5,
                                    ..tokens.text
                                },
                                value: tokens.text,
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            }),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .padding(8),
                )
                .style(move |_| container::Style {
                    background: Some({
                        let mut c = tokens.text;
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

        for item in self.items {
            let bg = if item.is_selected {
                Color::from_rgb8(0, 122, 255) // Classic Apple Blue
            } else {
                Color::TRANSPARENT
            };

            let text_color = if item.is_selected {
                Color::WHITE
            } else {
                tokens.text
            };

            let icon_color_hex = if item.is_selected {
                "#FFFFFF"
            } else {
                &hex_text
            };

            let icon_handle = icons::get_ui_icon(&item.icon_name, icon_color_hex);

            content = content.push(
                iced::widget::button(
                    row![
                        iced::widget::svg(icon_handle).width(16).height(16),
                        text(item.label).size(14),
                    ]
                    .spacing(12)
                    .align_y(Alignment::Center),
                )
                .on_press(item.on_press)
                .padding(Padding::from([8, 12]))
                .width(Length::Fill)
                .style(move |_theme, status| {
                    let final_bg = if item.is_selected {
                        bg
                    } else if status == iced::widget::button::Status::Hovered {
                        let mut c = tokens.text;
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
                background: Some(tokens.card_bg.into()),
                ..Default::default()
            })
            .into()
    }
}

pub struct NavigationSplitView<'a, Message> {
    pub sidebar: Element<'a, Message>,
    pub content: Element<'a, Message>,
    pub is_light: bool,
}

impl<'a, Message> NavigationSplitView<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(sidebar: Sidebar<'a, Message>, content: Element<'a, Message>) -> Self {
        Self {
            sidebar: sidebar.view(),
            content,
            is_light: true,
        }
    }

    pub fn theme(mut self, is_light: bool) -> Self {
        self.is_light = is_light;
        self
    }

    pub fn view(self) -> Element<'a, Message> {
        let is_light = self.is_light;
        let content_bg = if is_light {
            Color::WHITE
        } else {
            Color::from_rgb8(28, 28, 30) // Dark mode background
        };

        let row_content = row![
            self.sidebar,
            container(self.content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(24)
                .style(move |_| container::Style {
                    background: Some(content_bg.into()),
                    ..Default::default()
                })
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(row_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(content_bg.into()),
                ..Default::default()
            })
            .into()
    }
}
