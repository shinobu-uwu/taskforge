use iced::{
    Size, Theme,
    window::{Settings, settings::PlatformSpecific},
};

use crate::app::App;

mod app;
mod screen;
mod widgets;

fn main() -> anyhow::Result<()> {
    iced::application(App::new, App::update, App::view)
        .font(iced_fonts::LUCIDE_FONT_BYTES)
        .subscription(App::subscription)
        .theme(Theme::Dracula)
        .window(Settings {
            size: Size::new(1100.0, 700.0),
            min_size: Some(Size::new(800.0, 500.0)),
            platform_specific: PlatformSpecific {
                application_id: "taskforge".to_owned(),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()?;

    Ok(())
}
