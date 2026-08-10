use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Task, Window,
    div, relative,
};
use gpui_component::{
    ActiveTheme, Icon, IndexPath, Sizable, StyledExt,
    button::Button,
    label::Label,
    list::{List, ListDelegate, ListItem, ListState},
    white,
};
use sysinfo::{Pid, ThreadKind};
use system::monitor::{ProcessSnapshot, SystemMonitor, SystemSnapshot};

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
    monitor: Entity<SystemMonitor>,
    snapshot: Entity<SystemSnapshot>,
    selected_index: Option<IndexPath>,
    filtered: Vec<Pid>,
    query: String,
}

impl ProcessView {
    pub fn new(
        monitor: Entity<SystemMonitor>,
        snapshot: Entity<SystemSnapshot>,
        cpu_history: Entity<History<f32>>,
        memory_history: Entity<History<u64>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut filtered = snapshot
            .read(cx)
            .processes
            .values()
            .filter(|process| should_show_process(process))
            .map(|process| process.pid)
            .collect::<Vec<Pid>>();
        filtered.sort_unstable();
        let delegate = ProcessListDelegate {
            monitor,
            snapshot: snapshot.clone(),
            filtered,
            selected_index: None,
            query: String::new(),
        };
        let list_state = cx.new(|cx| ListState::new(delegate, window, cx));

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
        let &pid = self.filtered.get(ix.row)?;
        let snapshot = self.snapshot.read(cx);
        let process = snapshot.processes.get(&pid)?;
        let monitor = self.monitor.clone();
        let snapshot = self.snapshot.clone();

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
                )
                .suffix(move |_, _cx| {
                    Button::new(("kill-process", pid.as_u32() as usize))
                        .small()
                        .outline()
                        .label("Kill")
                        .tooltip("Kill process")
                        .on_click({
                            let monitor = monitor.clone();
                            let snapshot = snapshot.clone();
                            move |_, _, cx| {
                                let new_snapshot = monitor.update(cx, |monitor, _cx| {
                                    let killed = monitor.kill_process(pid);
                                    if killed {
                                        monitor.refresh();
                                        Some(monitor.snapshot())
                                    } else {
                                        None
                                    }
                                });

                                if let Some(new_snapshot) = new_snapshot {
                                    snapshot.update(cx, |snapshot, cx| {
                                        *snapshot = new_snapshot;
                                        cx.notify();
                                    });
                                }
                            }
                        })
                }),
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
        self.selected_index = None;
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
                .extend(snapshot.processes.values().filter_map(visible_process_pid));
        } else {
            let needle = self.query.to_ascii_lowercase();
            self.filtered.extend(
                snapshot
                    .processes
                    .values()
                    .filter(|p| {
                        should_show_process(p) && p.name.to_ascii_lowercase().contains(&needle)
                    })
                    .map(|p| p.pid),
            );
        }
        self.filtered.sort_unstable();
        cx.notify();
    }
}

fn visible_process_pid(process: &ProcessSnapshot) -> Option<Pid> {
    should_show_process(process).then_some(process.pid)
}

fn should_show_process(process: &ProcessSnapshot) -> bool {
    process.thread_kind != Some(ThreadKind::Kernel)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_with_thread_kind(thread_kind: Option<ThreadKind>) -> ProcessSnapshot {
        ProcessSnapshot {
            pid: Pid::from_u32(1),
            parent: None,
            name: "process".to_string(),
            cpu_usage: 0.0,
            memory: 0,
            thread_kind,
        }
    }

    #[test]
    fn hides_kernel_processes() {
        let process = process_with_thread_kind(Some(ThreadKind::Kernel));

        assert!(!should_show_process(&process));
    }

    #[test]
    fn shows_userland_and_unknown_thread_kind_processes() {
        let userland_process = process_with_thread_kind(Some(ThreadKind::Userland));
        let unknown_process = process_with_thread_kind(None);

        assert!(should_show_process(&userland_process));
        assert!(should_show_process(&unknown_process));
    }
}
