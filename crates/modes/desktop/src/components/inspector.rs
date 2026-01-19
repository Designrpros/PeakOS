use iced::widget::{button, column, container, pick_list, row, scrollable, text, text_input};
use iced::{Element, Length};
use peak_theme::ThemeTokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InspectorViewState {
    Overview,
}

use iced::Task;

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum InspectorMessage {
    ToggleVoice,
    SwitchView(InspectorViewState),
    MouseReleased,
    SetActiveModel(String),
    SelectModel(String),
    SyncAvailableModels(Vec<String>),
    InputChanged(String),
    SubmitMessage,
    SetVoiceEnabled(bool),
}

pub struct Inspector {
    pub is_visible: bool,
    pub view_state: InspectorViewState,
    pub input_content: String,
    pub chat_history: Vec<(String, String)>, // (Role, Content)
    pub active_model: Option<String>,
    pub available_models: Vec<String>,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            view_state: InspectorViewState::Overview,
            input_content: String::new(),
            chat_history: vec![(
                "system".to_string(),
                "Inspector initialized. Ready for query.".to_string(),
            )],
            active_model: None,
            available_models: vec![],
        }
    }

    pub fn update(&mut self, message: InspectorMessage) -> Task<InspectorMessage> {
        match message {
            InspectorMessage::ToggleVoice => {}
            InspectorMessage::SwitchView(state) => {
                self.view_state = state;
            }
            InspectorMessage::MouseReleased => {}
            InspectorMessage::SetActiveModel(id) => {
                self.active_model = Some(id);
            }
            InspectorMessage::SelectModel(id) => {
                self.active_model = Some(id);
            }
            InspectorMessage::SyncAvailableModels(mut models) => {
                // Ensure active model is in the list
                if let Some(ref active) = self.active_model {
                    if !models.contains(active) {
                        models.insert(0, active.clone());
                    }
                }
                self.available_models = models;
            }
            InspectorMessage::SetVoiceEnabled(_) => {}
            InspectorMessage::InputChanged(content) => {
                self.input_content = content;
            }
            InspectorMessage::SubmitMessage => {
                if !self.input_content.trim().is_empty() {
                    self.chat_history
                        .push(("user".to_string(), self.input_content.clone()));
                    self.input_content.clear();
                    // Auto-scroll to bottom
                    return iced::widget::scrollable::snap_to(
                        iced::widget::scrollable::Id::new("chat_scroll"),
                        iced::widget::scrollable::RelativeOffset::END,
                    );
                }
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> iced::Subscription<InspectorMessage> {
        iced::Subscription::none()
    }

    pub fn view(&self, tokens: ThemeTokens) -> Element<'_, InspectorMessage> {
        // Directly show chat view (removed segmented control for now)
        let chat_scroll: Element<'_, InspectorMessage> = scrollable(
            column(
                self.chat_history
                    .iter()
                    .map(|(role, msg)| {
                        let is_user = role == "user";
                        let bubble = container(text(msg.clone()).style(move |_| text::Style {
                            color: Some(tokens.text),
                        }))
                        .padding(12)
                        .style(move |_| container::Style {
                            background: Some(if is_user {
                                iced::Color::from_rgba(0.5, 0.5, 0.5, 0.2).into()
                            } else {
                                iced::Color::from_rgba(0.5, 0.5, 0.5, 0.05).into()
                            }),
                            border: iced::Border {
                                radius: 12.0.into(),
                                ..Default::default()
                            },
                            text_color: None,
                            shadow: iced::Shadow::default(),
                        })
                        .width(Length::Shrink);

                        // Align user messages to the right, assistant to the left
                        if is_user {
                            row![iced::widget::horizontal_space(), bubble].into()
                        } else {
                            row![bubble, iced::widget::horizontal_space()].into()
                        }
                    })
                    .collect::<Vec<_>>(),
            )
            .spacing(8)
            .width(Length::Fill),
        )
        .height(Length::Fill)
        .direction(iced::widget::scrollable::Direction::Vertical(
            iced::widget::scrollable::Scrollbar::new()
                .width(0)
                .scroller_width(0),
        ))
        .id(iced::widget::scrollable::Id::new("chat_scroll"))
        .into();

        // Input area with border encompassing everything
        let model_selector: Element<'_, InspectorMessage> = {
            if self.available_models.is_empty() && self.active_model.is_none() {
                // Show placeholder text when no models available
                container(text("No model").size(11).style(move |_theme| text::Style {
                    color: Some(iced::Color {
                        a: 0.5,
                        ..tokens.text
                    }),
                    ..Default::default()
                }))
                .padding([0, 8])
                .into()
            } else {
                container(
                    pick_list(
                        &self.available_models[..],
                        self.active_model.as_ref(),
                        InspectorMessage::SelectModel,
                    )
                    .text_size(11)
                    .padding(6)
                    .placeholder("Select model")
                    .style(
                        move |_theme: &iced::Theme, _status| iced::widget::pick_list::Style {
                            text_color: tokens.text,
                            placeholder_color: iced::Color {
                                a: 0.5,
                                ..tokens.text
                            },
                            handle_color: tokens.text,
                            background: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15).into(),
                            border: iced::Border {
                                radius: 8.0.into(),
                                ..Default::default()
                            },
                        },
                    ),
                )
                .into()
            }
        };

        let input_area: Element<'_, InspectorMessage> = container(
            column![
                text_input("Ask Peak...", &self.input_content)
                    .on_input(InspectorMessage::InputChanged)
                    .on_submit(InspectorMessage::SubmitMessage)
                    .padding(10)
                    .style(move |_theme, _status| iced::widget::text_input::Style {
                        background: iced::Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                        icon: tokens.text,
                        placeholder: iced::Color {
                            a: 0.5,
                            ..tokens.text
                        },
                        value: tokens.text,
                        selection: iced::Color::from_rgba(0.5, 0.5, 0.5, 0.3),
                    }),
                row![
                    model_selector,
                    iced::widget::horizontal_space(),
                    button(
                        iced::widget::svg(peak_core::icons::get_ui_icon(
                            "arrow_up",
                            &format!(
                                "#{:02X}{:02X}{:02X}",
                                (tokens.text.r * 255.0) as u8,
                                (tokens.text.g * 255.0) as u8,
                                (tokens.text.b * 255.0) as u8
                            )
                        ))
                        .width(16)
                        .height(16)
                    )
                    .on_press(InspectorMessage::SubmitMessage)
                    .padding(8)
                    .style(move |_theme, _status| {
                        iced::widget::button::Style {
                            background: Some(iced::Color::from_rgb(0.2, 0.2, 0.2).into()),
                            border: iced::Border {
                                radius: 100.0.into(),
                                ..Default::default()
                            },
                            text_color: iced::Color::WHITE,
                            shadow: iced::Shadow::default(),
                        }
                    })
                ]
                .spacing(8)
            ]
            .spacing(8),
        )
        .padding(4)
        .style(move |_| container::Style {
            border: iced::Border {
                width: 1.0,
                color: iced::Color {
                    a: 0.1,
                    ..tokens.text
                },
                radius: 12.0.into(),
            },
            ..Default::default()
        })
        .into();

        let content = column![chat_scroll, input_area].spacing(16);

        container(content)
            .padding(20)
            .width(Length::Fixed(340.0))
            .height(Length::Fill)
            .style(move |_: &iced::Theme| container::Style {
                background: Some(tokens.glass_bg.into()),
                border: iced::Border {
                    radius: 16.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .into()
    }
}
