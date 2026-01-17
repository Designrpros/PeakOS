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
    pub _phantom: std::marker::PhantomData<&'a ()>,
}

impl<'a, Message> Sidebar<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            items: Vec::new(),
            search_query: None,
            on_search: None,
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
        let mut content = column![].spacing(4);

        if let (Some(query), Some(on_search)) = (self.search_query, self.on_search) {
            content = content.push(
                container(
                    row![
                        iced::widget::svg(icons::get_ui_icon("search", "#999999"))
                            .width(16)
                            .height(16),
                        iced::widget::text_input("Search", &query)
                            .on_input(on_search)
                            .size(13)
                            .padding(0)
                            .style(|_theme, _status| iced::widget::text_input::Style {
                                background: Color::TRANSPARENT.into(),
                                border: iced::Border::default(),
                                icon: Color::TRANSPARENT,
                                placeholder: Color::from_rgb(0.6, 0.6, 0.6),
                                value: Color::from_rgb(0.2, 0.2, 0.2),
                                selection: Color::from_rgba(0.0, 0.5, 1.0, 0.3),
                            }),
                    ]
                    .spacing(8)
                    .align_y(Alignment::Center)
                    .padding(8),
                )
                .style(|_| container::Style {
                    background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.05).into()),
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
                Color::from_rgb8(60, 60, 60)
            };

            let icon_color_hex = if item.is_selected {
                "#FFFFFF"
            } else {
                "#3C3C3C"
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
                        Color::from_rgba(0.0, 0.0, 0.0, 0.05)
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
            .style(|_| container::Style {
                background: Some(Color::from_rgba(0.95, 0.95, 0.95, 0.5).into()),
                ..Default::default()
            })
            .into()
    }
}

pub struct NavigationSplitView<'a, Message> {
    pub sidebar: Element<'a, Message>,
    pub content: Element<'a, Message>,
}

impl<'a, Message> NavigationSplitView<'a, Message>
where
    Message: 'a + Clone,
{
    pub fn new(sidebar: Sidebar<'a, Message>, content: Element<'a, Message>) -> Self {
        Self {
            sidebar: sidebar.view(),
            content,
        }
    }

    pub fn view(self) -> Element<'a, Message> {
        row![
            self.sidebar,
            container(self.content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(24)
                .style(|_| container::Style {
                    background: Some(Color::WHITE.into()),
                    ..Default::default()
                })
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
