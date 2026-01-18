use iced::widget::{button, column, container, row, scrollable, text, text_editor, vertical_space};
use iced::{Alignment, Color, Element, Length, Task};
use peak_intelligence::llm::{LlmClient, Message, ModelProvider};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum InspectorMessage {
    InputChanged(text_editor::Action),
    SendPressed,
    ResponseReceived(Result<String, String>),
    #[allow(dead_code)]
    OpenSettings,
    MouseReleased, // Forward mouse releases to parent
}

#[derive(Debug, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

pub struct Inspector {
    pub is_visible: bool,
    input_content: text_editor::Content,
    messages: Vec<ChatMessage>,
    is_loading: bool,
    #[allow(dead_code)]
    provider: ModelProvider,
    client: Arc<LlmClient>,
}

impl Inspector {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            input_content: text_editor::Content::new(),
            messages: vec![ChatMessage {
                role: "system".to_string(),
                content:
                    "Hello! I am Peak Intelligence. How can I help you manage your system today?"
                        .to_string(),
            }],
            is_loading: false,
            provider: ModelProvider::Ollama, // Default to local
            client: Arc::new(LlmClient::new(
                ModelProvider::Ollama,
                "llama3".to_string(), // Default model
                None,
            )),
        }
    }

    pub fn update(&mut self, message: InspectorMessage) -> Task<InspectorMessage> {
        match message {
            InspectorMessage::InputChanged(action) => {
                self.input_content.perform(action);
                Task::none()
            }
            InspectorMessage::OpenSettings => Task::none(), // Handled in parent
            InspectorMessage::MouseReleased => Task::none(), // Handled in parent
            InspectorMessage::SendPressed => {
                let text = self.input_content.text();
                if text.trim().is_empty() || self.is_loading {
                    return Task::none();
                }

                let user_msg = ChatMessage {
                    role: "user".to_string(),
                    content: text.clone(),
                };
                self.messages.push(user_msg.clone());
                self.input_content = text_editor::Content::new(); // Clear input
                self.is_loading = true;

                let history: Vec<Message> = self
                    .messages
                    .iter()
                    .map(|m| Message {
                        role: m.role.clone(),
                        content: m.content.clone(),
                    })
                    .collect();

                let client = self.client.clone();
                Task::perform(
                    async move { client.chat(history).await },
                    InspectorMessage::ResponseReceived,
                )
            }
            InspectorMessage::ResponseReceived(result) => {
                self.is_loading = false;
                match result {
                    Ok(content) => {
                        self.messages.push(ChatMessage {
                            role: "assistant".to_string(),
                            content,
                        });
                    }
                    Err(e) => {
                        self.messages.push(ChatMessage {
                            role: "system".to_string(),
                            content: format!("Error: {}", e),
                        });
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, tokens: peak_theme::ThemeTokens) -> Element<'a, InspectorMessage> {
        if !self.is_visible {
            return container(vertical_space().height(0))
                .width(0)
                .height(0)
                .into();
        }

        let bg_color = tokens.glass_bg;
        let text_color = tokens.text;

        let header = container(text("Peak Intelligence").size(14).font(iced::Font {
            weight: iced::font::Weight::Bold,
            ..Default::default()
        }))
        .padding(15)
        .align_y(iced::alignment::Vertical::Center);

        // Chat List
        let chat_list = scrollable(
            column(
                self.messages
                    .iter()
                    .map(|msg| {
                        let is_user = msg.role == "user";
                        if is_user {
                            // User Bubble: Card-like or Subtle bg
                            let bubble_bg = tokens.card_bg;

                            container(text(&msg.content).size(14).color(text_color))
                                .padding(12)
                                .style(move |_| container::Style {
                                    background: Some(bubble_bg.into()),
                                    border: iced::Border {
                                        radius: 16.0.into(),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .max_width(280)
                                .align_x(Alignment::End)
                                .into()
                        } else {
                            // AI Answer: Full width, no border/background (minimal)
                            container(
                                text(&msg.content)
                                    .size(14)
                                    .color(text_color)
                                    .line_height(1.6),
                            )
                            .padding(12)
                            .width(Length::Fill)
                            .style(|_| container::Style::default())
                            .align_x(Alignment::Start)
                            .into()
                        }
                    })
                    .collect::<Vec<Element<'_, InspectorMessage>>>(),
            )
            .spacing(20)
            .padding(15),
        )
        .height(Length::Fill);

        // Icons for toolbar
        let hex_color = format!(
            "#{:02x}{:02x}{:02x}",
            (tokens.text.r * 255.0) as u8,
            (tokens.text.g * 255.0) as u8,
            (tokens.text.b * 255.0) as u8
        );
        let mic_icon = peak_core::icons::get_ui_icon("microphone", &hex_color);
        let arrow_icon = peak_core::icons::get_ui_icon("arrow_up", "#FFFFFF"); // Action icon stays white

        let refined_input_area = container(column![
            // Text Editor acts as the main input area
            container(
                text_editor(&self.input_content)
                    .placeholder("Ask anything (⌘L), @ to mention, / for workflows")
                    .on_action(InspectorMessage::InputChanged)
                    .padding(0)
                    .style(move |_, _| text_editor::Style {
                        background: Color::TRANSPARENT.into(),
                        border: iced::Border::default(),
                        value: text_color,
                        selection: tokens.accent,
                        placeholder: Color::from_rgb(0.5, 0.5, 0.5),
                        icon: Color::from_rgb(0.5, 0.5, 0.5),
                    })
            )
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: 0.0,
                left: 0.0
            }),
            vertical_space().height(8),
            row![
                // Plus Icon
                text("+").size(20).color(Color::from_rgb(0.6, 0.6, 0.6)),
                // Breadcrumbs / Status
                text("Planning")
                    .size(12)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                text("Gemini 3 Pro (High)")
                    .size(12)
                    .color(Color::from_rgb(0.6, 0.6, 0.6)),
                iced::widget::horizontal_space(),
                // Mic
                container(
                    iced::widget::svg(mic_icon)
                        .width(Length::Fixed(16.0))
                        .height(Length::Fixed(16.0))
                )
                .padding(4),
                // Send Button
                button(container(
                    iced::widget::svg(arrow_icon)
                        .width(Length::Fixed(14.0))
                        .height(Length::Fixed(14.0))
                ))
                .on_press(InspectorMessage::SendPressed)
                .padding(8)
                .style(move |_: &iced::Theme, status| {
                    let mut bg = tokens.accent;
                    if status != iced::widget::button::Status::Hovered {
                        bg.a = 0.8;
                    }
                    button::Style {
                        background: Some(bg.into()),
                        text_color: Color::WHITE,
                        border: iced::Border {
                            radius: 20.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        ])
        .padding(12)
        .style(move |_| container::Style {
            background: Some(tokens.card_bg.into()),
            border: iced::Border {
                radius: tokens.radius.into(),
                width: 1.0,
                color: tokens.glass_border,
            },
            ..Default::default()
        });

        container(
            column![header, chat_list, refined_input_area]
                .spacing(10)
                .padding(iced::Padding {
                    top: 40.0,
                    left: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                }),
        )
        .width(360)
        .height(Length::Fill)
        .padding(10)
        .style(move |_| container::Style {
            background: Some(bg_color.into()),
            border: iced::Border {
                width: 1.0,
                color: tokens.glass_border,
                ..Default::default()
            },
            shadow: iced::Shadow {
                color: tokens.shadow_color,
                offset: iced::Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        })
        .into()
    }
}
