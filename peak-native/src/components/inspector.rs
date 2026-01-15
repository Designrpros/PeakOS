use iced::widget::{button, column, container, row, scrollable, text, text_editor, vertical_space};
use iced::{Alignment, Color, Element, Length, Task};
use peak_intelligence::llm::{LlmClient, Message, ModelProvider};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum InspectorMessage {
    InputChanged(text_editor::Action),
    SendPressed,
    ResponseReceived(Result<String, String>),
    OpenSettings,
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

    pub fn view(&self, is_light: bool) -> Element<'_, InspectorMessage> {
        if !self.is_visible {
            return container(vertical_space().height(0))
                .width(0)
                .height(0)
                .into();
        }

        let bg_color = if is_light {
            Color::from_rgba8(255, 255, 255, 0.85)
        } else {
            Color::from_rgba8(0, 0, 0, 0.85)
        };

        let text_color = if is_light { Color::BLACK } else { Color::WHITE };

        // Settings Icon
        let settings_icon = crate::icons::get_app_icon(
            crate::registry::AppId::Settings,
            if is_light { "#000000" } else { "#FFFFFF" },
        );

        let header = row![
            text("Peak Intelligence").size(14).font(iced::Font {
                weight: iced::font::Weight::Bold,
                ..Default::default()
            }),
            iced::widget::horizontal_space(),
            button(container(
                iced::widget::svg(settings_icon)
                    .width(Length::Fixed(16.0))
                    .height(Length::Fixed(16.0))
            ))
            .on_press(InspectorMessage::OpenSettings)
            .style(move |_: &iced::Theme, _| button::Style {
                background: None,
                ..Default::default()
            })
            .padding(4)
        ]
        .padding(15)
        .align_y(Alignment::Center);

        // Chat List
        let chat_list = scrollable(
            column(
                self.messages
                    .iter()
                    .map(|msg| {
                        let is_user = msg.role == "user";
                        if is_user {
                            // User Bubble: Warm Gray
                            let warm_gray = if is_light {
                                Color::from_rgb8(230, 225, 220)
                            } else {
                                Color::from_rgb8(80, 75, 70)
                            };

                            container(text(&msg.content).size(14).color(text_color))
                                .padding(12)
                                .style(move |_| container::Style {
                                    background: Some(warm_gray.into()),
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

        // Input Area (Bottom)
        let _input_container_bg = if is_light {
            Color::from_rgba8(0, 0, 0, 0.05)
        } else {
            Color::from_rgba8(30, 30, 30, 1.0) // Darker background for input area
        };

        // Icons for toolbar
        let mic_icon =
            crate::icons::get_ui_icon("microphone", if is_light { "#000000" } else { "#FFFFFF" });
        let arrow_icon = crate::icons::get_ui_icon("arrow_up", "#FFFFFF"); // Always white on blue button

        // Let's refine the input layout. The user wants the input text likely ABOVE the toolbar icons (or inline).
        // The screenshot shows "Ask anything..." text at top left of box, and icons at bottom right.
        // It looks like one big text area with a toolbar at the bottom.

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
                        selection: Color::from_rgb(0.4, 0.4, 0.8),
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
                .style(move |_: &iced::Theme, status| button::Style {
                    background: if status == iced::widget::button::Status::Hovered {
                        Some(Color::from_rgb8(60, 60, 65).into())
                    } else {
                        Some(Color::from_rgba8(50, 50, 55, 1.0).into())
                    },
                    text_color: Color::WHITE,
                    border: iced::Border {
                        radius: 20.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
            ]
            .spacing(12)
            .align_y(Alignment::Center)
        ])
        .padding(12)
        .style(move |_| container::Style {
            background: Some(if is_light {
                Color::from_rgba8(240, 240, 240, 1.0).into()
            } else {
                Color::from_rgba8(40, 40, 40, 1.0).into()
            }),
            border: iced::Border {
                radius: 12.0.into(),
                width: 1.0,
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
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
                width: 0.0, // No border on the main panel edge to blend
                color: Color::TRANSPARENT,
                ..Default::default()
            },
            shadow: iced::Shadow::default(), // No shadow
            ..Default::default()
        })
        .into()
    }
}
