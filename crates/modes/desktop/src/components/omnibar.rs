use iced::widget::{button, column, container, row, text, text_input};
use iced::{Alignment, Background, Color, Element, Length};
use peak_core::registry::{AppId, AppInfo};

// Menu items for the default view
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItem {
    Apps,
    Learn,
    Trigger,
    Style,
    Setup,
    Install,
    Remove,
    Update,
    About,
    System,
}

impl MenuItem {
    fn icon(&self) -> &'static str {
        match self {
            MenuItem::Apps => "apps",
            MenuItem::Learn => "learn",
            MenuItem::Trigger => "trigger",
            MenuItem::Style => "style",
            MenuItem::Setup => "setup",
            MenuItem::Install => "install",
            MenuItem::Remove => "remove",
            MenuItem::Update => "update",
            MenuItem::About => "about",
            MenuItem::System => "system",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            MenuItem::Apps => "Apps",
            MenuItem::Learn => "Learn",
            MenuItem::Trigger => "Trigger",
            MenuItem::Style => "Style",
            MenuItem::Setup => "Setup",
            MenuItem::Install => "Install",
            MenuItem::Remove => "Remove",
            MenuItem::Update => "Update",
            MenuItem::About => "About",
            MenuItem::System => "System",
        }
    }

    fn all() -> Vec<Self> {
        vec![
            MenuItem::Apps,
            MenuItem::Learn,
            MenuItem::Trigger,
            MenuItem::Style,
            MenuItem::Setup,
            MenuItem::Install,
            MenuItem::Remove,
            MenuItem::Update,
            MenuItem::About,
            MenuItem::System,
        ]
    }
}

// Operating mode of the Spotlight
#[derive(Debug, Clone, PartialEq)]
pub enum OmnibarMode {
    Menu,    // Show menu items
    Search,  // Search local apps/files
    Install, // APK package search
}

// Search results for local apps
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub app_id: Option<AppId>,
}

// APK package result
#[derive(Debug, Clone)]
pub struct ApkPackage {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum OmnibarMessage {
    QueryChanged(String),
    Submit,
    SelectMenuItem(MenuItem),
    SelectApp(AppId),
    SelectApk(String),
    NavigateUp,
    NavigateDown,
    Cancel,
    ApkResults(Vec<ApkPackage>),
}

pub struct Omnibar {
    query: String,
    mode: OmnibarMode,
    menu_items: Vec<MenuItem>,
    search_results: Vec<SearchResult>,
    apk_results: Vec<ApkPackage>,
    selected_index: usize,
}

impl Omnibar {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            mode: OmnibarMode::Menu,
            menu_items: MenuItem::all(),
            search_results: Vec::new(),
            apk_results: Vec::new(),
            selected_index: 0,
        }
    }

    pub fn get_selected_app(&self) -> Option<AppId> {
        if self.mode == OmnibarMode::Search {
            self.search_results.get(self.selected_index)?.app_id
        } else {
            None
        }
    }

    pub fn get_selected_apk(&self) -> Option<String> {
        if self.mode == OmnibarMode::Install {
            self.apk_results
                .get(self.selected_index)
                .map(|p| p.name.clone())
        } else {
            None
        }
    }

    pub fn get_selected_menu_item(&self) -> Option<MenuItem> {
        if self.mode == OmnibarMode::Menu {
            self.menu_items.get(self.selected_index).copied()
        } else {
            None
        }
    }

    pub fn update(&mut self, message: OmnibarMessage) -> iced::Task<OmnibarMessage> {
        match message {
            OmnibarMessage::QueryChanged(new_query) => {
                self.query = new_query;

                if self.query.is_empty() {
                    // Return to menu mode
                    self.mode = OmnibarMode::Menu;
                    self.search_results = Vec::new();
                    self.apk_results = Vec::new();
                    self.selected_index = 0;
                    return iced::Task::none();
                }

                let trimmed = self.query.to_lowercase();

                // Enter search mode (or install mode if previously in install)
                if self.mode != OmnibarMode::Install {
                    self.mode = OmnibarMode::Search;
                }

                // Perform search based on mode
                match self.mode {
                    OmnibarMode::Search => {
                        // Search local apps
                        let mut hits = Vec::new();

                        for app in AppInfo::all() {
                            if app.name.to_lowercase().contains(&trimmed) {
                                hits.push(SearchResult {
                                    name: app.name.to_string(),
                                    app_id: Some(app.id),
                                });
                            }
                        }

                        self.search_results = hits;
                        self.selected_index = 0;
                    }
                    OmnibarMode::Install => {
                        // Search APK packages
                        if trimmed.len() > 2 {
                            let query = trimmed.clone();
                            return iced::Task::perform(
                                search_apk_packages(query),
                                OmnibarMessage::ApkResults,
                            );
                        }
                    }
                    _ => {}
                }
            }
            OmnibarMessage::ApkResults(results) => {
                self.apk_results = results;
            }
            OmnibarMessage::SelectMenuItem(item) => {
                if item == MenuItem::Install {
                    // Enter install mode
                    self.mode = OmnibarMode::Install;
                    // Don't clear query - keep it for immediate search
                    self.selected_index = 0;

                    // If there's already a query, search immediately
                    if self.query.len() > 2 {
                        let query = self.query.to_lowercase();
                        return iced::Task::perform(
                            search_apk_packages(query),
                            OmnibarMessage::ApkResults,
                        );
                    }
                }
                // Handle other menu items...
            }
            OmnibarMessage::SelectApk(pkg_name) => {
                // In a real system, this would trigger installation via the store.
                // For now, we print it to console.
                println!("Installing APK: {}", pkg_name);
            }
            OmnibarMessage::NavigateUp => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            OmnibarMessage::NavigateDown => {
                let max_index = match self.mode {
                    OmnibarMode::Menu => self.menu_items.len().saturating_sub(1),
                    OmnibarMode::Search => self.search_results.len().saturating_sub(1),
                    OmnibarMode::Install => self.apk_results.len().saturating_sub(1),
                };
                if self.selected_index < max_index {
                    self.selected_index += 1;
                }
            }
            _ => {}
        }
        iced::Task::none()
    }

    pub fn view(&self, tokens: peak_theme::ThemeTokens) -> Element<'_, OmnibarMessage> {
        // Input field
        let input = text_input("Go...", &self.query)
            .on_input(OmnibarMessage::QueryChanged)
            .on_submit(OmnibarMessage::Submit)
            .padding(12)
            .size(16)
            .style(move |_, _| text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: iced::Border {
                    width: 0.0,
                    ..Default::default()
                },
                icon: tokens.colors.text_primary,
                placeholder: {
                    let mut c = tokens.colors.text_primary;
                    c.a = 0.4;
                    c
                },
                value: tokens.colors.text_primary,
                selection: tokens.colors.primary,
            });

        // Content based on mode
        let content: Element<_> = match self.mode {
            OmnibarMode::Menu => self.view_menu(tokens),
            OmnibarMode::Search => self.view_search_results(tokens),
            OmnibarMode::Install => self.view_install_packages(tokens),
        };

        // Main container
        container(column![input, content].spacing(10))
            .width(Length::Fixed(400.0))
            .padding(16)
            .style(move |_| container::Style {
                background: Some(Background::Color({
                    let mut c = tokens.colors.surface;
                    c.a = tokens.glass_opacity;
                    c
                })),
                border: iced::Border {
                    color: tokens.colors.border,
                    width: 1.0,
                    radius: tokens.radius.into(),
                },
                shadow: iced::Shadow {
                    color: tokens.shadow_color,
                    offset: iced::Vector::new(0.0, 20.0),
                    blur_radius: 40.0,
                },
                ..Default::default()
            })
            .into()
    }

    fn view_menu(&self, tokens: peak_theme::ThemeTokens) -> Element<'_, OmnibarMessage> {
        let items: Vec<Element<OmnibarMessage>> = self
            .menu_items
            .iter()
            .enumerate()
            .map(|(i, item)| {
                let is_selected = i == self.selected_index;
                self.view_menu_item(item, is_selected, tokens)
            })
            .collect();

        column(items).spacing(4).into()
    }

    fn view_menu_item(
        &self,
        item: &MenuItem,
        is_selected: bool,
        tokens: peak_theme::ThemeTokens,
    ) -> Element<'_, OmnibarMessage> {
        let bg_color = if is_selected {
            let mut c = tokens.colors.primary;
            c.a = 0.2;
            c
        } else {
            Color::TRANSPARENT
        };

        let text_color = if is_selected {
            tokens.colors.primary
        } else {
            tokens.colors.text_primary
        };

        let hex_color = format!(
            "#{:02x}{:02x}{:02x}",
            (text_color.r * 255.0) as u8,
            (text_color.g * 255.0) as u8,
            (text_color.b * 255.0) as u8
        );

        let icon = iced::widget::svg(peak_core::icons::get_ui_icon(item.icon(), &hex_color))
            .width(Length::Fixed(16.0))
            .height(Length::Fixed(16.0));

        button(
            row![icon, text(item.label()).size(14).color(text_color)]
                .spacing(12)
                .align_y(Alignment::Center),
        )
        .on_press(OmnibarMessage::SelectMenuItem(*item))
        .padding([8, 12])
        .width(Length::Fill)
        .style(move |_, status| {
            let mut final_bg = bg_color;
            if status == button::Status::Hovered && !is_selected {
                final_bg = tokens.colors.text_primary;
                final_bg.a = 0.1;
            }

            button::Style {
                background: Some(Background::Color(final_bg)),
                text_color,
                border: iced::Border {
                    radius: tokens.radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn view_search_results(&self, tokens: peak_theme::ThemeTokens) -> Element<'_, OmnibarMessage> {
        if self.search_results.is_empty() {
            let mut text_color = tokens.colors.text_primary;
            text_color.a = 0.4;
            container(text("No apps found").size(14).color(text_color))
                .padding(20)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
        } else {
            let items: Vec<Element<OmnibarMessage>> = self
                .search_results
                .iter()
                .enumerate()
                .map(|(i, result)| {
                    let is_selected = i == self.selected_index;
                    self.view_search_result(result, is_selected, tokens)
                })
                .collect();

            column(items).spacing(4).into()
        }
    }

    fn view_search_result<'a>(
        &'a self,
        result: &'a SearchResult,
        is_selected: bool,
        tokens: peak_theme::ThemeTokens,
    ) -> Element<'a, OmnibarMessage> {
        let bg_color = if is_selected {
            let mut c = tokens.colors.primary;
            c.a = 0.2;
            c
        } else {
            Color::TRANSPARENT
        };

        let text_color = if is_selected {
            tokens.colors.primary
        } else {
            tokens.colors.text_primary
        };

        let msg = if let Some(app_id) = result.app_id {
            OmnibarMessage::SelectApp(app_id)
        } else {
            OmnibarMessage::Submit
        };

        let hex_color = format!(
            "#{:02x}{:02x}{:02x}",
            (text_color.r * 255.0) as u8,
            (text_color.g * 255.0) as u8,
            (text_color.b * 255.0) as u8
        );

        let icon = if let Some(app_id) = result.app_id {
            iced::widget::svg(peak_core::icons::get_app_icon(app_id, &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
        } else {
            iced::widget::svg(peak_core::icons::get_ui_icon("document", &hex_color))
                .width(Length::Fixed(16.0))
                .height(Length::Fixed(16.0))
        };

        button(
            row![icon, text(&result.name).size(14).color(text_color)]
                .spacing(12)
                .align_y(Alignment::Center),
        )
        .on_press(msg)
        .padding([8, 12])
        .width(Length::Fill)
        .style(move |_, status| {
            let mut final_bg = bg_color;
            if status == button::Status::Hovered && !is_selected {
                final_bg = tokens.colors.text_primary;
                final_bg.a = 0.1;
            }
            button::Style {
                background: Some(Background::Color(final_bg)),
                text_color,
                border: iced::Border {
                    radius: tokens.radius.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
    }

    fn view_install_packages(
        &self,
        tokens: peak_theme::ThemeTokens,
    ) -> Element<'_, OmnibarMessage> {
        let mut placeholder_color = tokens.colors.text_primary;
        placeholder_color.a = 0.4;

        if self.query.is_empty() {
            container(
                text("Search for packages to install")
                    .size(14)
                    .color(placeholder_color),
            )
            .padding(20)
            .width(Length::Fill)
            .center_x(Length::Fill)
            .into()
        } else if self.apk_results.is_empty() {
            container(text("Searching...").size(14).color(placeholder_color))
                .padding(20)
                .width(Length::Fill)
                .center_x(Length::Fill)
                .into()
        } else {
            let items: Vec<Element<OmnibarMessage>> = self
                .apk_results
                .iter()
                .enumerate()
                .map(|(i, pkg)| {
                    let is_selected = i == self.selected_index;
                    let bg_color = if is_selected {
                        let mut c = tokens.colors.primary;
                        c.a = 0.2;
                        c
                    } else {
                        Color::TRANSPARENT
                    };

                    let text_color = if is_selected {
                        tokens.colors.primary
                    } else {
                        tokens.colors.text_primary
                    };

                    button(
                        column![
                            text(&pkg.name).size(14).color(text_color),
                            text(&pkg.description).size(12).color({
                                let mut c = text_color;
                                c.a = 0.6;
                                c
                            }),
                        ]
                        .spacing(4),
                    )
                    .on_press(OmnibarMessage::SelectApk(pkg.name.clone()))
                    .padding([8, 12])
                    .width(Length::Fill)
                    .style(move |_, status| {
                        let mut final_bg = bg_color;
                        if status == button::Status::Hovered && !is_selected {
                            final_bg = tokens.colors.text_primary;
                            final_bg.a = 0.1;
                        }
                        button::Style {
                            background: Some(Background::Color(final_bg)),
                            border: iced::Border {
                                radius: tokens.radius.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                    })
                    .into()
                })
                .collect();

            column(items).spacing(2).into()
        }
    }
}

async fn search_apk_packages(query: String) -> Vec<ApkPackage> {
    // Check if apk command exists
    let apk_exists = tokio::process::Command::new("which")
        .arg("apk")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !apk_exists {
        // Fallback for development on macOS - return mock data
        return vec![
            ApkPackage {
                name: format!("{}-example", query),
                description: "Example package (apk not available on this system)".to_string(),
            },
            ApkPackage {
                name: format!("{}-dev", query),
                description: "Development package (mock data)".to_string(),
            },
            ApkPackage {
                name: format!("{}-docs", query),
                description: "Documentation package (mock data)".to_string(),
            },
        ];
    }

    // Run apk search
    match tokio::process::Command::new("apk")
        .arg("search")
        .arg(&query)
        .output()
        .await
    {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .take(10)
                    .filter_map(|line| {
                        let parts: Vec<&str> = line.splitn(2, " - ").collect();
                        if parts.len() == 2 {
                            Some(ApkPackage {
                                name: parts[0].trim().to_string(),
                                description: parts[1].trim().to_string(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            } else {
                Vec::new()
            }
        }
        Err(_) => Vec::new(),
    }
}
