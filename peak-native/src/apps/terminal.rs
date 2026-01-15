#![allow(dead_code, unused_imports)]

use iced::widget::{canvas, container, scrollable, text, text_input, Column};
use iced::{Background, Color, Element, Length, Subscription};
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

// The Message specific to the Terminal
#[derive(Debug, Clone)]
pub enum TerminalMessage {
    OutputReceived(String),
    InputChanged(String),
    InputSubmitted,
}

pub struct TerminalApp {
    content: String,
    input_buffer: String,
    // We keep the writer to send keystrokes to bash
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    // Channel receiver for PTY output
    receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<String>>>,
    pub is_open: bool,
}

impl TerminalApp {
    pub fn new() -> Self {
        // 1. Spawn the PTY
        let pty_system = NativePtySystem::default();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to create PTY");

        // 2. Spawn the Shell (bash or sh)
        let shell = if std::path::Path::new("/bin/bash").exists() {
            "bash"
        } else {
            "sh"
        };
        let cmd = CommandBuilder::new(shell);
        let _child = pair
            .slave
            .spawn_command(cmd)
            .expect("Failed to spawn shell");

        // 3. Store handles & spawn background reader
        let mut reader = pair.master.try_clone_reader().unwrap();
        let writer = pair.master.take_writer().unwrap();

        let (tx, rx) = mpsc::unbounded_channel();

        // Background thread to read from PTY continuously
        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&buf[..n]).to_string();
                        if tx.send(text).is_err() {
                            break; // Channel closed
                        }
                    }
                    _ => break, // Error or EOF
                }
            }
        });

        Self {
            content: String::from("PeakOS Terminal v0.1\n> "),
            input_buffer: String::new(),
            writer: Arc::new(Mutex::new(writer)),
            receiver: Arc::new(tokio::sync::Mutex::new(rx)),
            is_open: false,
        }
    }

    pub fn update(&mut self, message: TerminalMessage) {
        match message {
            TerminalMessage::OutputReceived(text) => {
                // Strip ANSI escape sequences to avoid "square" characters
                let cleaned = strip_ansi(&text);
                self.content.push_str(&cleaned);

                // Keep content length manageable
                if self.content.len() > 10000 {
                    let to_remove = self.content.len() - 10000;
                    self.content.drain(..to_remove);
                }
            }
            TerminalMessage::InputChanged(val) => {
                self.input_buffer = val;
            }
            TerminalMessage::InputSubmitted => {
                let cmd = format!("{}\n", self.input_buffer);
                self.input_buffer.clear();
                if let Ok(mut writer) = self.writer.lock() {
                    write!(writer, "{}", cmd).unwrap();
                }
            }
        }
    }

    pub fn view<'a>(&'a self, is_light: bool) -> Element<'a, TerminalMessage> {
        let (text_color, bg_color, border_color) = if is_light {
            (
                Color::from_rgb8(35, 30, 30),
                Color::from_rgb8(247, 245, 242),
                Color::from_rgba(0.0, 0.0, 0.0, 0.1),
            )
        } else {
            (
                Color::from_rgb8(235, 230, 225),
                Color::from_rgb8(15, 14, 14),
                Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            )
        };

        let output = text(&self.content)
            .font(iced::Font::MONOSPACE)
            .size(12)
            .color(text_color);

        let input = text_input("Type a command...", &self.input_buffer)
            .on_input(TerminalMessage::InputChanged)
            .on_submit(TerminalMessage::InputSubmitted)
            .padding(10)
            .size(12)
            .font(iced::Font::MONOSPACE)
            .style(move |_, _| text_input::Style {
                background: Background::Color(Color::TRANSPARENT),
                border: iced::Border {
                    width: 0.0,
                    radius: 0.0.into(),
                    color: Color::TRANSPARENT,
                },
                icon: Color::TRANSPARENT,
                placeholder: text_color,
                value: text_color,
                selection: text_color,
            });

        let term_content = Column::new()
            .push(scrollable(output).height(Length::Fill).width(Length::Fill))
            .push(
                container(input)
                    .padding(5)
                    .style(move |_| container::Style {
                        border: iced::Border {
                            width: 1.0,
                            color: border_color,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    }),
            );

        container(term_content)
            .padding(8)
            .style(move |_| container::Style {
                background: Some(bg_color.into()),
                border: iced::Border {
                    color: border_color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<TerminalMessage> {
        iced::Subscription::run_with_id(
            "terminal_listener",
            iced::futures::stream::unfold(self.receiver.clone(), |receiver| async move {
                let mut rx = receiver.lock().await;
                if let Some(text) = rx.recv().await {
                    Some((TerminalMessage::OutputReceived(text), receiver.clone()))
                } else {
                    // Channel closed
                    None
                }
            }),
        )
    }
}

fn strip_ansi(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_escape = false;
    let mut in_csi = false; // Control Sequence Introducer

    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_escape {
            if b == b'[' {
                in_csi = true;
                in_escape = false;
            } else if (0x40..=0x5F).contains(&b) {
                // Other escape sequences like ESC N
                in_escape = false;
            } else {
                // Invalid or incomplete
                in_escape = false;
            }
        } else if in_csi {
            // CSI sequences end with 0x40-0x7E
            if (0x40..=0x7E).contains(&b) {
                in_csi = false;
            }
        } else if b == 0x1B {
            // ESC
            in_escape = true;
        } else {
            result.push(b as char);
        }
        i += 1;
    }
    result
}
