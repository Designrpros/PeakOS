use iced::widget::{button, column, container, svg, text};
use iced::{Element, Length, Point};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DesktopMessage {
    Select(PathBuf, bool), // path, is_multi (shift held)
    Open(PathBuf),
    StartSelection(Point, iced::keyboard::Modifiers),
    UpdateSelection(Point),
    EndSelection,
    #[allow(dead_code)]
    Drag(PathBuf, Point),
}

pub struct Desktop {
    pub items: Vec<DesktopItem>,
    pub selected: Vec<PathBuf>,
    pub last_click: Option<(PathBuf, std::time::Instant)>,
    pub selection_rect: Option<iced::Rectangle>,
    pub drag_start: Option<Point>,
}

#[derive(Debug, Clone)]
pub struct DesktopItem {
    pub path: PathBuf,
    pub name: String,
    pub position: Point,
}

impl Desktop {
    pub fn new() -> Self {
        let mut desktop = Self {
            items: Vec::new(),
            selected: Vec::new(),
            last_click: None,
            selection_rect: None,
            drag_start: None,
        };
        desktop.refresh();
        desktop
    }

    pub fn refresh(&mut self) {
        let desktop_path = dirs::home_dir()
            .map(|h| h.join("Desktop"))
            .unwrap_or_else(|| PathBuf::from("/Users/vegarberentsen/Desktop"));

        self.items.clear();

        if let Ok(entries) = std::fs::read_dir(&desktop_path) {
            let mut x = 20.0;
            let mut y = 50.0; // Start below menubar (32px)
            let spacing = 100.0;

            for entry in entries.flatten() {
                let path = entry.path();
                let name = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                self.items.push(DesktopItem {
                    path,
                    name,
                    position: Point::new(x, y),
                });

                y += spacing;
                if y > 700.0 {
                    y = 50.0;
                    x += spacing;
                }
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, DesktopMessage> {
        let mut stack = iced::widget::Stack::new();

        // 0. The background is handled in app.rs.
        // We avoid putting a button here to ensure mouse release is captured globally.

        for item in &self.items {
            let is_selected = self.selected.contains(&item.path);
            let path = item.path.clone();
            let name = item.name.clone();
            let pos = item.position;

            let icon = view_icon(&path);
            let label = text(name)
                .size(10)
                .color(iced::Color::WHITE)
                .align_x(iced::alignment::Horizontal::Center);

            let column = column![icon, label]
                .spacing(5)
                .align_x(iced::Alignment::Center);

            let button = button(column)
                .on_press(DesktopMessage::Select(path, false)) // Default single select
                .style(move |_, status| button::Style {
                    background: if is_selected {
                        Some(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.2).into())
                    } else if status == iced::widget::button::Status::Hovered {
                        Some(iced::Color::from_rgba(1.0, 1.0, 1.0, 0.1).into())
                    } else {
                        None
                    },
                    text_color: iced::Color::WHITE,
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .padding(8);

            stack = stack.push(
                container(button)
                    .width(Length::Fixed(80.0))
                    .height(Length::Fixed(80.0))
                    .padding(iced::Padding {
                        top: pos.y,
                        left: pos.x,
                        ..Default::default()
                    }),
            );
        }

        // Marquee selection overlay
        if let Some(rect) = self.selection_rect {
            stack = stack.push(
                container(
                    container(iced::widget::Space::new(
                        Length::Fixed(rect.width),
                        Length::Fixed(rect.height),
                    ))
                    .style(|_| container::Style {
                        background: Some(iced::Color::from_rgba(0.0, 0.5, 1.0, 0.1).into()),
                        border: iced::Border {
                            color: iced::Color::from_rgba(0.0, 0.5, 1.0, 0.5),
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    }),
                )
                .padding(iced::Padding {
                    top: rect.y,
                    left: rect.x,
                    ..Default::default()
                }),
            );
        }

        container(stack)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&mut self, message: DesktopMessage) -> Option<DesktopMessage> {
        match message {
            DesktopMessage::Select(path, is_multi) => {
                let now = std::time::Instant::now();
                if let Some((last_path, last_time)) = &self.last_click {
                    if last_path == &path
                        && now.duration_since(*last_time) < std::time::Duration::from_millis(500)
                    {
                        self.last_click = None;
                        return Some(DesktopMessage::Open(path));
                    }
                }

                if is_multi {
                    if let Some(pos) = self.selected.iter().position(|p| p == &path) {
                        self.selected.remove(pos);
                    } else {
                        self.selected.push(path.clone());
                    }
                } else {
                    self.selected = vec![path.clone()];
                }
                self.last_click = Some((path, now));
            }
            DesktopMessage::StartSelection(point, modifiers) => {
                self.drag_start = Some(point);
                if !modifiers.shift() && !modifiers.command() {
                    self.selected.clear();
                }
                self.selection_rect = None;
            }
            DesktopMessage::UpdateSelection(point) => {
                if let Some(start) = self.drag_start {
                    let min_x = start.x.min(point.x);
                    let min_y = start.y.min(point.y);
                    let max_x = start.x.max(point.x);
                    let max_y = start.y.max(point.y);

                    let rect = iced::Rectangle {
                        x: min_x,
                        y: min_y,
                        width: max_x - min_x,
                        height: max_y - min_y,
                    };
                    self.selection_rect = Some(rect);

                    // Select items intersecting with rect
                    self.selected.clear();
                    for item in &self.items {
                        let item_rect = iced::Rectangle {
                            x: item.position.x,
                            y: item.position.y,
                            width: 80.0,
                            height: 80.0,
                        };
                        if rect.intersects(&item_rect) {
                            self.selected.push(item.path.clone());
                        }
                    }
                }
            }
            DesktopMessage::EndSelection => {
                self.drag_start = None;
                self.selection_rect = None;
            }
            DesktopMessage::Open(_path) => {}
            _ => {}
        }
        None
    }

    pub fn is_over_icon(&self, point: Point) -> bool {
        for item in &self.items {
            let rect = iced::Rectangle {
                x: item.position.x,
                y: item.position.y,
                width: 80.0,
                height: 80.0,
            };
            if rect.contains(point) {
                return true;
            }
        }
        false
    }
}

// Reuse the icon logic from explorer.rs or move to a common place
fn view_icon<'a>(path: &std::path::Path) -> Element<'a, DesktopMessage> {
    let is_dir = path.is_dir();
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    let icon_name = if is_dir {
        "folder"
    } else {
        match ext.as_str() {
            "png" | "jpg" | "jpeg" | "gif" | "svg" => "image",
            "mp3" | "wav" | "flac" | "ogg" => "media",
            "rs" | "sh" | "toml" | "json" | "md" | "txt" => "document",
            _ => "document",
        }
    };

    svg(peak_core::icons::get_ui_icon(icon_name, "#FFFFFF"))
        .width(Length::Fixed(32.0))
        .height(Length::Fixed(32.0))
        .into()
}
