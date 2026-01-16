use iced::Task;
use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum TerminalMessage {
    OutputReceived(String),
    InputChanged(String),
    InputSubmitted,
}

pub struct TerminalApp {
    pub content: String,
    pub input_buffer: String,
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
    receiver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<String>>>,
    pub is_open: bool,
}

impl TerminalApp {
    pub fn new() -> Self {
        let pty_system = NativePtySystem::default();
        let pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                pixel_width: 0,
                pixel_height: 0,
            })
            .expect("Failed to create PTY");

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

        let mut reader = pair.master.try_clone_reader().unwrap();
        let writer = pair.master.take_writer().unwrap();

        let (tx, rx) = mpsc::unbounded_channel();

        std::thread::spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match reader.read(&mut buf) {
                    Ok(n) if n > 0 => {
                        let text = String::from_utf8_lossy(&buf[..n]).to_string();
                        if tx.send(text).is_err() {
                            break;
                        }
                    }
                    _ => break,
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

    pub fn update(&mut self, message: TerminalMessage) -> Task<TerminalMessage> {
        match message {
            TerminalMessage::OutputReceived(text) => {
                let cleaned = strip_ansi(&text);
                self.content.push_str(&cleaned);

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
                    let _ = write!(writer, "{}", cmd);
                }
            }
        }
        Task::none()
    }

    pub fn subscription(&self) -> iced::Subscription<TerminalMessage> {
        iced::Subscription::run_with_id(
            "terminal_listener",
            iced::futures::stream::unfold(self.receiver.clone(), |receiver| async move {
                let mut rx = receiver.lock().await;
                if let Some(text) = rx.recv().await {
                    Some((TerminalMessage::OutputReceived(text), receiver.clone()))
                } else {
                    None
                }
            }),
        )
    }
}

pub fn strip_ansi(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_escape = false;
    let mut in_csi = false;

    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if in_escape {
            if b == b'[' {
                in_csi = true;
                in_escape = false;
            } else if (0x40..=0x5F).contains(&b) {
                in_escape = false;
            } else {
                in_escape = false;
            }
        } else if in_csi {
            if (0x40..=0x7E).contains(&b) {
                in_csi = false;
            }
        } else if b == 0x1B {
            in_escape = true;
        } else {
            result.push(b as char);
        }
        i += 1;
    }
    result
}
