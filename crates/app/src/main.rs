mod view;

use crate::view::root::RootView;
use gpui::{AppContext, WindowOptions};
use gpui_component::Root;

fn main() -> anyhow::Result<()> {
    gpui_platform::application().run(move |cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|cx| RootView::new(window, cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });

    Ok(())
}
