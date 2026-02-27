use iced::{Font, Size, window::Settings};

use crate::app::App;

mod app;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .window(Settings {
            size: Size {
                width: 1080.0,
                height: 600.0,
            },
            resizable: false,
            ..Settings::default()
        })
        .default_font(Font::MONOSPACE)
        .run()
}
