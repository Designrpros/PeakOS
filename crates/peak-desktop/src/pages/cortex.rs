#![allow(dead_code)]
use crate::app::Message;
use crate::components::graph::Graph;
use peak_core::theme::Theme;
use iced::widget::{column, container, row, text};
use iced::{Color, Element, Length};
use std::collections::VecDeque;

// Configuration
const HISTORY_SIZE: usize = 60; // 60 seconds of history

pub struct State {
    cpu_history: VecDeque<f32>,
    ram_history: VecDeque<f32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            cpu_history: VecDeque::from(vec![0.0; HISTORY_SIZE]),
            ram_history: VecDeque::from(vec![0.0; HISTORY_SIZE]),
        }
    }

    // Called by app.rs when a Tick happens
    pub fn push_data(&mut self, cpu: f32, ram_percent: f32) {
        self.cpu_history.push_back(cpu);
        if self.cpu_history.len() > HISTORY_SIZE {
            self.cpu_history.pop_front();
        }

        self.ram_history.push_back(ram_percent);
        if self.ram_history.len() > HISTORY_SIZE {
            self.ram_history.pop_front();
        }
    }

    pub fn view(&self, _theme: &Theme) -> Element<'_, Message> {
        let title = text("Cortex Telemetry").size(32);

        // CPU Graph (Cyber Pink)
        let cpu_graph = container(
            Graph::new(
                self.cpu_history.iter().copied().collect(),
                Color::from_rgb8(255, 0, 128),
                "CPU LOAD".into(),
            )
            .view(),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(15, 15, 20))),
            border: iced::Border {
                radius: 10.0.into(),
                ..iced::Border::default()
            },
            ..Default::default()
        });

        // RAM Graph (Cyber Blue)
        let ram_graph = container(
            Graph::new(
                self.ram_history.iter().copied().collect(),
                Color::from_rgb8(0, 255, 200),
                "MEMORY USAGE".into(),
            )
            .view(),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .style(move |_| container::Style {
            background: Some(iced::Background::Color(Color::from_rgb8(15, 15, 20))),
            border: iced::Border {
                radius: 10.0.into(),
                ..iced::Border::default()
            },
            ..Default::default()
        });

        column![
            title,
            row![cpu_graph, ram_graph]
                .spacing(20)
                .height(Length::Fixed(300.0)),
            // Add GPU or Thermal graphs below...
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
