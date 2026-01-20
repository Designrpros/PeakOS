use iced::{Element, Length, Renderer, Theme};
use peak_core::registry::ShellMode;
use peak_theme::{ThemeTokens, ThemeTone};
use peak_ui::prelude::*;

pub fn main() -> iced::Result {
    iced::application("PeakUI Showcase", App::update, App::view).run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Page {
    Typography,
    Controls,
    Layouts,
    Forms,
    AdaptiveTest,
}

struct App {
    current_page: Page,
    mode: ShellMode,
    tone: ThemeTone,
    toggle_state: bool,
    slider_value: f32,
    show_sidebar: bool,
}

#[derive(Debug, Clone)]
enum Message {
    PageSelected(Page),
    ToneChanged(bool),
    ButtonTapped,
    ToggleToggled(bool),
    SliderChanged(f32),
    GoBack,
}

impl App {
    fn update(state: &mut Self, message: Message) {
        match message {
            Message::PageSelected(page) => {
                state.current_page = page;
                state.show_sidebar = false;
            }
            Message::ToneChanged(is_dark) => {
                state.tone = if is_dark {
                    ThemeTone::Dark
                } else {
                    ThemeTone::Light
                }
            }
            Message::ButtonTapped => println!("Button tapped!"),
            Message::ToggleToggled(b) => state.toggle_state = b,
            Message::SliderChanged(v) => state.slider_value = v,
            Message::GoBack => state.show_sidebar = true,
        }
    }

    fn view(state: &Self) -> Element<'_, Message, Theme, Renderer> {
        let current_page = state.current_page;
        let mode = state.mode;
        let tone = state.tone;
        let toggle_state = state.toggle_state;
        let slider_value = state.slider_value;
        let show_sidebar = state.show_sidebar;

        responsive(mode, ThemeTokens::get(mode, tone), move |context| {
            NavigationSplitView::new(
                // Sidebar
                VStack::new()
                    .padding(20.0)
                    .spacing(24.0)
                    .push(
                        VStack::new()
                            .spacing(4.0)
                            .push(Text::new("PeakUI").large_title())
                            .push(Text::new("Framework Preview").secondary()),
                    )
                    .push(Section::new(
                        "CATEGORIES",
                        VStack::new()
                            .spacing(4.0)
                            .push(
                                Button::new("Typography")
                                    .style(if current_page == Page::Typography {
                                        ButtonStyle::Primary
                                    } else {
                                        ButtonStyle::Ghost
                                    })
                                    .on_press(Message::PageSelected(Page::Typography))
                                    .width(Length::Fill),
                            )
                            .push(
                                Button::new("Controls")
                                    .style(if current_page == Page::Controls {
                                        ButtonStyle::Primary
                                    } else {
                                        ButtonStyle::Ghost
                                    })
                                    .on_press(Message::PageSelected(Page::Controls))
                                    .width(Length::Fill),
                            )
                            .push(
                                Button::new("Layouts")
                                    .style(if current_page == Page::Layouts {
                                        ButtonStyle::Primary
                                    } else {
                                        ButtonStyle::Ghost
                                    })
                                    .on_press(Message::PageSelected(Page::Layouts))
                                    .width(Length::Fill),
                            )
                            .push(
                                Button::new("Forms")
                                    .style(if current_page == Page::Forms {
                                        ButtonStyle::Primary
                                    } else {
                                        ButtonStyle::Ghost
                                    })
                                    .on_press(Message::PageSelected(Page::Forms))
                                    .width(Length::Fill),
                            )
                            .push(
                                Button::new("Adaptive")
                                    .style(if current_page == Page::AdaptiveTest {
                                        ButtonStyle::Primary
                                    } else {
                                        ButtonStyle::Ghost
                                    })
                                    .on_press(Message::PageSelected(Page::AdaptiveTest))
                                    .width(Length::Fill),
                            ),
                    ))
                    .push(Section::new(
                        "APPEARANCE",
                        VStack::new().spacing(10.0).push(Toggle::new(
                            "Dark Mode",
                            tone == ThemeTone::Dark,
                            Message::ToneChanged,
                        )),
                    )),
                // Content
                VStack::new().width(Length::Fill).push(match current_page {
                    Page::Typography => Box::new(App::typography_view()) as Box<dyn View<Message>>,
                    Page::Controls => {
                        Box::new(App::controls_view(toggle_state)) as Box<dyn View<Message>>
                    }
                    Page::Layouts => {
                        Box::new(App::layouts_view(&context)) as Box<dyn View<Message>>
                    }
                    Page::Forms => Box::new(App::forms_view(toggle_state, slider_value))
                        as Box<dyn View<Message>>,
                    Page::AdaptiveTest => {
                        Box::new(App::adaptive_view(&context)) as Box<dyn View<Message>>
                    }
                }),
            )
            .force_sidebar_on_slim(show_sidebar)
            .on_back(Message::GoBack)
            .view(&context)
        })
        .into()
    }
    fn typography_view() -> Box<dyn View<Message>> {
        Box::new(VStack::new()
            .spacing(32.0)
            .padding(40.0)
            .push(Text::new("Typography").large_title())
            .push(Section::new("HEADINGS", 
                VStack::new()
                    .spacing(8.0)
                    .push(Text::new("Display Title").large_title())
                    .push(Text::new("Subheading").title())
            ))
            .push(Section::new("BODY", 
                Card::new(
                    VStack::new()
                        .spacing(12.0)
                        .push(Text::new("This is standard body text that uses the primary theme color. It is designed for maximum readability across all platforms.").body())
                        .push(Text::new("This is secondary text, used for metadata, captions, or less important information. It has a slightly reduced opacity and size.").secondary()),
                ).width(Length::Fill)
            )))
    }

    fn controls_view(toggle_state: bool) -> Box<dyn View<Message>> {
        Box::new(
            VStack::new()
                .spacing(32.0)
                .padding(40.0)
                .push(Text::new("Controls").large_title())
                .push(Section::new(
                    "BUTTON STYLES",
                    Card::new(
                        HStack::new()
                            .spacing(16.0)
                            .push(
                                Button::new("Prominent")
                                    .style(ButtonStyle::Primary)
                                    .on_press(Message::ButtonTapped),
                            )
                            .push(
                                Button::new("Secondary")
                                    .style(ButtonStyle::Secondary)
                                    .on_press(Message::ButtonTapped),
                            )
                            .push(
                                Button::new("Ghost")
                                    .style(ButtonStyle::Ghost)
                                    .on_press(Message::ButtonTapped),
                            )
                            .push(
                                Button::new("Destructive")
                                    .style(ButtonStyle::Destructive)
                                    .on_press(Message::ButtonTapped),
                            ),
                    )
                    .width(Length::Fill),
                ))
                .push(Section::new(
                    "SELECTION",
                    Card::new(
                        VStack::new()
                            .spacing(12.0)
                            .push(Toggle::new(
                                "Enable Analytics",
                                toggle_state,
                                Message::ToggleToggled,
                            ))
                            .push(
                                Text::new("Analytics help us improve the framework experience.")
                                    .secondary(),
                            ),
                    )
                    .width(Length::Fill),
                )),
        )
    }

    fn layouts_view(_context: &Context) -> Box<dyn View<Message>> {
        Box::new(
            VStack::new()
                .spacing(32.0)
                .padding(40.0)
                .push(Text::new("Responsive Layouts").large_title())
                .push(Section::new(
                    "GRIDS (RESIZE WINDOW TO TEST)",
                    ResponsiveGrid::new()
                        .spacing(20.0)
                        .push(
                            Card::new(
                                VStack::new()
                                    .spacing(8.0)
                                    .push(Text::new("Performance").title())
                                    .push(Text::new("Hardware accelerated rendering with minimal overhead.").secondary())
                            )
                            .width(Length::Fill)
                        )
                        .push(
                            Card::new(
                                VStack::new()
                                    .spacing(8.0)
                                    .push(Text::new("Experience").title())
                                    .push(Text::new("Thoughtfully crafted interactions for modern workflows.").secondary())
                            )
                            .width(Length::Fill)
                        )
                        .push(
                            Card::new(
                                VStack::new()
                                    .spacing(8.0)
                                    .push(Text::new("Modularity").title())
                                    .push(Text::new("Easily extend and customize every part of the system.").secondary())
                            )
                            .width(Length::Fill)
                        ),
                ))
                .push(Section::new(
                    "OVERLAYS",
                    ZStack::new()
                        .width(Length::Fill)
                        .height(Length::Fixed(160.0))
                        .push(
                            VStack::new()
                                .width(Length::Fill)
                                .height(Length::Fill)
                                .align_x(iced::Alignment::Center)
                                .push(Text::new("Background Content").secondary()),
                        )
                        .push(
                            Card::new(
                                VStack::new()
                                    .spacing(4.0)
                                    .push(Text::new("Floating Card").title())
                                    .push(Text::new("Using ZStack").secondary()),
                            )
                            .width(Length::Fixed(200.0)),
                        ),
                )),
        )
    }

    fn adaptive_view(context: &Context) -> Box<dyn View<Message>> {
        Box::new(
            VStack::new()
                .spacing(20.0)
                .padding(40.0)
                .push(Text::new("Adaptive Layout").large_title())
                .push(Text::new(format!("Current Device: {:?}", context.device)).body())
                .push(Card::new(
                    VStack::new()
                        .push(Text::new("The Sidebar in this app is context-aware.").body())
                        .push(
                            Text::new("In Desktop mode, it is persistent on the left.").secondary(),
                        )
                        .push(Text::new("In Mobile/TV mode, it is hidden by default.").secondary()),
                )),
        )
    }

    fn forms_view(toggle_state: bool, slider_value: f32) -> Box<dyn View<Message>> {
        Box::new(
            VStack::new()
                .spacing(32.0)
                .padding(40.0)
                .push(Text::new("Forms").large_title())
                .push(
                    Form::new()
                        .style(FormStyle::Grouped)
                        .push(Section::new(
                            "SCREEN TIME",
                            VStack::new()
                                .spacing(12.0)
                                .push(Toggle::new(
                                    "App Limits",
                                    toggle_state,
                                    Message::ToggleToggled,
                                ))
                                .push(
                                    Text::new("Set daily time limits for app categories.")
                                        .secondary(),
                                ),
                        ))
                        .push(Section::new(
                            "BRIGHTNESS",
                            VStack::new()
                                .spacing(12.0)
                                .push(
                                    HStack::new()
                                        .spacing(12.0)
                                        .push(Text::new("Display").body())
                                        .push(
                                            Slider::new(
                                                0.0..=100.0,
                                                slider_value,
                                                Message::SliderChanged,
                                            )
                                            .width(Length::Fill),
                                        ),
                                )
                                .push(
                                    Text::new(format!("Current Brightness: {:.0}%", slider_value))
                                        .secondary(),
                                ),
                        ))
                        .push(Section::new(
                            "ACCOUNT",
                            VStack::new().push(
                                Button::new("Sign Out")
                                    .style(ButtonStyle::Destructive)
                                    .on_press(Message::ButtonTapped)
                                    .width(Length::Fill),
                            ),
                        )),
                ),
        )
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_page: Page::Typography,
            mode: ShellMode::Desktop,
            tone: ThemeTone::Light,
            toggle_state: false,
            slider_value: 50.0,
            show_sidebar: true,
        }
    }
}
