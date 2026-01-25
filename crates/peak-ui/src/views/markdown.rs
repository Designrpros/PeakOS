use crate::prelude::*;
use iced::widget::{text, Column, Container, Row};
use iced::{font, Element, Length, Theme};

pub struct MarkdownView {
    content: String,
}

impl MarkdownView {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

impl<Message> View<Message, IcedBackend> for MarkdownView
where
    Message: 'static + Clone,
{
    fn view(&self, context: &Context) -> Element<'static, Message, Theme, iced::Renderer> {
        let mut children: Vec<Element<'static, Message, Theme, iced::Renderer>> = Vec::new();

        let mut in_code_block = false;
        let mut current_language: Option<String> = None;
        let mut code_buffer = String::new();

        for line in self.content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                if in_code_block {
                    // Render code block
                    children.push(render_code_block(
                        &code_buffer,
                        current_language.as_deref(),
                        context,
                    ));
                    code_buffer.clear();
                    in_code_block = false;
                    current_language = None;
                } else {
                    in_code_block = true;
                    let lang = trimmed.trim_start_matches("```").trim();
                    if !lang.is_empty() {
                        current_language = Some(lang.to_string());
                    }
                }
                continue;
            }

            if in_code_block {
                code_buffer.push_str(line);
                code_buffer.push('\n');
                continue;
            }

            if trimmed.is_empty() {
                continue;
            }

            // Headers
            if trimmed.starts_with("# ") {
                children.push(
                    text(trimmed.trim_start_matches("# ").trim().to_string())
                        .size(32.0)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            } else if trimmed.starts_with("## ") {
                children.push(
                    text(trimmed.trim_start_matches("## ").trim().to_string())
                        .size(24.0)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            } else if trimmed.starts_with("### ") {
                children.push(
                    text(trimmed.trim_start_matches("### ").trim().to_string())
                        .size(20.0)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .width(Length::Fill)
                        .color(context.theme.colors.text_primary)
                        .into(),
                );
            }
            // Checkboxes
            else if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") {
                let is_checked = trimmed.starts_with("- [x] ");
                let content = if is_checked {
                    trimmed.trim_start_matches("- [x] ").trim().to_string()
                } else {
                    trimmed.trim_start_matches("- [ ] ").trim().to_string()
                };

                let icon_text = if is_checked {
                    text("[x]").size(16).color(context.theme.colors.success)
                } else {
                    text("[ ]")
                        .size(16)
                        .color(context.theme.colors.text_secondary)
                };

                children.push(
                    Row::new()
                        .spacing(12)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(icon_text)
                        .push(render_rich_text(&content, context))
                        .into(),
                );
            }
            // Bullet points
            else if trimmed.starts_with("- ") {
                let content = trimmed.trim_start_matches("- ").trim().to_string();
                children.push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("•")
                                .size(14)
                                .color(context.theme.colors.text_secondary),
                        )
                        .push(render_rich_text(&content, context))
                        .into(),
                );
            }
            // Numbered list (Simple detection)
            else if let Some(rest) = parse_numbered_list(trimmed) {
                let content = rest.to_string();
                children.push(
                    Row::new()
                        .spacing(8)
                        .width(Length::Fill)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("1.")
                                .size(14)
                                .color(context.theme.colors.text_secondary)
                                .font(font::Font::MONOSPACE),
                        )
                        .push(render_rich_text(&content, context))
                        .into(),
                );
            }
            // Paragraph
            else {
                children.push(
                    container(render_rich_text(trimmed, context))
                        .width(Length::Fill)
                        .into(),
                );
            }
        }

        if !code_buffer.is_empty() {
            // Clone buffer for static ownership
            children.push(render_code_block(
                &code_buffer,
                current_language.as_deref(),
                context,
            ));
        }

        // Use standard Column from iced
        Column::with_children(children)
            .spacing(16)
            .width(Length::Fill)
            .padding(iced::Padding {
                top: 0.0,
                right: 0.0,
                bottom: 48.0,
                left: 0.0,
            }) // Bottom padding only
            .into()
    }

    fn describe(&self, _context: &Context) -> crate::core::SemanticNode {
        crate::core::SemanticNode {
            role: "article".to_string(),
            label: Some("Markdown Content".to_string()),
            content: Some(self.content.chars().take(100).collect()),
            children: Vec::new(),
        }
    }
}

fn parse_numbered_list(line: &str) -> Option<&str> {
    let chars: Vec<char> = line.chars().take(10).collect();
    if let Some(dot_idx) = chars.iter().position(|&c| c == '.') {
        if dot_idx > 0 && dot_idx < chars.len() - 1 && chars[dot_idx + 1] == ' ' {
            if chars[0..dot_idx].iter().all(|c| c.is_numeric()) {
                return Some(line[dot_idx + 2..].trim());
            }
        }
    }
    None
}

fn render_rich_text<'a, Message>(
    content: &str,
    context: &Context,
) -> Element<'a, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    let mut spans = Vec::new();
    let theme = context.theme;

    // Split strictly by ** for bold and ` for code
    let mut remaining = content;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
            // Text before **
            if start > 0 {
                spans.push(
                    text::Span::new(remaining[..start].to_string())
                        .color(theme.colors.text_secondary),
                );
            }
            if let Some(end) = remaining[start + 2..].find("**") {
                let bold_text = &remaining[start + 2..start + 2 + end];
                spans.push(
                    text::Span::new(bold_text.to_string())
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(theme.colors.text_primary), // Bold gets primary color
                );
                remaining = &remaining[start + 2 + end + 2..];
            } else {
                spans.push(
                    text::Span::new(remaining[..].to_string()).color(theme.colors.text_secondary),
                );
                break;
            }
        } else if let Some(start) = remaining.find('`') {
            if start > 0 {
                spans.push(
                    text::Span::new(remaining[..start].to_string())
                        .color(theme.colors.text_secondary),
                );
            }
            if let Some(end) = remaining[start + 1..].find('`') {
                let code_text = &remaining[start + 1..start + 1 + end];
                spans.push(
                    text::Span::new(format!(" {} ", code_text))
                        .font(font::Font::MONOSPACE)
                        .color(theme.colors.primary), // Code gets accent color
                );
                remaining = &remaining[start + 1 + end + 1..];
            } else {
                spans.push(
                    text::Span::new(remaining[..].to_string()).color(theme.colors.text_secondary),
                );
                break;
            }
        } else {
            spans.push(text::Span::new(remaining.to_string()).color(theme.colors.text_secondary));
            break;
        }
    }

    iced::widget::rich_text(spans)
        .size(16.0) // Fixed size 16.0
        .width(Length::Fill)
        .into()
}

fn render_code_block<Message>(
    content: &str,
    language: Option<&str>,
    context: &Context,
) -> Element<'static, Message, Theme, iced::Renderer>
where
    Message: 'static + Clone,
{
    let theme = context.theme;

    // Explicit fixed sizing, bypassing scaling factor
    let code_text = text(content.to_string())
        .font(iced::Font::MONOSPACE)
        .size(14.0)
        .color(theme.colors.text_primary)
        .width(Length::Shrink);

    let scrollable_code = iced::widget::scrollable(code_text)
        .direction(iced::widget::scrollable::Direction::Horizontal(
            iced::widget::scrollable::Scrollbar::new()
                .width(4)
                .scroller_width(4)
                .margin(2),
        ))
        .width(Length::Fill)
        .id(iced::widget::scrollable::Id::new("code_block"));

    let inner: Element<_, _, _> = if let Some(lang) = language {
        Column::new()
            .spacing(8)
            .width(Length::Fill)
            .push(
                Container::new(
                    text(lang.to_uppercase())
                        .size(12)
                        .font(font::Font {
                            weight: font::Weight::Bold,
                            ..Default::default()
                        })
                        .color(theme.colors.text_secondary),
                )
                .padding([4, 8])
                .style(move |_| container::Style {
                    background: Some(theme.colors.surface_variant.scale_alpha(0.3).into()),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .push(scrollable_code)
            .into()
    } else {
        scrollable_code.into()
    };

    Container::new(inner)
        .padding(16)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(theme.colors.surface_variant.scale_alpha(0.15).into()),
            border: iced::Border {
                radius: 12.0.into(),
                width: 1.0,                                  // Keeping 1.0 width
                color: theme.colors.border.scale_alpha(0.5), // Back to normal border color
            },
            ..Default::default()
        })
        .into()
}
