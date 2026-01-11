use crate::registry::{AppId, AppInfo};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Background, Border, Color, Element, Length};

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum SearchCategory {
    Applications,
    Files,
    Web,
    AUR,
}

#[derive(serde::Deserialize)]
struct AurRpcResponse {
    results: Vec<AurPackage>,
}

#[derive(serde::Deserialize)]
struct AurPackage {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Description")]
    description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    pub name: String,
    pub category: SearchCategory,
    pub app_id: Option<AppId>,
    pub path: Option<String>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OmnibarMessage {
    SearchChanged(String),
    Confirm,       // Enter pressed
    Cancel,        // Esc pressed
    Select(usize), // Selection by index
    AurResults(Vec<SearchResult>),
}

async fn search_aur(query: String) -> Vec<SearchResult> {
    let url = format!(
        "https://aur.archlinux.org/rpc/?v=5&type=search&arg={}",
        query
    );

    // We use a short timeout to not block UI responsiveness logic unnecessarily
    // (though Task::perform is async)
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .user_agent("PeakOS/0.1.0")
        .build();

    if let Ok(client) = client {
        if let Ok(resp) = client.get(&url).send().await {
            if let Ok(json) = resp.json::<AurRpcResponse>().await {
                return json
                    .results
                    .into_iter()
                    .take(5)
                    .map(|pkg| {
                        SearchResult {
                            id: pkg.name.clone(),
                            name: pkg.name.clone(),
                            category: SearchCategory::AUR,
                            app_id: None,
                            path: Some(pkg.description.unwrap_or_default()), // Use path field for desc
                        }
                    })
                    .collect();
            }
        }
    }
    Vec::new()
}

pub struct Omnibar {
    query: String,
    results: Vec<SearchResult>,
    selected_index: usize,
}

impl Omnibar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn get_selected(&self) -> Option<&SearchResult> {
        self.results.get(self.selected_index)
    }

    pub fn update(&mut self, message: OmnibarMessage) -> iced::Task<OmnibarMessage> {
        match message {
            OmnibarMessage::SearchChanged(new_query) => {
                self.query = new_query;

                if self.query.is_empty() {
                    self.results = Vec::new();
                    return iced::Task::none();
                }

                let mut hits = Vec::new();
                let trimmed = self.query.to_lowercase();

                // 1. Search Apps
                for app in AppInfo::all() {
                    if app.name.to_lowercase().contains(&trimmed) {
                        hits.push(SearchResult {
                            id: app.id.to_string(),
                            name: app.name.to_string(),
                            category: SearchCategory::Applications,
                            app_id: Some(app.id),
                            path: None,
                        });
                    }
                }

                self.results = hits;
                self.selected_index = 0;

                // 2. Perform AUR Search asynchronously
                if trimmed.len() > 2 {
                    let query = trimmed.clone();
                    return iced::Task::perform(search_aur(query), OmnibarMessage::AurResults);
                }
            }
            OmnibarMessage::AurResults(aur_hits) => {
                // Merge AUR results
                // Limit total results if needed
                self.results.extend(aur_hits);
            }
            OmnibarMessage::Select(idx) => {
                self.selected_index = idx;
            }
            _ => {}
        }
        iced::Task::none()
    }
    pub fn view(&self) -> Element<'_, OmnibarMessage> {
        // 1. The Search Input
        let input = text_input("Search PeakOS...", &self.query)
            .on_input(OmnibarMessage::SearchChanged)
            .on_submit(OmnibarMessage::Confirm)
            .padding(15)
            .size(20)
            .style(|_, _| text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: iced::Border {
                    width: 0.0,
                    ..Default::default()
                },
                icon: Color::WHITE,
                placeholder: Color::from_rgba(1.0, 1.0, 1.0, 0.5),
                value: Color::WHITE,
                selection: Color::from_rgb(0.3, 0.3, 0.5),
            });

        // 2. The Results List
        let results: Element<_> = if self.results.is_empty() {
            container(
                text(if self.query.is_empty() {
                    "Start typing to search..."
                } else {
                    "No results found"
                })
                .color(Color::from_rgba(1.0, 1.0, 1.0, 0.3)),
            )
            .padding(40)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        } else {
            let list = column(self.results.iter().enumerate().map(|(i, res)| {
                let is_selected = i == self.selected_index;
                let bg = if is_selected {
                    Color::from_rgba(1.0, 0.647, 0.0, 0.8) // Peak Orange
                } else {
                    Color::TRANSPARENT
                };

                container(
                    button(
                        row![
                            text("○").size(14),
                            column![
                                text(&res.name).size(15).font(iced::Font::DEFAULT),
                                row![
                                    text(format!("{:?}", res.category).to_uppercase())
                                        .size(9)
                                        .color(if is_selected {
                                            Color::WHITE
                                        } else {
                                            Color::from_rgba(1.0, 1.0, 1.0, 0.4)
                                        }),
                                    text(format!(" • {}", res.id))
                                        .size(8)
                                        .color(Color::from_rgba(1.0, 1.0, 1.0, 0.15))
                                ]
                                .spacing(5)
                            ]
                        ]
                        .spacing(15)
                        .align_y(Alignment::Center),
                    )
                    .on_press(OmnibarMessage::Select(i))
                    .style(move |_theme, _status| button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        text_color: Color::WHITE,
                        ..Default::default()
                    })
                    .width(Length::Fill),
                )
                .padding(10)
                .style(move |_| container::Style {
                    background: Some(Background::Color(bg)),
                    border: Border {
                        radius: 12.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .into()
            }))
            .spacing(5);

            scrollable(list).height(Length::Shrink).into()
        };

        // 3. The Glass Container
        container(column![
            input,
            iced::widget::Rule::horizontal(1).style(|_| iced::widget::rule::Style {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
                width: 1,
                radius: 0.0.into(),
                fill_mode: iced::widget::rule::FillMode::Full
            }),
            results
        ])
        .width(Length::Fixed(600.0))
        .padding(10)
        .style(move |_| container::Style {
            background: Some(Background::Color(Color::from_rgba(0.05, 0.05, 0.1, 0.85))), // Dark Glass
            border: iced::Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: iced::Shadow {
                color: Color::BLACK,
                offset: iced::Vector::new(0.0, 20.0),
                blur_radius: 50.0,
            },
            ..Default::default()
        })
        .into()
    }
}
