use iced::{
    Center, Element, Fill,
    widget::{button, column, row, rule, scrollable, space, text, text_input},
};
use iced_fonts::lucide::advanced_text::search;
use sysinfo::Pid;
use system::monitor::SystemSnapshot;

use crate::widgets::icon::IntoTextInputIcon;

#[derive(Debug, Default)]
pub(crate) struct ProcessScreen {
    search_query: String,
    selected_process: Option<Pid>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    SearchChanged(String),
    ProcessSelected(Pid),
    KillProcess(Pid),
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum Action {
    None,
    KillProcess(Pid),
}

impl ProcessScreen {
    pub(crate) fn update(&mut self, message: Message) -> Action {
        match message {
            Message::SearchChanged(query) => {
                self.search_query = query;
                Action::None
            }
            Message::ProcessSelected(pid) => {
                self.selected_process = Some(pid);
                Action::None
            }
            Message::KillProcess(pid) => Action::KillProcess(pid),
        }
    }

    pub(crate) fn view(&self, snapshot: &SystemSnapshot) -> Element<'_, Message> {
        let mut pids = snapshot
            .processes
            .values()
            .filter(|p| p.thread_kind.is_none())
            .filter(|p| {
                p.name
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
            })
            .map(|p| p.pid)
            .collect::<Vec<_>>();
        pids.sort_unstable();

        column![
            row![
                text("Processes").size(32),
                space::horizontal(),
                text_input("Search processes...", &self.search_query)
                    .icon(search().into_text_input_icon())
                    .on_input(Message::SearchChanged)
                    .width(240),
            ]
            .width(Fill)
            .align_y(Center)
            .padding(8),
            rule::horizontal(1),
            scrollable(column(pids.into_iter().map(|pid| {
                let process = snapshot.processes.get(&pid).expect("Cannot find process");

                button(row![
                    iced_fonts::lucide::cpu(),
                    text(process.name.clone()),
                    button(iced_fonts::lucide::x()).on_press(Message::KillProcess(pid))
                ])
                .width(Fill)
                .style(if self.selected_process == Some(pid) {
                    button::primary
                } else {
                    button::text
                })
                .on_press(Message::ProcessSelected(pid))
                .into()
            })))
            .width(Fill)
            .height(Fill),
        ]
        .into()
    }
}
