use super::model::Page;
use crate::prelude::*;
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderMode {
    #[default]
    Canvas,
    Terminal,
    Neural,
    Spatial,
}

pub struct App {
    pub active_tab: Page,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub search_query: String,
    pub expanded_sections: std::collections::HashSet<String>,
    pub theme_tone: ThemeTone,
    pub theme: PeakTheme,

    // Component Lab States
    pub button_lab: ButtonLabState,
    pub render_mode: RenderMode,
    // Layout States
    pub sidebar_width: f32,
    pub inspector_width: f32,
    pub is_resizing_sidebar: bool,
    pub is_resizing_inspector: bool,
}

#[derive(Debug, Clone)]
pub struct ButtonLabState {
    pub label: String,
    pub icon: Option<String>,
    pub variant: Variant,
    pub intent: Intent,
    pub size: ControlSize,
    pub is_full_width: bool,
    pub is_disabled: bool,
    pub is_focused: bool,
}

impl Default for ButtonLabState {
    fn default() -> Self {
        Self {
            label: "Click Me".to_string(),
            icon: None,
            variant: Variant::Solid,
            intent: Intent::Primary,
            size: ControlSize::Medium,
            is_full_width: false,
            is_disabled: false,
            is_focused: false,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTab(Page),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
    Search(String),
    SetTheme(ThemeTone),
    SetThemeKind(PeakTheme),
    CopyCode(String),
    SetRenderMode(RenderMode),

    // Button Lab Messages
    UpdateButtonLabel(String),
    UpdateButtonIcon(Option<String>),
    UpdateButtonSize(ControlSize),
    UpdateButtonVariant(Variant),
    UpdateButtonIntent(Intent),
    ToggleButtonFullWidth(bool),
    ToggleButtonDisabled(bool),
    ToggleButtonFocused(bool),
    ResizeSidebar(f32),
    ResizeInspector(f32),
    StartResizingSidebar,
    StopResizingSidebar,
    StartResizingInspector,
    StopResizingInspector,
    FontLoaded(std::result::Result<(), iced::font::Error>),
    None,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: Page::Introduction,
            show_search: false,
            show_inspector: false,
            show_sidebar: true,
            show_user_profile: false,
            navigation_mode: "Start".to_string(),
            search_query: "".to_string(),
            expanded_sections: ["COMPONENTS".to_string()].into_iter().collect(),
            theme_tone: ThemeTone::Light,
            theme: PeakTheme::Peak,
            button_lab: ButtonLabState::default(),
            render_mode: RenderMode::Canvas,
            sidebar_width: 240.0,
            inspector_width: 300.0,
            is_resizing_sidebar: false,
            is_resizing_inspector: false,
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTab(tab) => {
                log::debug!(
                    "Setting Tab: {:?} (Category: {})",
                    tab,
                    tab.navigation_mode()
                );
                self.navigation_mode = tab.navigation_mode();
                self.active_tab = tab.clone();
                self.show_search = false;

                #[cfg(target_arch = "wasm32")]
                {
                    let path = tab.to_path();
                    let _ = web_sys::window().and_then(|w| w.location().set_hash(&path).ok());
                }

                Task::none()
            }
            Message::ToggleSearch => {
                self.show_search = !self.show_search;
                if !self.show_search {
                    self.search_query.clear();
                }
                Task::none()
            }
            Message::ToggleInspector => {
                self.show_inspector = !self.show_inspector;
                Task::none()
            }
            Message::ToggleSidebar => {
                self.show_sidebar = !self.show_sidebar;
                Task::none()
            }
            Message::ToggleUserProfile => {
                self.show_user_profile = !self.show_user_profile;
                Task::none()
            }
            Message::SetNavigationMode(mode) => {
                self.navigation_mode = mode.clone();
                self.active_tab = match mode.as_str() {
                    "Start" => Page::Introduction,
                    "Catalog" => Page::Button,
                    "Data" => Page::PeakDB,
                    "Settings" => Page::Appearance,
                    _ => self.active_tab.clone(),
                };
                Task::none()
            }
            Message::ToggleSection(section) => {
                if self.expanded_sections.contains(&section) {
                    self.expanded_sections.remove(&section);
                } else {
                    self.expanded_sections.insert(section);
                }
                Task::none()
            }
            Message::Search(query) => {
                self.search_query = query;
                Task::none()
            }
            Message::SetTheme(tone) => {
                self.theme_tone = tone;
                Task::none()
            }
            Message::SetThemeKind(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::CopyCode(code) => iced::clipboard::write(code),
            Message::SetRenderMode(mode) => {
                self.render_mode = mode;
                Task::none()
            }

            // Button Lab Handlers
            Message::UpdateButtonLabel(label) => {
                self.button_lab.label = label;
                Task::none()
            }
            Message::UpdateButtonIcon(icon) => {
                self.button_lab.icon = icon;
                Task::none()
            }
            Message::UpdateButtonVariant(variant) => {
                self.button_lab.variant = variant;
                Task::none()
            }
            Message::UpdateButtonIntent(intent) => {
                self.button_lab.intent = intent;
                Task::none()
            }
            Message::UpdateButtonSize(size) => {
                self.button_lab.size = size;
                Task::none()
            }
            Message::ToggleButtonFullWidth(full_width) => {
                self.button_lab.is_full_width = full_width;
                Task::none()
            }
            Message::ToggleButtonDisabled(disabled) => {
                self.button_lab.is_disabled = disabled;
                Task::none()
            }
            Message::ToggleButtonFocused(focused) => {
                self.button_lab.is_focused = focused;
                Task::none()
            }
            Message::ResizeSidebar(width) => {
                self.sidebar_width = width.max(160.0).min(400.0);
                Task::none()
            }
            Message::ResizeInspector(width) => {
                self.inspector_width = width.max(180.0).min(600.0);
                Task::none()
            }
            Message::StartResizingSidebar => {
                self.is_resizing_sidebar = true;
                Task::none()
            }
            Message::StopResizingSidebar => {
                self.is_resizing_sidebar = false;
                Task::none()
            }
            Message::StartResizingInspector => {
                self.is_resizing_inspector = true;
                Task::none()
            }
            Message::StopResizingInspector => {
                self.is_resizing_inspector = false;
                Task::none()
            }
            Message::FontLoaded(_) => Task::none(),
            Message::None => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mode = ShellMode::Desktop;
        let tone = self.theme_tone;
        let tokens = ThemeTokens::with_theme(self.theme, tone);

        // 1. Prepare Content
        let content = super::views::ContentView::new(self);

        crate::core::responsive(mode, tokens, move |context| {
            // Main App Content - ContentView handles splitting, inspector, and layers
            iced::widget::container(content.view(&context))
                .width(Length::Fill)
                .height(Length::Fill)
                .style(move |_| iced::widget::container::Style {
                    background: Some(tokens.colors.background.into()),
                    ..Default::default()
                })
                .into()
        })
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;

            iced::Subscription::run(|| {
                let (mut sender, receiver) = iced::futures::channel::mpsc::channel(1);
                let window = web_sys::window().expect("window not found");

                let on_hash_change = wasm_bindgen::prelude::Closure::wrap(Box::new(move || {
                    let hash = web_sys::window()
                        .and_then(|w| w.location().hash().ok())
                        .unwrap_or_default();

                    let path = if hash.starts_with('#') {
                        &hash[1..]
                    } else {
                        &hash
                    };

                    let page = Page::from_path(path);
                    let _ = sender.try_send(Message::SetTab(page));
                })
                    as Box<dyn FnMut()>);

                window.set_onhashchange(Some(on_hash_change.as_ref().unchecked_ref()));
                on_hash_change.forget(); // Keep closure alive

                receiver
            })
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            iced::Subscription::none()
        }
    }
}
