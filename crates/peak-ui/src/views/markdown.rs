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
        let mut col = Column::new().spacing(16).width(Length::Fill);
        let mut in_code_block = false;
        let mut current_language: Option<String> = None;
        let mut code_buffer = String::new();

        for line in self.content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                if in_code_block {
                    col = col.push(render_code_block(
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

            if trimmed.starts_with("# ") {
                col = col.push(
                    Text::<IcedBackend>::new(trimmed.trim_start_matches("# ").trim())
                        .large_title()
                        .bold()
                        .view(context),
                );
            } else if trimmed.starts_with("## ") {
                col = col.push(
                    Text::<IcedBackend>::new(trimmed.trim_start_matches("## ").trim())
                        .title1()
                        .bold()
                        .view(context),
                );
            } else if trimmed.starts_with("### ") {
                col = col.push(
                    Text::<IcedBackend>::new(trimmed.trim_start_matches("### ").trim())
                        .title3()
                        .bold()
                        .view(context),
                );
            } else if trimmed.starts_with("- [ ] ") || trimmed.starts_with("- [x] ") {
                let is_checked = trimmed.starts_with("- [x] ");
                let content = if is_checked {
                    trimmed.trim_start_matches("- [x] ").trim()
                } else {
                    trimmed.trim_start_matches("- [ ] ").trim()
                };

                col = col.push(
                    Row::new()
                        .spacing(12)
                        .align_y(iced::Alignment::Start)
                        .push(
                            Icon::<IcedBackend>::new(if is_checked {
                                "check-square"
                            } else {
                                "square"
                            })
                            .size(16.0)
                            .color(if is_checked {
                                context.theme.colors.success
                            } else {
                                context.theme.colors.text_secondary
                            })
                            .view(context),
                        )
                        .push(render_rich_text(content, context)),
                );
            } else if trimmed.starts_with("- ") {
                col = col.push(
                    Row::new()
                        .spacing(8)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("•")
                                .size(14)
                                .color(context.theme.colors.text_secondary),
                        )
                        .push(render_rich_text(
                            trimmed.trim_start_matches("- ").trim(),
                            context,
                        )),
                );
            } else if let Some(rest) = parse_numbered_list(trimmed) {
                // Numbered list: "1. Content"
                col = col.push(
                    Row::new()
                        .spacing(8)
                        .align_y(iced::Alignment::Start)
                        .push(
                            text("1.") // For now, simple bullet, or we could parse the actual number
                                .size(14)
                                .color(context.theme.colors.text_secondary)
                                .font(font::Font::MONOSPACE),
                        )
                        .push(render_rich_text(rest, context)),
                );
            } else {
                col = col.push(render_rich_text(trimmed, context));
            }
        }

        if !code_buffer.is_empty() {
            col = col.push(render_code_block(
                &code_buffer,
                current_language.as_deref(),
                context,
            ));
        }

        col.into()
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
    // Basic check for digit + dot + space
    // e.g. "1. "
    let chars: Vec<char> = line.chars().take(10).collect();
    if let Some(dot_idx) = chars.iter().position(|&c| c == '.') {
        if dot_idx > 0 && dot_idx < chars.len() - 1 && chars[dot_idx + 1] == ' ' {
            // Check if everything before dot is digit
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
    // Simple inline parser for **bold** and `code`
    // We split by tokens
    let mut spans = Vec::new();
    let theme = context.theme;

    let mut remaining = content;

    while !remaining.is_empty() {
        if let Some(start) = remaining.find("**") {
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
                        .color(theme.colors.text_primary),
                );
                remaining = &remaining[start + 2 + end + 2..];
            } else {
                // Unclosed bold
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
                        .color(theme.colors.primary),
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
        .size(14)
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
    let radius = context.radius(12.0);
    let bg = theme.colors.surface_variant.scale_alpha(0.15);

    // Debug: Red border to see container
    let border_color = iced::Color::from_rgb(1.0, 0.0, 0.0);

    let content_text = text(content.to_string())
        .font(iced::Font::DEFAULT) // Fallback to safe font
        .size(13.0)
        .color(theme.colors.text_primary)
        .width(Length::Fill);

    let inner: Element<_, _, _> = if let Some(lang) = language {
        Column::new()
            .spacing(8)
            .width(Length::Fill)
            .push(
                Container::new(
                    text(lang.to_uppercase())
                        .size(10)
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
            .push(content_text)
            .into()
    } else {
        content_text.into()
    };

    container(inner)
        .padding(24)
        .width(Length::Fill)
        .style(move |_| container::Style {
            background: Some(bg.into()),
            border: iced::Border {
                radius,
                color: border_color, // Use debug Red
                width: 2.0,          // Thicker border
            },
            ..Default::default()
        })
        .into()
}
