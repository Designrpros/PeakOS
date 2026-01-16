use crate::app::Message;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::Task;
use iced::{Color, Element, Length};
use peak_core::integrations::appimage::{AppImageInfo, AppImageManager};
use peak_core::styles;

#[derive(Debug, Clone)]
pub struct StoreApp {
    pub search_query: String,
    #[allow(dead_code)]
    pub is_loading: bool,
    pub search_results: Vec<AppPackage>,
    pub all_apps: Vec<AppPackage>,
    pub selected_category: Option<AppCategory>,
    pub installing_apps: Vec<String>, // Track which apps are currently installing
    pub appimage_manager: AppImageManager,
    pub installed_appimages: Vec<AppImageInfo>,
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
            AppCategory::Web => "browser",
            AppCategory::Creative => "palette",
            AppCategory::Development => "terminal",
            AppCategory::Media => "media",
            AppCategory::Utility => "settings",
            AppCategory::Games => "console",
            AppCategory::System => "settings",
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppPackage {
    pub name: String,
    pub description: String,
    pub category: AppCategory,
    #[allow(dead_code)]
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
    SearchResultsReceived(Vec<AppPackage>),
    InstallationComplete(String, bool), // (app_name, success)
    InstallAppImage,                    // Open file picker
    AppImageSelected(Option<std::path::PathBuf>),
    #[allow(dead_code)]
    LaunchAppImage(String), // Launch by name
}

impl StoreApp {
    pub fn new() -> Self {
        let appimage_manager = AppImageManager::new();
        let installed_appimages = appimage_manager.list_installed();

        Self {
            search_query: String::new(),
            is_loading: false,
            search_results: crate::apps::store::get_initial_apps(), // Start with all/featured
            selected_category: None,
            all_apps: crate::apps::store::get_initial_apps(),
            installing_apps: Vec::new(),
            appimage_manager,
            installed_appimages,
        }
    }

    pub fn update(&mut self, message: StoreMessage) -> Task<Message> {
        match message {
            StoreMessage::SearchChanged(query) => {
                self.search_query = query;
                self.filter_apps();
                Task::none()
            }
            StoreMessage::SearchSubmit => {
                if self.search_query.trim().is_empty() {
                    self.filter_apps();
                    Task::none()
                } else {
                    let query = self.search_query.clone();
                    Task::perform(search_apk(query), StoreMessage::SearchResultsReceived)
                        .map(Message::Store)
                }
            }
            StoreMessage::SearchResultsReceived(results) => {
                self.search_results = results;
                Task::none()
            }
            StoreMessage::SelectCategory(cat) => {
                // Toggle if same category selected
                if self.selected_category == cat && cat.is_some() {
                    self.selected_category = None;
                } else {
                    self.selected_category = cat;
                }
                self.filter_apps();
                Task::none()
            }
            StoreMessage::InstallApp(name) => {
                let pkg_name = name.to_lowercase();

                // Add to installing list
                if !self.installing_apps.contains(&name) {
                    self.installing_apps.push(name.clone());
                }

                // Spawn async installation task
                Task::perform(install_package(pkg_name), move |success| {
                    Message::Store(StoreMessage::InstallationComplete(name.clone(), success))
                })
            }
            StoreMessage::InstallationComplete(name, success) => {
                // Remove from installing list
                self.installing_apps.retain(|app| app != &name);

                if success {
                    // Mark as installed
                    self.update_app_status(&name, true);
                } else {
                    // Could show error message here
                    eprintln!("Failed to install {}", name);
                }

                Task::none()
            }
            StoreMessage::LaunchApp(name) => {
                // Check if it's a URL
                if name.starts_with("http") || name.starts_with("www.") {
                    let url = if name.starts_with("www.") {
                        format!("https://{}", name)
                    } else {
                        name
                    };

                    return Task::done(Message::LaunchBrowser(url));
                } else {
                    let bin_name = name.to_lowercase();
                    // Intercept browser launches to use internal managed browser
                    if bin_name.contains("firefox")
                        || bin_name.contains("chrome")
                        || bin_name.contains("brave")
                        || bin_name == "netscape"
                    {
                        return Task::done(Message::LaunchBrowser("https://google.com".into()));
                    }

                    // Spawn detached process
                    std::thread::spawn(move || {
                        let _ = std::process::Command::new(bin_name)
                            .spawn()
                            .map_err(|e| eprintln!("Failed to launch app: {}", e));
                    });
                    Task::none()
                }
            }
            StoreMessage::UninstallApp(name) => {
                self.update_app_status(&name, false);
                Task::none()
            }
            StoreMessage::InstallAppImage => {
                // Open file picker for .AppImage files
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("AppImage", &["AppImage"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    StoreMessage::AppImageSelected,
                )
                .map(Message::Store)
            }
            StoreMessage::AppImageSelected(path) => {
                if let Some(path) = path {
                    // Install the AppImage
                    match self.appimage_manager.install(&path) {
                        Ok(info) => {
                            self.installed_appimages.push(info);
                            println!("✓ AppImage installed successfully");
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to install AppImage: {}", e);
                        }
                    }
                }
                Task::none()
            }
            StoreMessage::LaunchAppImage(name) => {
                // Find and launch the AppImage
                if let Some(info) = self.installed_appimages.iter().find(|app| app.name == name) {
                    match self.appimage_manager.run(info) {
                        Ok(_) => {
                            println!("✓ Launched AppImage: {}", name);
                        }
                        Err(e) => {
                            eprintln!("✗ Failed to launch {}: {}", name, e);
                        }
                    }
                }
                Task::none()
            }
        }
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

        // Dynamic URL entry
        if query.starts_with("http") || query.starts_with("www.") {
            self.search_results.insert(
                0,
                AppPackage {
                    name: self.search_query.clone(),
                    description: "Go to Website".into(),
                    category: AppCategory::Web,
                    version: "Web".into(),
                    is_installed: true, // "Installed" so it shows "Open" button
                },
            );
        }
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
                    .map(|app| view_app_card(app, is_light, &self.installing_apps))
                    .collect::<Vec<_>>(),
            )
            .spacing(10)
            .into()
        };

        // AppImage install button
        let appimage_btn = button(text("+ AppImage").size(13))
            .on_press(StoreMessage::InstallAppImage)
            .padding([6, 16])
            .style(move |_, status| styles::style_pill_button(status, is_light));

        container(
            column![
                search_bar,
                row![
                    scrollable(category_row)
                        .direction(scrollable::Direction::Horizontal(Default::default())),
                    appimage_btn
                ]
                .spacing(10)
                .align_y(iced::alignment::Vertical::Center),
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

fn view_app_card<'a>(
    app: &'a AppPackage,
    is_light: bool,
    installing_apps: &'a [String],
) -> Element<'a, StoreMessage> {
    let icon_name = app.category.icon();
    let icon_color = if is_light { "#000000" } else { "#FFFFFF" };

    let icon = container(
        iced::widget::svg(peak_core::icons::get_ui_icon(icon_name, icon_color))
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
        text(&app.description)
            .size(14)
            .style(move |_| iced::widget::text::Style {
                color: Some(Color::from_rgba(0.5, 0.5, 0.5, 1.0)),
            }),
    ]
    .width(Length::Fill);

    let is_installing = installing_apps.contains(&app.name);

    let action_btn = if is_installing {
        button(text("Installing...").size(12))
            .padding([8, 16])
            .style(move |_, _| styles::style_secondary_button(button::Status::Disabled, is_light))
    } else if app.is_installed {
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
            name: "Docker".into(),
            description: "Container Platform.".into(),
            category: AppCategory::Development,
            version: "24.0".into(),
            is_installed: check_installed("docker"),
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

async fn search_apk(query: String) -> Vec<AppPackage> {
    if query.len() < 2 {
        return Vec::new();
    }

    // Run 'apk search -v -d [query]' for names, versions, and descriptions
    let output = match std::process::Command::new("apk")
        .arg("search")
        .arg("-v")
        .arg("-d")
        .arg(&query)
        .output()
    {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => return Vec::new(),
    };

    output
        .lines()
        .map(|line| {
            // line format: "name-version - description"
            let parts: Vec<&str> = line.splitn(2, " - ").collect();
            let name_ver = parts[0];
            let description = parts
                .get(1)
                .unwrap_or(&"No description available.")
                .to_string();

            // Try to split name and version (last hyphen usually)
            let last_hyphen = name_ver.rfind('-');
            let (name, version) = if let Some(idx) = last_hyphen {
                (name_ver[..idx].to_string(), name_ver[idx + 1..].to_string())
            } else {
                (name_ver.to_string(), "unknown".to_string())
            };

            AppPackage {
                name,
                description,
                category: AppCategory::Utility, // default for search
                version,
                is_installed: false, // will check on status update if we want more detail
            }
        })
        .collect()
}

async fn install_package(name: String) -> bool {
    // Check if apk command exists (Alpine Linux)
    let apk_exists = tokio::process::Command::new("which")
        .arg("apk")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false);

    if !apk_exists {
        eprintln!("⚠️  Package installation not available (apk not found)");
        eprintln!("   Note: Installation only works on Alpine Linux");
        eprintln!("   Current system: Development environment (macOS)");
        return false;
    }

    match tokio::process::Command::new("apk")
        .arg("add")
        .arg(&name)
        .status()
        .await
    {
        Ok(status) => {
            if status.success() {
                println!("✓ Successfully installed {}", name);
                true
            } else {
                eprintln!("✗ Installation failed for {}", name);
                false
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to run apk: {}", e);
            false
        }
    }
}
