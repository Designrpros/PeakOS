use iced::widget::{container, image, stack};
use iced::{ContentFit, Element, Length};

pub fn view<'a, Message>(
    background_path: impl AsRef<std::path::Path>,
    foreground_content: Element<'a, Message>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    // Layer 0: The Wallpaper (Absolute Bottom)
    let wallpaper = container(
        image(iced::widget::image::Handle::from_path(
            background_path.as_ref(),
        ))
        .width(Length::Fill)
        .height(Length::Fill)
        .content_fit(ContentFit::Cover),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::Background::Color(iced::Color::from_rgb8(10, 10, 15))), // Deep Space Blue fallback
        ..Default::default()
    });

    // Layer 1: The UI (Absolute Top)
    stack![
        wallpaper,
        container(foreground_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(iced::Color::TRANSPARENT.into()),
                ..Default::default()
            })
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
