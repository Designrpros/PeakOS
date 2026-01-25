use super::model::Page;
use crate::prelude::*;
use peak_core::registry::ShellMode;
use peak_theme::{PeakTheme, ThemeTokens, ThemeTone};

pub struct App {
    pub active_tab: Page,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub search_query: String,
    pub expanded_sections: std::collections::HashSet<String>, // Note: typo in original file logic or just keep it? It was `expanded_sections`.
    pub theme_tone: ThemeTone,
    pub theme: PeakTheme,
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
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTab(tab) => {
                self.active_tab = tab;
                self.show_search = false;
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
}
