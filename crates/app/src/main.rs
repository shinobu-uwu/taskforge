mod history;
mod state;
mod view;
mod widgets;

use std::path::PathBuf;

use crate::view::root::RootView;
use crate::widgets::icon::Assets;
use gpui::{App, AppContext, SharedString, WindowOptions};
use gpui_component::{Root, Theme, ThemeRegistry};
use tracing::info;

fn main() -> anyhow::Result<()> {
    let (non_blocking, _guard) = tracing_appender::non_blocking(std::io::stdout());
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();
    info!("Starting app");

    gpui_platform::application()
        .with_assets(Assets)
        .run(move |cx| {
            info!("Initializing gpui-component");
            gpui_component::init(cx);
            info!("Initialed gpui-component");

            info!("Initializing themes");
            init_theme(cx);
            info!("Initialed themes");

            cx.spawn(async move |cx| {
                cx.open_window(WindowOptions::default(), |window, cx| {
                    let view = cx.new(|cx| RootView::new(window, cx));
                    cx.new(|cx| Root::new(view, window, cx))
                })
                .expect("Failed to open window");
            })
            .detach();
        });

    Ok(())
}

fn init_theme(cx: &mut App) {
    let theme_name = SharedString::from("Dracula");
    // Load and watch themes from ./themes directory
    if let Err(err) = ThemeRegistry::watch_dir(PathBuf::from("./themes"), cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    }) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }
}
