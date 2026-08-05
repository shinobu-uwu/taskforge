use std::time::Duration;

use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};
use gpui_component::{ActiveTheme, StyledExt};
use system::monitor::{SystemMonitor, SystemSnapshot};

use crate::{
    history::History,
    state::CurrentView,
    view::{charts::ChartsView, process::ProcessView},
    widgets::sidebar::{SideBar, SideBarEvent},
};

#[derive(Debug)]
pub struct RootView {
    current_view: CurrentView,
    sidebar: Entity<SideBar>,
    process_list: Entity<ProcessView>,
    charts_view: Entity<ChartsView>,
}

impl RootView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let monitor = SystemMonitor::default();
        let snapshot = cx.new(|_cx| monitor.snapshot());
        let cpu_history = cx.new(|_cx| History::new());
        let memory_history = cx.new(|_cx| History::new());
        let sidebar = cx.new(|_| SideBar::new());
        cx.subscribe(&sidebar, |this, _sidebar, event, cx| {
            let SideBarEvent::Selected(view) = event;
            this.current_view = *view;
            cx.notify();
        })
        .detach();
        let process_list = cx.new(|cx| {
            ProcessView::new(
                snapshot.clone(),
                cpu_history.clone(),
                memory_history.clone(),
                window,
                cx,
            )
        });
        let charts_view = cx.new(|_| ChartsView::new(cpu_history.clone(), memory_history.clone()));

        Self::start_polling(
            snapshot.clone(),
            cpu_history.clone(),
            memory_history.clone(),
            cx,
        );

        Self {
            current_view: CurrentView::default(),
            sidebar,
            process_list,
            charts_view,
        }
    }

    fn start_polling(
        snapshot: Entity<SystemSnapshot>,
        cpu_history: Entity<History<f32>>,
        memory_history: Entity<History<u64>>,
        cx: &mut Context<Self>,
    ) {
        cx.spawn(async move |_this, cx| {
            loop {
                cx.background_executor().timer(Duration::from_secs(1)).await;
                let new_snapshot = cx
                    .background_executor()
                    .spawn(async {
                        let monitor = SystemMonitor::default();
                        monitor.snapshot()
                    })
                    .await;
                let cpu_usage = new_snapshot.cpu_usage;
                let memory_usage = new_snapshot.memory_usage;
                snapshot.update(cx, move |snapshot, cx| {
                    *snapshot = new_snapshot;
                    cx.notify();
                });
                cpu_history.update(cx, move |cpu_history, cx| {
                    cpu_history.push_back(cpu_usage);
                    cx.notify();
                });
                memory_history.update(cx, move |memory_history, cx| {
                    memory_history.push_back(memory_usage);
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
            .h_flex()
            .size_full()
            .bg(cx.theme().background)
            .child(self.sidebar.clone())
            .child(match self.current_view {
                CurrentView::Processes => self.process_list.clone().into_any_element(),
                CurrentView::Charts => self.charts_view.clone().into_any_element(),
            })
    }
}
