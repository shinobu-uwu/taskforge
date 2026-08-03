use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
};
use gpui_component::{
    IndexPath,
    label::Label,
    list::{List, ListDelegate, ListItem, ListState},
};
use sysinfo::Pid;
use system::monitor::SystemMonitor;

fn pid_list(monitor: &Entity<SystemMonitor>, cx: &App) -> Vec<Pid> {
    let mut pids: Vec<Pid> = monitor
        .read(cx)
        .system
        .processes()
        .keys()
        .copied()
        .collect();
    pids.sort_unstable();
    pids
}

pub struct ProcessListView {
    list_state: Entity<ListState<ProcessListDelegate>>,
}

impl ProcessListView {
    pub fn new(
        monitor: Entity<SystemMonitor>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let pids = pid_list(&monitor, cx);
        let delegate = ProcessListDelegate {
            monitor: monitor.clone(),
            pids,
            selected_index: None,
        };
        let list_state = cx.new(|cx| ListState::new(delegate, window, cx));

        cx.observe(&monitor, {
            let list_state = list_state.clone();
            move |_this, monitor, cx| {
                let pids = pid_list(&monitor, cx);
                list_state.update(cx, |state, cx| {
                    state.delegate_mut().pids = pids;
                    cx.notify();
                });
            }
        })
        .detach();

        Self { list_state }
    }
}

impl Render for ProcessListView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(List::new(&self.list_state))
    }
}

pub struct ProcessListDelegate {
    monitor: Entity<SystemMonitor>,
    pids: Vec<Pid>,
    selected_index: Option<IndexPath>,
}

impl ListDelegate for ProcessListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.pids.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let pid = *self.pids.get(ix.row)?;
        let name = self
            .monitor
            .read(cx)
            .system
            .process(pid)
            .map(|p| p.name().to_string_lossy().into_owned())
            .unwrap_or_else(|| "?".to_string());

        Some(
            ListItem::new(ix)
                .child(Label::new(name))
                .selected(Some(ix) == self.selected_index),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}
