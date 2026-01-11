#![allow(dead_code)]
use crate::app::Message;
use crate::theme::Theme;
use iced::{
    widget::{container, text},
    Element, Theme as IcedTheme,
};

pub fn view<'a>(theme: &Theme) -> Element<'a, Message, IcedTheme> {
    container(text(format!("Settings Placeholder. Current Theme: {:?}", theme)).size(30))
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .into()
}
