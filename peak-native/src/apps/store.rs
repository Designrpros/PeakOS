use crate::app::Message;
use crate::styles;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::Task;
use iced::{Color, Element, Length};

#[derive(Debug, Clone)]
pub struct StoreApp {
    pub search_query: String,
    #[allow(dead_code)]
    pub is_loading: bool,
    pub search_results: Vec<AppPackage>,
    pub all_apps: Vec<AppPackage>,
    pub selected_category: Option<AppCategory>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppCategory {
    Web,
    Creative,
    Development,
    Media,
    Utility,
    Games,
    System,
}

impl AppCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            AppCategory::Web => "Web",
            AppCategory::Creative => "Creative",
            AppCategory::Development => "Dev",
            AppCategory::Media => "Media",
            AppCategory::Utility => "Utility",
            AppCategory::Games => "Games",
            AppCategory::System => "System",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AppCategory::Web => "icons/browser.svg",
            AppCategory::Creative => "icons/palette.svg",
            AppCategory::Development => "icons/terminal.svg",
            AppCategory::Media => "icons/media.svg",
            AppCategory::Utility => "icons/settings.svg",
            AppCategory::Games => "icons/library.svg", // Recycle existing or new
            AppCategory::System => "icons/settings.svg",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppPackage {
    pub name: String,
    pub description: String,
    pub category: AppCategory,
    pub version: String,
    pub is_installed: bool,
}

#[derive(Debug, Clone)]
pub enum StoreMessage {
    SearchChanged(String),
    SearchSubmit,
    InstallApp(String), // App Name
    LaunchApp(String),  // New: Launch installed app
    #[allow(dead_code)]
    UninstallApp(String),
    SelectCategory(Option<AppCategory>),
}

impl StoreApp {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            is_loading: false,
            search_results: crate::apps::store::get_initial_apps(), // Start with all/featured
            selected_category: None,
            all_apps: crate::apps::store::get_initial_apps(),
        }
    }

    pub fn update(&mut self, message: StoreMessage) -> Task<Message> {
        match message {
            StoreMessage::SearchChanged(query) => {
                self.search_query = query;
                // Auto-filter as you type if desired, or wait for submit
                // Let's filter immediately for responsiveness
                self.filter_apps();
            }
            StoreMessage::SearchSubmit => {
                self.filter_apps();
            }
            StoreMessage::SelectCategory(cat) => {
                // Toggle if same category selected
                if self.selected_category == cat && cat.is_some() {
                    self.selected_category = None;
                } else {
                    self.selected_category = cat;
                }
                self.filter_apps();
            }
            StoreMessage::InstallApp(name) => {
                let pkg_name = name.to_lowercase();
                self.is_loading = true; // Optimistic UI update

                // Spawn background installation
                std::thread::spawn(move || {
                    // Try Alpine APK first
                    let _ = std::process::Command::new("apk")
                        .arg("add")
                        .arg(&pkg_name)
                        .status();

                    // Fallback or other package managers could go here
                });

                // We optimistically mark as installed for immediate feedback,
                // but real check happens on refresh.
                self.update_app_status(&name, true);
            }
            StoreMessage::LaunchApp(name) => {
                let bin_name = name.to_lowercase();

                // Spawn detached process
                std::thread::spawn(move || {
                    let _ = std::process::Command::new(bin_name)
                        .spawn()
                        .map_err(|e| eprintln!("Failed to launch app: {}", e));
                });
            }
            StoreMessage::UninstallApp(name) => {
                self.update_app_status(&name, false);
            }
        }
        Task::none()
    }

    fn filter_apps(&mut self) {
        let query = self.search_query.to_lowercase();

        self.search_results = self
            .all_apps
            .iter()
            .filter(|app| {
                let matches_search = app.name.to_lowercase().contains(&query)
                    || app.description.to_lowercase().contains(&query);
                let matches_category = match self.selected_category {
                    Some(cat) => app.category == cat,
                    None => true,
                };

                matches_search && matches_category
            })
            .cloned()
            .collect();
    }

    fn update_app_status(&mut self, name: &str, installed: bool) {
        if let Some(app) = self.all_apps.iter_mut().find(|a| a.name == name) {
            app.is_installed = installed;
        }
        // Update search results ensuring synced state
        if let Some(app) = self.search_results.iter_mut().find(|a| a.name == name) {
            app.is_installed = installed;
        }
    }

    pub fn view<'a>(&'a self, is_light: bool) -> Element<'a, StoreMessage> {
        // Redesigned Header: Search Bar top, no Title text.

        let search_bar = text_input("Search packages...", &self.search_query)
            .on_input(StoreMessage::SearchChanged)
            .on_submit(StoreMessage::SearchSubmit)
            .padding(12)
            .width(Length::Fill)
            .style(move |_, status| styles::style_soft_input(status, is_light));

        // Category Pills
        let categories = [
            AppCategory::Web,
            AppCategory::Creative,
            AppCategory::Development,
            AppCategory::Media,
            AppCategory::Games,
            AppCategory::Utility,
            AppCategory::System,
        ];

        let category_row = row(categories
            .into_iter()
            .map(|cat| {
                let is_selected = self.selected_category == Some(cat);
                button(text(cat.as_str()).size(13))
                    .on_press(StoreMessage::SelectCategory(Some(cat)))
                    .padding([6, 16])
                    .style(move |_, status| {
                        if is_selected {
                            // Active Pill
                            button::Style {
                                background: Some(Color::from_rgb8(50, 100, 255).into()), // Blue accent
                                text_color: Color::WHITE,
                                border: iced::Border {
                                    radius: 15.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            }
                        } else {
                            // Inactive Pill
                            styles::style_pill_button(status, is_light)
                        }
                    })
                    .into()
            })
            .collect::<Vec<_>>())
        .spacing(10);

        let content: Element<StoreMessage> = if self.search_results.is_empty() {
            container(
                text("No apps found.")
                    .size(14)
                    .color(Color::from_rgb8(150, 150, 150)),
            )
            .width(Length::Fill)
            .align_x(iced::alignment::Horizontal::Center)
            .into()
        } else {
            column(
                self.search_results
                    .iter()
                    .map(|app| view_app_card(app, is_light))
                    .collect::<Vec<_>>(),
            )
            .spacing(10)
            .into()
        };

        container(
            column![
                search_bar,
                scrollable(category_row)
                    .direction(scrollable::Direction::Horizontal(Default::default())),
                scrollable(content).height(Length::Fill)
            ]
            .spacing(20)
            .padding(40),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(iced::alignment::Horizontal::Center)
        .into()
    }
}

fn view_app_card<'a>(app: &'a AppPackage, is_light: bool) -> Element<'a, StoreMessage> {
    let icon_path = app.category.icon();
    let theme_dir = if is_light { "black" } else { "white" };
    let filename = std::path::Path::new(icon_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

    let relative_themed = format!("icons/menubar/{}/{}", theme_dir, filename);
    let abs_themed = crate::utils::assets::get_asset_path(&relative_themed);

    let final_path = if std::path::Path::new(&abs_themed).exists() {
        abs_themed
    } else {
        crate::utils::assets::get_asset_path(icon_path)
    };

    let icon = container(
        iced::widget::svg(iced::widget::svg::Handle::from_path(final_path))
            .width(Length::Fixed(32.0))
            .height(Length::Fixed(32.0)),
    )
    .width(Length::Fixed(50.0))
    .height(Length::Fixed(50.0))
    .align_x(iced::alignment::Horizontal::Center)
    .align_y(iced::alignment::Vertical::Center)
    .style(move |_| container::Style {
        background: Some(if is_light {
            Color::from_rgb8(240, 240, 240).into()
        } else {
            Color::from_rgb8(40, 40, 40).into()
        }),
        border: iced::Border {
            radius: 12.0.into(),
            ..Default::default()
        },
        ..Default::default()
    });

    let info = column![
        text(&app.name)
            .size(16)
            .font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            })
            .style(move |_: &iced::Theme| text::Style {
                color: Some(if is_light { Color::BLACK } else { Color::WHITE })
            }),
        text(format!("{} • v{}", app.description, app.version))
            .size(12)
            .style(move |_: &iced::Theme| text::Style {
                color: Some(if is_light {
                    Color::from_rgb8(100, 100, 100)
                } else {
                    Color::from_rgb8(180, 180, 180)
                })
            }),
    ]
    .width(Length::Fill);

    let action_btn = if app.is_installed {
        button(text("Open").size(12))
            .on_press(StoreMessage::LaunchApp(app.name.clone()))
            .padding([8, 16])
            .style(move |_, status| styles::style_secondary_button(status, is_light))
    } else {
        button(
            text("Get")
                .size(12)
                .style(move |_: &iced::Theme| text::Style {
                    color: Some(if is_light { Color::WHITE } else { Color::BLACK }),
                }),
        )
        .on_press(StoreMessage::InstallApp(app.name.clone()))
        .padding([8, 16])
        .style(move |_, status| styles::style_pill_button(status, is_light))
    };

    container(
        row![icon, info, action_btn]
            .spacing(15)
            .align_y(iced::alignment::Vertical::Center)
            .padding(15),
    )
    .style(move |_| styles::style_glass_card(is_light))
    .width(Length::Fill)
    .into()
}

fn check_installed(name: &str) -> bool {
    // Special case for Antigravity (Peak Intelligence)
    if name == "Antigravity" {
        return true;
    }

    // Check path for common apps
    let linux_path = format!("/usr/bin/{}", name.to_lowercase());
    if std::path::Path::new(&linux_path).exists() {
        return true;
    }

    // Mac Check
    let mac_app = format!("/Applications/{}.app", name);
    if std::path::Path::new(&mac_app).exists() {
        return true;
    }

    // Check via 'which' command as fallback
    if let Ok(output) = std::process::Command::new("which")
        .arg(name.to_lowercase())
        .output()
    {
        return output.status.success();
    }

    false
}

pub fn get_initial_apps() -> Vec<AppPackage> {
    vec![
        // FIRST: Antigravity
        AppPackage {
            name: "Antigravity".into(),
            description: "Google Agentic IDE.".into(),
            category: AppCategory::Utility, // Or Development
            version: "1.0.0".into(),
            is_installed: check_installed("Antigravity"),
        },
        // WEB
        AppPackage {
            name: "Firefox".into(),
            description: "Fast, private browser.".into(),
            category: AppCategory::Web,
            version: "120.0".into(),
            is_installed: check_installed("firefox"),
        },
        AppPackage {
            name: "Chromium".into(),
            description: "Open source web browser.".into(),
            category: AppCategory::Web,
            version: "119.0".into(),
            is_installed: check_installed("chromium"),
        },
        AppPackage {
            name: "Brave".into(),
            description: "Privacy-focused browser.".into(),
            category: AppCategory::Web,
            version: "1.60".into(),
            is_installed: check_installed("brave"),
        },
        // CREATIVE
        AppPackage {
            name: "Blender".into(),
            description: "3D Creation Suite.".into(),
            category: AppCategory::Creative,
            version: "4.0".into(),
            is_installed: check_installed("blender"),
        },
        AppPackage {
            name: "GIMP".into(),
            description: "GNU Image Manipulation Program.".into(),
            category: AppCategory::Creative,
            version: "2.10".into(),
            is_installed: check_installed("gimp"),
        },
        AppPackage {
            name: "Inkscape".into(),
            description: "Vector Graphics Editor.".into(),
            category: AppCategory::Creative,
            version: "1.3".into(),
            is_installed: check_installed("inkscape"),
        },
        AppPackage {
            name: "Krita".into(),
            description: "Digital Painting.".into(),
            category: AppCategory::Creative,
            version: "5.2".into(),
            is_installed: check_installed("krita"),
        },
        AppPackage {
            name: "Audacity".into(),
            description: "Audio Editor.".into(),
            category: AppCategory::Creative,
            version: "3.4".into(),
            is_installed: check_installed("audacity"),
        },
        // DEV
        AppPackage {
            name: "VS Code".into(),
            description: "Code Editor.".into(),
            category: AppCategory::Development,
            version: "1.85".into(),
            is_installed: check_installed("code"),
        },
        AppPackage {
            name: "Zed".into(),
            description: "High-performance editor.".into(),
            category: AppCategory::Development,
            version: "0.120".into(),
            is_installed: check_installed("zed"),
        },
        AppPackage {
            name: "Sublime Text".into(),
            description: "Sophisticated text editor.".into(),
            category: AppCategory::Development,
            version: "4.0".into(),
            is_installed: check_installed("subl"),
        },
        AppPackage {
            name: "Docker".into(),
            description: "Container Platform.".into(),
            category: AppCategory::Development,
            version: "24.0".into(),
            is_installed: check_installed("docker"),
        },
        AppPackage {
            name: "Git".into(),
            description: "Version Control.".into(),
            category: AppCategory::Development,
            version: "2.42".into(),
            is_installed: check_installed("git"),
        },
        // MEDIA
        AppPackage {
            name: "VLC".into(),
            description: "Media Player.".into(),
            category: AppCategory::Media,
            version: "3.0.20".into(),
            is_installed: check_installed("vlc"),
        },
        AppPackage {
            name: "Spotify".into(),
            description: "Music Streaming.".into(),
            category: AppCategory::Media,
            version: "1.2".into(),
            is_installed: check_installed("Spotify"),
        },
        AppPackage {
            name: "OBS Studio".into(),
            description: "Live Streaming software.".into(),
            category: AppCategory::Media,
            version: "30.0".into(),
            is_installed: check_installed("obs"),
        },
        AppPackage {
            name: "Discord".into(),
            description: "Chat & VoIP.".into(),
            category: AppCategory::Media,
            version: "0.0.30".into(),
            is_installed: check_installed("discord"),
        },
        AppPackage {
            name: "Stremio".into(),
            description: "Media Center.".into(),
            category: AppCategory::Media,
            version: "4.4".into(),
            is_installed: check_installed("stremio"),
        },
        // UTILITY
        AppPackage {
            name: "System Tuner".into(),
            description: "Optimization Tool.".into(),
            category: AppCategory::Utility,
            version: "1.0".into(),
            is_installed: true,
        }, // Built-in?
        AppPackage {
            name: "Bitwarden".into(),
            description: "Password Manager.".into(),
            category: AppCategory::Utility,
            version: "2023.12".into(),
            is_installed: check_installed("bitwarden"),
        },
        AppPackage {
            name: "Etcher".into(),
            description: "Flash OS images to SD cards.".into(),
            category: AppCategory::Utility,
            version: "1.18".into(),
            is_installed: check_installed("balenaetcher"),
        },
        // GAMES
        AppPackage {
            name: "Steam".into(),
            description: "Gaming Platform.".into(),
            category: AppCategory::Games,
            version: "1.0".into(),
            is_installed: check_installed("steam"),
        },
        AppPackage {
            name: "Lutris".into(),
            description: "Open Source Gaming Platform.".into(),
            category: AppCategory::Games,
            version: "0.5.14".into(),
            is_installed: check_installed("lutris"),
        },
        AppPackage {
            name: "Minecraft".into(),
            description: "Block building game.".into(),
            category: AppCategory::Games,
            version: "1.20".into(),
            is_installed: check_installed("minecraft-launcher"),
        },
        // SYSTEM
        AppPackage {
            name: "Nvidia Drivers".into(),
            description: "Graphics Drivers.".into(),
            category: AppCategory::System,
            version: "550.0".into(),
            is_installed: true,
        },
        AppPackage {
            name: "Linux Kernel".into(),
            description: "Core System.".into(),
            category: AppCategory::System,
            version: "6.6.7".into(),
            is_installed: true,
        },
    ]
}
