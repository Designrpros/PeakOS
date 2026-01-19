use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Vector};
use peak_core::registry::{AppId, AppInfo};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SwitcherMessage {
    Next,
    Prev,
    Select(AppId),
}

pub struct AppSwitcher {
    pub selected_index: usize,
    pub apps: Vec<AppInfo>,
}

impl Default for AppSwitcher {
    fn default() -> Self {
        Self::new()
    }
}

impl AppSwitcher {
    pub fn new() -> Self {
        // For now, we use a subset of registry apps as a demonstration
        // In a future update, this will be populated by "Running" apps
        let apps = AppInfo::all().into_iter().take(5).collect();

        Self {
            selected_index: 0,
            apps,
        }
    }

    pub fn next(&mut self) {
        if !self.apps.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.apps.len();
        }
    }

    pub fn prev(&mut self) {
        if !self.apps.is_empty() {
            self.selected_index = (self.selected_index + self.apps.len() - 1) % self.apps.len();
        }
    }

    pub fn view(&self, tokens: peak_theme::ThemeTokens) -> Element<'_, SwitcherMessage> {
        let text_color = tokens.text;
        let card_bg = tokens.glass_bg;

        let hex_color = format!(
            "#{:02x}{:02x}{:02x}",
            (tokens.text.r * 255.0) as u8,
            (tokens.text.g * 255.0) as u8,
            (tokens.text.b * 255.0) as u8
        );

        let content = row(self.apps.iter().enumerate().map(|(i, app)| {
            let is_selected = i == self.selected_index;

            let bg = if is_selected {
                let mut c = tokens.text;
                c.a = 0.1;
                c
            } else {
                Color::TRANSPARENT
            };

            let border_color = if is_selected {
                tokens.glass_border
            } else {
                Color::TRANSPARENT
            };

            let icon = peak_core::icons::get_app_icon(app.id, &hex_color);

            container(
                column![
                    container(iced::widget::svg(icon).width(40).height(40))
                        .width(60)
                        .height(60)
                        .center_x(Length::Fill)
                        .center_y(Length::Fill)
                        .style(|_| container::Style {
                            background: Some(Background::Color(Color::from_rgba(
                                1.0, 1.0, 1.0, 0.03
                            ))),
                            border: Border {
                                radius: 14.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    text(app.name).size(11).color(text_color)
                ]
                .spacing(12)
                .align_x(Alignment::Center),
            )
            .padding(12)
            .width(100)
            .height(115)
            .style(move |_| container::Style {
                background: Some(Background::Color(bg)),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: tokens.radius.into(),
                },
                ..Default::default()
            })
            .into()
        }))
        .spacing(15)
        .align_y(Alignment::Center);

        container(content)
            .padding(15)
            .style(move |_| container::Style {
                background: Some(Background::Color(card_bg)),
                border: Border {
                    color: tokens.glass_border,
                    width: 1.0,
                    radius: (tokens.radius * 2.0).into(),
                },
                shadow: Shadow {
                    color: tokens.shadow_color,
                    offset: Vector::new(0.0, 20.0),
                    blur_radius: 50.0,
                },
                ..Default::default()
            })
            .into()
    }
}
