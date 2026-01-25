use iced::Result;
use peak_ui::reference;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

fn main() -> Result {
    #[cfg(not(target_arch = "wasm32"))]
    {
        iced::application(
            "PeakUI Showcase",
            reference::App::update,
            reference::App::view,
        )
        .run()
    }

    #[cfg(target_arch = "wasm32")]
    {
        // This will be called by #[wasm_bindgen(start)] below,
        // but adding a dummy return for the compiler.
        Ok(())
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("Console log failed");
    log::info!("PeakUI Showcase WASM started");

    let result = iced::application(
        "WASM Clean Slate",
        reference::App::update,
        reference::App::view,
    )
    .window(iced::window::Settings {
        visible: true,
        ..Default::default()
    })
    .style(|_theme, _style| iced::application::Appearance {
        background_color: iced::Color::BLACK,
        text_color: iced::Color::WHITE,
    })
    .font(include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Bold.ttf").as_slice())
    .run();

    #[cfg(target_arch = "wasm32")]
    if let Err(e) = result {
        log::error!("Iced run failed: {:?}", e);
    }
}
