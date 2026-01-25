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
        "PeakUI Showcase",
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
    .subscription(reference::App::subscription)
    .run_with(|| {
        #[cfg(target_arch = "wasm32")]
        let (initial_page, hash, path) = {
            let h = web_sys::window()
                .and_then(|w| w.location().hash().ok())
                .unwrap_or_default();

            // Remove '#' if present
            let p = if h.starts_with('#') {
                h[1..].to_string()
            } else {
                h.clone()
            };

            let page = reference::model::Page::from_path(&p);
            (page, h, p)
        };

        #[cfg(target_arch = "wasm32")]
        log::info!(
            "BOOTING - Hash: '{}', Path: '{}', Page: {:?}, Mode: {}",
            hash,
            path,
            initial_page,
            initial_page.navigation_mode()
        );

        #[cfg(not(target_arch = "wasm32"))]
        let initial_page = reference::model::Page::default();

        let mut app = reference::App::default();
        app.navigation_mode = initial_page.navigation_mode();
        app.active_tab = initial_page;

        (
            app,
            iced::font::load(
                include_bytes!("../assets/fonts/Fira_Sans/FiraSans-Bold.ttf").as_slice(),
            )
            .map(|_| reference::app::Message::FontLoaded(Ok(()))),
        )
    });

    #[cfg(target_arch = "wasm32")]
    if let Err(e) = result {
        log::error!("Iced run failed: {:?}", e);
    }
}
