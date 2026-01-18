use iced::widget::{button, column, container, row, scrollable, text, text_editor, vertical_space};
use iced::{Alignment, Color, Element, Length, Task};
use peak_intelligence::brain::assistant::{Assistant, Backend, Message as BrainMessage, Token};
use peak_intelligence::brain::{self};
use peak_intelligence::sipper::Sipper;
use std::sync::Arc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum InspectorMessage {
    InputChanged(text_editor::Action),
    SendPressed,
    ResponseReceived(Result<String, String>),
    OpenSettings,
    MouseReleased, // Forward mouse releases to parent
    BootProgress(String, u32),
    BootFinished(Result<Arc<Assistant>, String>),
    StreamToken(String),
    StreamFinished,
    SetActiveModel(String),
}

#[derive(Debug, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum InspectorState {
    Idle,
    Booting { progress: u32, stage: String },
    Ready,
    Streaming,
    Error(String),
}

pub struct Inspector {
    pub is_visible: bool,
    input_content: text_editor::Content,
    messages: Vec<ChatMessage>,
    pub state: InspectorState,
    assistant: Option<Arc<Assistant>>,
    pending_chat: Option<(String, Vec<BrainMessage>)>,
    active_model_id: Option<String>,
}

impl Inspector {
    pub fn new() -> Self {
        let inspector = Self {
            is_visible: false,
            input_content: text_editor::Content::new(),
            messages: vec![ChatMessage {
                role: "system".to_string(),
                content: "Initializing Peak Intelligence...".to_string(),
            }],
            state: InspectorState::Idle,
            assistant: None,
            pending_chat: None,
            active_model_id: None,
        };

        // We defer boot to first update or explicit call to avoid blocking init
        inspector
    }

    pub fn boot(&mut self) -> Task<InspectorMessage> {
        self.state = InspectorState::Booting {
            progress: 0,
            stage: "Starting...".into(),
        };

        let target_id = self.active_model_id.clone();

        Task::perform(
            async move {
                let lib = brain::model::Library::default();
                // TODO: Load configured model from settings
                // For now, we attempt to find *any* model or fail
                let files = lib.files();
                if files.is_empty() {
                    return Err("No models found. Please download one in settings.".into());
                }

                // Find target model or fallback to first
                let file = if let Some(id) = target_id {
                    files
                        .iter()
                        .find(|f| f.model.0 == id)
                        .cloned()
                        .unwrap_or_else(|| files[0].clone())
                } else {
                    files[0].clone()
                };

                let directory = lib.directory().clone();

                let mut boot_process = Assistant::boot(directory, file, Backend::Cpu).pin();

                while let Some(_event) = boot_process.sip().await {
                    // Just wait for completion
                }

                match boot_process.await {
                    Ok(a) => Ok(Arc::new(a)),
                    Err(e) => Err(e.to_string()),
                }
            },
            InspectorMessage::BootFinished,
        )
    }

    pub fn update(&mut self, message: InspectorMessage) -> Task<InspectorMessage> {
        match message {
            InspectorMessage::SetActiveModel(id) => {
                let was_different = self.active_model_id.as_ref() != Some(&id);
                self.active_model_id = Some(id);

                // If model changed and we are already running, we should reboot
                if was_different {
                    // Only reboot if we have valid state to reboot from (not streaming/booting)
                    match self.state {
                        InspectorState::Ready | InspectorState::Error(_) => {
                            self.messages.push(ChatMessage {
                                role: "system".into(),
                                content: "Switching model...".into(),
                            });
                            return self.boot();
                        }
                        _ => {}
                    }
                }
                Task::none()
            }
            InspectorMessage::InputChanged(action) => {
                self.input_content.perform(action);
                Task::none()
            }
            InspectorMessage::OpenSettings => Task::none(),
            InspectorMessage::MouseReleased => Task::none(),
            InspectorMessage::BootProgress(stage, pct) => {
                if let InspectorState::Booting { .. } = &mut self.state {
                    self.state = InspectorState::Booting {
                        progress: pct,
                        stage,
                    };
                }
                Task::none()
            }
            InspectorMessage::BootFinished(result) => {
                match result {
                    Ok(assistant) => {
                        self.assistant = Some(assistant);
                        self.state = InspectorState::Ready;
                        self.messages.push(ChatMessage {
                            role: "system".to_string(),
                            content: "System Ready.".to_string(),
                        });
                    }
                    Err(e) => {
                        self.state = InspectorState::Error(e.clone());
                        self.messages.push(ChatMessage {
                            role: "system".to_string(),
                            content: format!("Initialization Failed: {}", e),
                        });
                    }
                }
                Task::none()
            }
            InspectorMessage::SendPressed => {
                let text = self.input_content.text();
                if text.trim().is_empty() {
                    return Task::none();
                }

                // If not ready, try boot?
                if self.assistant.is_none() {
                    return self.boot();
                }

                let user_msg = ChatMessage {
                    role: "user".to_string(),
                    content: text.clone(),
                };
                self.messages.push(user_msg.clone());
                self.input_content = text_editor::Content::new();
                self.state = InspectorState::Streaming;

                // Prepare history
                let history: Vec<BrainMessage> = self
                    .messages
                    .iter()
                    .map(|m| match m.role.as_str() {
                        "user" => BrainMessage::User(m.content.clone()),
                        "assistant" => BrainMessage::Assistant(m.content.clone()),
                        "system" => BrainMessage::System(m.content.clone()),
                        _ => BrainMessage::User(m.content.clone()),
                    })
                    .collect();

                // Set pending chat for subscription
                self.pending_chat = Some((text.clone(), history));

                // Add placeholder for assistant response
                self.messages.push(ChatMessage {
                    role: "assistant".into(),
                    content: String::new(),
                });

                Task::none()
            }
            InspectorMessage::StreamToken(t) => {
                if let Some(msg) = self.messages.last_mut() {
                    if msg.role == "assistant" {
                        msg.content.push_str(&t);
                    }
                }
                Task::none()
            }
            InspectorMessage::StreamFinished => {
                self.state = InspectorState::Ready;
                self.pending_chat = None;
                Task::none()
            }
            InspectorMessage::ResponseReceived(_) => Task::none(), // Legacy
        }
    }

    pub fn subscription(&self) -> iced::Subscription<InspectorMessage> {
        if let InspectorState::Streaming = self.state {
            if let Some((prompt, history)) = &self.pending_chat {
                if let Some(assistant) = &self.assistant {
                    let assistant = assistant.as_ref().clone();
                    let prompt = prompt.clone();
                    let history = history.clone();

                    use peak_intelligence::sipper::StreamExt;

                    return iced::Subscription::run_with_id(
                        (prompt.clone(), history.len()),
                        StreamExt::map(
                            assistant.reply(prompt, history, vec![]),
                            |item| match item {
                                (_, Token::Talking(s)) => InspectorMessage::StreamToken(s),
                                _ => InspectorMessage::ResponseReceived(Ok(String::new())),
                            },
                        ),
                    );
                }
            }
        }
        iced::Subscription::none()
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
