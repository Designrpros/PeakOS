use iced::widget::{button, column, container, row, text, text_editor};
use iced::Element;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    ContentChanged(iced::widget::text_editor::Action),
    Save,
    #[allow(dead_code)]
    Close,
}

pub struct EditorApp {
    pub path: Option<PathBuf>,
    pub content: text_editor::Content,
    pub is_dirty: bool,
}

impl EditorApp {
    pub fn new() -> Self {
        Self {
            path: None,
            content: text_editor::Content::new(),
            is_dirty: false,
        }
    }

    pub fn open(path: PathBuf) -> Self {
        let content_str = std::fs::read_to_string(&path).unwrap_or_default();
        Self {
            path: Some(path),
            content: text_editor::Content::with_text(&content_str),
            is_dirty: false,
        }
    }

    pub fn view(&self, _is_light: bool) -> Element<'_, EditorMessage> {
        let path_str = self
            .path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or("Untitled".to_string());
        let toolbar: Element<EditorMessage> = row![
            button(text("Save").size(12)).on_press(EditorMessage::Save),
            text(path_str).size(12),
        ]
        .spacing(10)
        .align_y(iced::Alignment::Center)
        .into();

        let content: Element<EditorMessage> = text_editor(&self.content)
            .on_action(EditorMessage::ContentChanged)
            .into();

        container(column![toolbar, content,].spacing(10))
            .padding(10)
            .into()
    }

    pub fn update(&mut self, message: EditorMessage) {
        match message {
            EditorMessage::ContentChanged(action) => {
                self.content.perform(action);
                self.is_dirty = true;
            }
            EditorMessage::Save => {
                if let Some(path) = &self.path {
                    let text = self.content.text();
                    std::fs::write(path, text).ok();
                    self.is_dirty = false;
                }
            }
            EditorMessage::Close => {
                // Handled by main app
            }
        }
    }
}
