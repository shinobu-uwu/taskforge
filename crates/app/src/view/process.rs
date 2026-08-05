use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Task, Window,
    div, relative,
};
use gpui_component::{
    ActiveTheme, Icon, IndexPath, Sizable, StyledExt,
    label::Label,
    list::{List, ListDelegate, ListItem, ListState},
    white,
};
use sysinfo::Pid;
use system::monitor::SystemSnapshot;

use crate::{
    history::History,
    widgets::{
        chart::{cpu_chart, memory_chart},
        icon::FontAwesomeIconName,
    },
};

#[derive(Debug)]
pub struct ProcessView {
    cpu_history: Entity<History<f32>>,
    memory_history: Entity<History<u64>>,
    list_state: Entity<ListState<ProcessListDelegate>>,
}

#[derive(Debug)]
pub struct ProcessListDelegate {
    snapshot: Entity<SystemSnapshot>,
    selected_index: Option<IndexPath>,
    filtered: Vec<Pid>,
    query: String,
}

impl ProcessView {
    pub fn new(
        snapshot: Entity<SystemSnapshot>,
        cpu_history: Entity<History<f32>>,
        memory_history: Entity<History<u64>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut filtered = snapshot
            .read(cx)
            .processes
            .iter()
            .map(|p| p.pid)
            .collect::<Vec<Pid>>();
        filtered.sort_unstable();
        let delegate = ProcessListDelegate {
            snapshot: snapshot.clone(),
            filtered,
            selected_index: None,
            query: String::new(),
        };
        let list_state = cx.new(|cx| ListState::new(delegate, window, cx).searchable(true));

        cx.observe(&snapshot, {
            let list_state = list_state.clone();
            move |_this, _snapshot, cx| {
                list_state.update(cx, |state, cx| {
                    state.delegate_mut().filter(cx);
                });
            }
        })
        .detach();

        Self {
            list_state,
            cpu_history,
            memory_history,
        }
    }
}

impl Render for ProcessView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .h_flex()
            .size_full()
            .child(
                div()
                    .flex_basis(relative(3. / 4.))
                    .size_full()
                    .child(List::new(&self.list_state)),
            )
            .child(
                div()
                    .flex()
                    .v_flex()
                    .h_full()
                    .flex_basis(relative(1. / 4.))
                    .gap_4()
                    .p_4()
                    .border_l_1()
                    .border_color(white())
                    .child(Label::new("Performance").text_lg())
                    .child(
                        div()
                            .flex()
                            .flex_1()
                            .v_flex()
                            .justify_evenly()
                            .items_center()
                            .child(
                                div()
                                    .w_full()
                                    .h_48()
                                    .child(cpu_chart(&self.cpu_history, cx)),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .h_48()
                                    .child(memory_chart(&self.memory_history, cx)),
                            )
                            .child(
                                div()
                                    .w_full()
                                    .h_48()
                                    .child(cpu_chart(&self.cpu_history, cx)),
                            )
                            .child(
                                div()
                                    .p_4()
                                    .border_1()
                                    .border_color(white())
                                    .rounded_lg()
                                    .bg(cx.theme().accent)
                                    .child("CPU Usage")
                                    .child("Memory Usage")
                                    .child("Uptime"),
                            ),
                    ),
            )
    }
}

impl ListDelegate for ProcessListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, _cx: &App) -> usize {
        self.filtered.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let snapshot = self.snapshot.read(cx);

        let pid = self.filtered[ix.row];

        let process = snapshot.processes.iter().find(|p| p.pid == pid)?;

        Some(
            ListItem::new(ix)
                .border_b_1()
                .border_color(cx.theme().border.opacity(0.5))
                .py_4()
                .gap_2()
                .selected(Some(ix) == self.selected_index)
                .child(
                    div()
                        .h_flex()
                        .gap_2()
                        .child(Icon::new(FontAwesomeIconName::RegularHdd).large())
                        .child(Label::new(process.name.clone())),
                ),
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

    fn perform_search(
        &mut self,
        query: &str,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Task<()> {
        self.query = query.to_string();
        self.filter(cx);
        Task::ready(())
    }
}

impl ProcessListDelegate {
    fn filter(&mut self, cx: &mut Context<ListState<Self>>) {
        let snapshot = self.snapshot.read(cx);
        self.filtered.clear();

        if self.query.is_empty() {
            self.filtered
                .extend(snapshot.processes.iter().map(|p| p.pid));
        } else {
            let needle = self.query.to_ascii_lowercase();
            self.filtered.extend(
                snapshot
                    .processes
                    .iter()
                    .filter(|p| p.name.to_ascii_lowercase().contains(&needle))
                    .map(|p| p.pid),
            );
        }

        cx.notify();
    }
}
