use iced::{Alignment, Color, Element, Length, Task};
pub use peak_core::apps::settings::{SettingsApp, SettingsMessage, SettingsTab, ThemeMode};
use peak_core::registry::ShellMode;
use peak_core::theme::Theme;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub trait SettingsDesktopView {
    fn view(
        &self,
        theme: &Theme,
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Element<'static, SettingsMessage, iced::Theme, iced::Renderer>;

    fn view_tab_content(
        &self,
        ctx: &Context,
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Box<dyn View<SettingsMessage>>;

    fn labeled_row(
        &self,
        ctx: &Context,
        label: &str,
        widget: impl View<SettingsMessage> + 'static,
    ) -> Box<dyn View<SettingsMessage>>;

    fn theme_preview(
        &self,
        ctx: &Context,
        label: &str,
        mode: ThemeMode,
    ) -> Box<dyn View<SettingsMessage>>;

    fn mode_preview(
        &self,
        ctx: &Context,
        label: &str,
        mode: ShellMode,
    ) -> Box<dyn View<SettingsMessage>>;

    fn shell_style_preview(
        &self,
        ctx: &Context,
        label: &str,
        style: peak_core::registry::ShellStyle,
    ) -> Box<dyn View<SettingsMessage>>;
}

impl SettingsDesktopView for SettingsApp {
    fn view(
        &self,
        theme: &Theme,
        handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Element<'static, SettingsMessage, iced::Theme, iced::Renderer> {
        let is_light = *theme == Theme::Light;
        let tone = if is_light {
            ThemeTone::Light
        } else {
            ThemeTone::Dark
        };
        let mode = peak_core::registry::ShellMode::Desktop;
        let tokens = ThemeTokens::get(mode, tone);
        let handles_clone = handles.clone();
        let app = self.clone();

        responsive(mode, tokens, move |ctx| {
            let sidebar = Sidebar::new("Settings", tokens)
                .with_search(app.search_query.clone(), SettingsMessage::SearchChanged)
                .item(
                    "Network",
                    "wifi_full",
                    SettingsMessage::TabChanged(SettingsTab::WiFi),
                    app.current_tab == SettingsTab::WiFi,
                )
                .item(
                    "Bluetooth",
                    "wifi_off",
                    SettingsMessage::TabChanged(SettingsTab::Bluetooth),
                    app.current_tab == SettingsTab::Bluetooth,
                )
                .item(
                    "General",
                    "settings",
                    SettingsMessage::TabChanged(SettingsTab::General),
                    app.current_tab == SettingsTab::General,
                )
                .item(
                    "Appearance",
                    "palette",
                    SettingsMessage::TabChanged(SettingsTab::Appearance),
                    app.current_tab == SettingsTab::Appearance,
                )
                .item(
                    "Display",
                    "about",
                    SettingsMessage::TabChanged(SettingsTab::Display),
                    app.current_tab == SettingsTab::Display,
                )
                .item(
                    "Sound",
                    "media",
                    SettingsMessage::TabChanged(SettingsTab::Sound),
                    app.current_tab == SettingsTab::Sound,
                )
                .item(
                    "Focus",
                    "moon",
                    SettingsMessage::TabChanged(SettingsTab::Focus),
                    app.current_tab == SettingsTab::Focus,
                )
                .item(
                    "Intelligence",
                    "sparkles",
                    SettingsMessage::TabChanged(SettingsTab::Intelligence),
                    app.current_tab == SettingsTab::Intelligence,
                )
                .item(
                    "Modes",
                    "trigger",
                    SettingsMessage::TabChanged(SettingsTab::Modes),
                    app.current_tab == SettingsTab::Modes,
                );

            let tab_title = Text::new(format!("{:?}", app.current_tab)).title2();

            // FIX: Add .height(Length::Shrink) to VStack and ensure NO COMMA before the dot
            let content = ScrollView::new(
                VStack::new()
                    .spacing(24.0)
                    .push(tab_title)
                    .push(app.view_tab_content(&ctx, &handles_clone)) // <--- No comma here!
                    .height(Length::Shrink), // <--- This prevents the panic
            );

            NavigationSplitView::new(sidebar, content)
                .on_back(SettingsMessage::TabChanged(SettingsTab::General))
                .view(&ctx)
        })
    }

    fn view_tab_content(
        &self,
        ctx: &Context,
        _handles: &std::collections::HashMap<String, iced::widget::image::Handle>,
    ) -> Box<dyn View<SettingsMessage>> {
        match self.current_tab {
            SettingsTab::General => VStack::new()
                .spacing(24.0)
                .push(Section::new(
                    "About",
                    Card::new(
                        VStack::new()
                            .push(self.labeled_row(
                                ctx,
                                "Name",
                                Text::new("PeakOS Device").secondary(),
                            ))
                            .push(Divider::new())
                            .push(self.labeled_row(
                                ctx,
                                "Model",
                                Text::new("Peak Native (Apple Silicon)").secondary(),
                            ))
                            .push(Divider::new())
                            .push(self.labeled_row(
                                ctx,
                                "Version",
                                Text::new("1.0.0 (Alpha)").secondary(),
                            )),
                    ),
                ))
                .push(Section::new(
                    "Software Update",
                    Card::new(self.labeled_row(
                        ctx,
                        "Automatic Updates",
                        Text::new("On").secondary(),
                    )),
                ))
                .into_box(),

            SettingsTab::Appearance => {
                let mut grid = ResponsiveGrid::new().spacing(12.0);

                for wp in &self.wallpapers {
                    let wp_clone = wp.clone();
                    let image_path =
                        peak_core::utils::assets::get_asset_path(&format!("wallpapers/{}", wp));

                    grid = grid.push(
                        Button::new(
                            Image::new(&image_path)
                                .width(120.0.into())
                                .height(75.0.into())
                                .corner_radius(8.0),
                        )
                        .on_press(SettingsMessage::WallpaperChanged(wp_clone)),
                    );
                }

                VStack::new()
                    .spacing(24.0)
                    .push(Section::new(
                        "Theme",
                        Card::new(
                            VStack::new()
                                .push(
                                    self.labeled_row(
                                        ctx,
                                        "Appearance",
                                        HStack::new()
                                            .spacing(12.0)
                                            .push(self.theme_preview(
                                                ctx,
                                                "Light",
                                                ThemeMode::Light,
                                            ))
                                            .push(self.theme_preview(ctx, "Dark", ThemeMode::Dark)),
                                    ),
                                )
                                .push(Divider::new())
                                .push(self.labeled_row(
                                    ctx,
                                    "Dynamic Theme",
                                    Toggle::new("Auto", true, |b| SettingsMessage::ToggleWiFi(b)),
                                )),
                        ),
                    ))
                    .push(Section::new("Wallpaper", grid))
                    .into_box()
            }

            SettingsTab::Display => {
                let mut grid = ResponsiveGrid::new().spacing(12.0);
                for s in vec![
                    peak_core::registry::ShellStyle::Cupertino,
                    peak_core::registry::ShellStyle::Redmond,
                    peak_core::registry::ShellStyle::AI,
                    peak_core::registry::ShellStyle::Console,
                    peak_core::registry::ShellStyle::TV,
                ] {
                    grid = grid.push(self.shell_style_preview(ctx, &format!("{:?}", s), s));
                }

                VStack::new()
                    .spacing(24.0)
                    .push(Section::new("Shell Style", Card::new(grid)))
                    .push(Section::new(
                        "Night Shift",
                        Card::new(self.labeled_row(
                            ctx,
                            "Scheduled",
                            Toggle::new("Active", false, |b| SettingsMessage::ToggleWiFi(b)),
                        )),
                    ))
                    .into_box()
            }

            SettingsTab::WiFi => VStack::new()
                .spacing(24.0)
                .push(Section::new(
                    "Wi-Fi",
                    Card::new(self.labeled_row(
                        ctx,
                        "Wi-Fi",
                        Toggle::new("Enable", self.wifi_enabled, |b| {
                            SettingsMessage::ToggleWiFi(b)
                        }),
                    )),
                ))
                .push(if self.wifi_enabled {
                    Section::new(
                        "Known Networks",
                        Card::new(
                            VStack::new()
                                .push(self.labeled_row(
                                    ctx,
                                    "Home WiFi",
                                    Text::new("Connected").intent(Intent::Success),
                                ))
                                .push(Divider::new())
                                .push(self.labeled_row(
                                    ctx,
                                    "Office",
                                    Text::new("Saved").secondary(),
                                )),
                        ),
                    )
                } else {
                    Section::new("Information", Text::new("Wi-Fi is off").secondary())
                })
                .into_box(),

            SettingsTab::Bluetooth => VStack::new()
                .spacing(24.0)
                .push(Section::new(
                    "Bluetooth",
                    Card::new(self.labeled_row(
                        ctx,
                        "Bluetooth",
                        Toggle::new("Enable", self.bluetooth_enabled, |b| {
                            SettingsMessage::ToggleBluetooth(b)
                        }),
                    )),
                ))
                .push(if self.bluetooth_enabled {
                    Section::new(
                        "My Devices",
                        Card::new(
                            VStack::new()
                                .push(self.labeled_row(
                                    ctx,
                                    "AirPods Pro",
                                    Text::new("Not Connected").secondary(),
                                ))
                                .push(Divider::new())
                                .push(self.labeled_row(
                                    ctx,
                                    "Magic Keyboard",
                                    Text::new("Connected").intent(Intent::Success),
                                )),
                        ),
                    )
                } else {
                    Section::new("Information", Text::new("Bluetooth is off").secondary())
                })
                .into_box(),

            SettingsTab::Sound => VStack::new()
                .spacing(24.0)
                .push(Section::new(
                    "Output & Input",
                    Card::new(self.labeled_row(
                        ctx,
                        "Output Volume",
                        Slider::new(0.0..=1.0, self.volume, |v| {
                            SettingsMessage::VolumeChanged(v)
                        }),
                    )),
                ))
                .into_box(),

            SettingsTab::Intelligence => {
                let active_model = self
                    .recommended_models
                    .iter()
                    .chain(self.custom_models.iter())
                    .find(|m| m.is_active);

                VStack::new()
                    .spacing(24.0)
                    .push(Section::new(
                        "Settings",
                        Card::new(
                            VStack::new()
                                .push(
                                    self.labeled_row(
                                        ctx,
                                        "Status",
                                        match active_model {
                                            Some(m) => Text::new(format!("Active ({})", m.name))
                                                .intent(Intent::Success),
                                            None => Text::new("No Active Model").secondary(),
                                        },
                                    ),
                                )
                                .push(Divider::new())
                                .push(self.labeled_row(
                                    ctx,
                                    "Captions",
                                    Toggle::new("Enable", self.captions_enabled, |b| {
                                        SettingsMessage::ToggleCaptions(b)
                                    }),
                                ))
                                .push(Divider::new())
                                .push(self.labeled_row(
                                    ctx,
                                    "Voice",
                                    Toggle::new("Enable", self.voice_enabled, |b| {
                                        SettingsMessage::ToggleVoice(b)
                                    }),
                                )),
                        ),
                    ))
                    .push(Section::new(
                        "Models",
                        VStack::new().spacing(12.0).push(
                            VStack::new()
                                .push(Text::new("Llama 3 8B").size(13.0))
                                .push(Text::new("Mistral 7B").size(13.0)),
                        ),
                    ))
                    .into_box()
            }

            SettingsTab::Modes => {
                let mut grid = ResponsiveGrid::new().spacing(12.0);
                for m in vec![
                    ShellMode::Desktop,
                    ShellMode::Mobile,
                    ShellMode::TV,
                    ShellMode::Console,
                    ShellMode::Kiosk,
                    ShellMode::Fireplace,
                    ShellMode::Auto,
                    ShellMode::Robot,
                    ShellMode::Server,
                    ShellMode::SmartHome,
                ] {
                    grid = grid.push(self.mode_preview(ctx, &format!("{:?}", m), m));
                }

                VStack::new()
                    .spacing(24.0)
                    .push(Section::new("OS Modes", Card::new(grid)))
                    .into_box()
            }

            _ => Text::new("Placeholder for future settings.")
                .secondary()
                .into_box(),
        }
    }

    fn labeled_row(
        &self,
        _ctx: &Context,
        label: &str,
        widget: impl View<SettingsMessage> + 'static,
    ) -> Box<dyn View<SettingsMessage>> {
        HStack::new()
            .align_y(Alignment::Center)
            .push(Text::new(label).size(13.0))
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(widget)
            .into_box()
    }

    fn theme_preview(
        &self,
        _ctx: &Context,
        label: &str,
        mode: ThemeMode,
    ) -> Box<dyn View<SettingsMessage>> {
        let is_selected = self.theme_mode == mode;
        let is_light = mode == ThemeMode::Light;

        Button::new(
            VStack::new()
                .spacing(8.0)
                .push(
                    Rectangle::new(80.0.into(), 50.0.into())
                        .color(if is_light {
                            Color::WHITE
                        } else {
                            Color::from_rgb8(28, 28, 30)
                        })
                        .corner_radius(4.0)
                        .border(
                            if is_selected { 2.0 } else { 1.0 },
                            if is_selected {
                                Color::from_rgb8(0, 122, 255)
                            } else {
                                Color::from_rgba(0.0, 0.0, 0.0, 0.1)
                            },
                        ),
                )
                .push(Text::new(label).size(12.0).center()),
        )
        .on_press(SettingsMessage::ThemeChanged(mode))
        .into_box()
    }

    fn mode_preview(
        &self,
        ctx: &Context,
        label: &str,
        mode: ShellMode,
    ) -> Box<dyn View<SettingsMessage>> {
        let is_selected = self.current_mode == mode;
        let tokens = ctx.theme;

        let button_bg = tokens.colors.primary;

        Button::new(
            VStack::new()
                .spacing(8.0)
                .push(
                    ZStack::new()
                        .push(
                            Rectangle::new(80.0.into(), 50.0.into())
                                .color(button_bg)
                                .corner_radius(4.0)
                                .border(
                                    if is_selected { 2.0 } else { 0.0 },
                                    if is_selected {
                                        Color::from_rgb8(0, 122, 255)
                                    } else {
                                        Color::TRANSPARENT
                                    },
                                ),
                        )
                        .push(
                            Icon::new(peak_core::icons::get_mode_icon_name(mode))
                                .size(24.0)
                                .color(Color::WHITE),
                        ),
                )
                .push(Text::new(label).size(11.0).center()),
        )
        .on_press(SettingsMessage::ModeChanged(mode))
        .into_box()
    }

    fn shell_style_preview(
        &self,
        _ctx: &Context,
        label: &str,
        style: peak_core::registry::ShellStyle,
    ) -> Box<dyn View<SettingsMessage>> {
        let is_selected = self.current_shell_style == style;

        Button::new(
            VStack::new()
                .spacing(8.0)
                .push(
                    ZStack::new()
                        .push(
                            Rectangle::new(80.0.into(), 50.0.into())
                                .color(Color::from_rgba(0.5, 0.5, 0.5, 0.1))
                                .corner_radius(4.0)
                                .border(
                                    if is_selected { 2.0 } else { 0.0 },
                                    if is_selected {
                                        Color::from_rgb8(0, 122, 255)
                                    } else {
                                        Color::TRANSPARENT
                                    },
                                ),
                        )
                        .push(
                            Text::new(match style {
                                peak_core::registry::ShellStyle::Cupertino => "◈",
                                peak_core::registry::ShellStyle::Redmond => "⊞",
                                peak_core::registry::ShellStyle::AI => "✧",
                                peak_core::registry::ShellStyle::Console => "🎮",
                                peak_core::registry::ShellStyle::TV => "📺",
                            })
                            .size(20.0)
                            .center(),
                        ),
                )
                .push(Text::new(label).size(11.0).center()),
        )
        .on_press(SettingsMessage::ShellStyleChanged(style))
        .into_box()
    }
}
// Wrapper for Registry
pub struct DesktopSettingsApp {
    pub app: SettingsApp,
    pub handles: std::collections::HashMap<String, iced::widget::image::Handle>,
}

impl Default for DesktopSettingsApp {
    fn default() -> Self {
        Self::new()
    }
}

impl DesktopSettingsApp {
    pub fn new() -> Self {
        // Dynamically scan wallpapers directory
        #[cfg(not(target_arch = "wasm32"))]
        let wallpapers_dir = peak_core::utils::assets::get_asset_path("wallpapers");

        #[cfg(not(target_arch = "wasm32"))]
        let wallpapers: Vec<String> = std::fs::read_dir(&wallpapers_dir)
            .ok()
            .map(|entries| {
                let mut list: Vec<String> = entries
                    .filter_map(|e| e.ok())
                    .filter_map(|e| {
                        let path = e.path();
                        if path.is_file() {
                            let ext = path.extension()?.to_str()?.to_lowercase();
                            if ["jpg", "jpeg", "png", "webp"].contains(&ext.as_str()) {
                                return Some(e.file_name().to_string_lossy().to_string());
                            }
                        }
                        None
                    })
                    .collect();
                list.sort();
                list
            })
            .unwrap_or_else(|| {
                // Fallback to known wallpapers if dir scan fails
                vec![
                    "mountain_classic.jpg".to_string(),
                    "mountain_classic_light.jpg".to_string(),
                ]
            });

        #[cfg(target_arch = "wasm32")]
        let wallpapers: Vec<String> = vec![
            "mountain_classic.jpg".to_string(),
            "mountain_classic_light.jpg".to_string(),
        ];

        // Pre-load handles
        let mut handles = std::collections::HashMap::new();
        #[cfg(not(target_arch = "wasm32"))]
        for wp in &wallpapers {
            let path = wallpapers_dir.join(wp);
            handles.insert(wp.clone(), iced::widget::image::Handle::from_path(path));
        }

        #[cfg(target_arch = "wasm32")]
        let _ = handles; // Avoid unused warning if truly empty or just remove mut if possible

        let mut app = SettingsApp::new();
        app.wallpapers = wallpapers;
        Self { app, handles }
    }
}

use peak_core::app_traits::{PeakApp, ShellContext};

impl PeakApp for DesktopSettingsApp {
    type Message = SettingsMessage;

    fn title(&self) -> String {
        self.app.title()
    }

    fn update(
        &mut self,
        message: Self::Message,
        context: &dyn ShellContext,
    ) -> Task<Self::Message> {
        self.app.update(message, context)
    }

    fn view(&self, theme: &Theme) -> Element<'static, Self::Message> {
        SettingsDesktopView::view(&self.app, theme, &self.handles)
    }
}
