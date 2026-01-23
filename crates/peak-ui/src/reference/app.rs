use super::views::ContentView;
use crate::prelude::*;
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};

pub struct App {
    pub active_tab: String,
    pub show_search: bool,
    pub show_inspector: bool,
    pub show_sidebar: bool,
    pub show_user_profile: bool,
    pub navigation_mode: String,
    pub expanded_sections: std::collections::HashSet<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    SetTab(String),
    ToggleSearch,
    ToggleInspector,
    ToggleSidebar,
    ToggleUserProfile,
    SetNavigationMode(String),
    ToggleSection(String),
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: "Introduction".to_string(),
            show_search: false,
            show_inspector: true,
            show_sidebar: true,
            show_user_profile: false,
            navigation_mode: "Documentation".to_string(),
            expanded_sections: ["Components".to_string()].into_iter().collect(),
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetTab(tab) => {
                self.active_tab = tab;
                Task::none()
            }
            Message::ToggleSearch => {
                self.show_search = !self.show_search;
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
                self.navigation_mode = mode;
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
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let mode = ShellMode::Desktop;
        let tone = ThemeTone::Light;
        let tokens = ThemeTokens::get(mode, tone);

        let content = ContentView::new(self);

        responsive(mode, tokens, move |context| content.view(&context))
    }
}
