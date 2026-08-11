use iced::{
    Size,
    window::{Settings, settings::PlatformSpecific},
};

use crate::app::App;

mod app;
mod screen;
mod widgets;

fn main() -> anyhow::Result<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();
    iced::application(App::new, App::update, App::view)
        .font(iced_fonts::LUCIDE_FONT_BYTES)
        .subscription(App::subscription)
        .theme(|app: &App| app.current_theme.clone())
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
