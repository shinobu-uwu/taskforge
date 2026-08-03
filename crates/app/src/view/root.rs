use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{ActiveTheme, StyledExt, TitleBar};
use system::monitor::SystemMonitor;

use crate::view::process_list::ProcessListView;

#[derive(Debug)]
pub struct RootView {
    process_list: Entity<ProcessListView>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let monitor = cx.new(|_cx| SystemMonitor::default());
        Self::start_polling(monitor.clone(), cx);

        let process_list = cx.new(|cx| ProcessListView::new(monitor.clone(), window, cx));

        Self { process_list }
    }

    fn start_polling(monitor: Entity<SystemMonitor>, cx: &mut Context<Self>) {
        cx.spawn(async move |_this, cx| {
            loop {
                cx.background_executor()
                    .timer(std::time::Duration::from_secs(1))
                    .await;

                monitor.update(cx, |monitor, cx| {
                    monitor.poll();
                    cx.notify();
                });
            }
        })
        .detach();
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(TitleBar::new().mb_1().shadow_sm().child("Taskforge"))
            .child(self.process_list.clone())
    }
}
