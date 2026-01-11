use crate::registry::{AppId, AppInfo};
use iced::widget::{column, container, row, text};
use iced::{Alignment, Background, Border, Color, Element, Length, Shadow, Vector};

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

    pub fn view(&self) -> Element<'_, SwitcherMessage> {
        let content = row(self.apps.iter().enumerate().map(|(i, app)| {
            let is_selected = i == self.selected_index;

            let bg = if is_selected {
                Color::from_rgba(1.0, 1.0, 1.0, 0.15)
            } else {
                Color::TRANSPARENT
            };

            let border_color = if is_selected {
                Color::from_rgba(1.0, 1.0, 1.0, 0.3)
            } else {
                Color::TRANSPARENT
            };

            container(
                column![
                    // Placeholder for App Icon
                    container(
                        text(app.name.chars().next().unwrap_or('?'))
                            .size(32)
                            .color(Color::WHITE)
                    )
                    .width(48)
                    .height(48)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
                        border: Border {
                            radius: 10.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    text(app.name)
                        .size(10)
                        .color(Color::from_rgba(1.0, 1.0, 1.0, 0.8))
                ]
                .spacing(10)
                .align_x(Alignment::Center),
            )
            .padding(15)
            .width(100)
            .height(110)
            .style(move |_| container::Style {
                background: Some(Background::Color(bg)),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            })
            .into()
        }))
        .spacing(10)
        .align_y(Alignment::Center);

        container(content)
            .padding(10)
            .style(move |_| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.1, 0.1, 0.15, 0.85))),
                border: Border {
                    color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                    width: 1.0,
                    radius: 20.0.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    offset: Vector::new(0.0, 15.0),
                    blur_radius: 40.0,
                },
                ..Default::default()
            })
            .into()
    }
}
