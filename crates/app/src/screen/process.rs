use iced::{
    Center, Element, Fill,
    widget::{
        button, column, container, opaque, row, rule, scrollable, space, stack, text, text_input,
    },
};
use iced_fonts::lucide::{self, advanced_text::search};
use sysinfo::Pid;
use system::monitor::SystemSnapshot;

use crate::widgets::{
    button::{icon_button, rounded},
    icon::IntoTextInputIcon,
};

#[derive(Debug, Default)]
pub(crate) struct ProcessScreen {
    search_query: String,
    selected_process: Option<Pid>,
    pid_to_kill: Option<Pid>,
}

#[derive(Debug, Clone)]
pub(crate) enum Message {
    SearchChanged(String),
    ProcessSelected(Pid),
    KillProcess(Pid),
    ShowDialog(Pid),
    DismissDialog,
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
            Message::KillProcess(pid) => {
                self.pid_to_kill = None;
                Action::KillProcess(pid)
            }
            Message::ShowDialog(pid) => {
                self.pid_to_kill = Some(pid);
                Action::None
            }
            Message::DismissDialog => {
                self.pid_to_kill = None;
                Action::None
            }
        }
    }

    pub(crate) fn view(&self, snapshot: &SystemSnapshot) -> Element<'_, Message> {
        let mut pids = snapshot
            .processes
            .values()
            .filter(|p| {
                p.name
                    .to_lowercase()
                    .contains(&self.search_query.to_lowercase())
            })
            .map(|p| p.pid)
            .collect::<Vec<_>>();
        pids.sort_unstable();

        let mut stack = stack![column![
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
                let is_selected = self.selected_process == Some(pid);

                column![
                    button(
                        row![
                            lucide::cpu(),
                            text(process.name.clone()).width(Fill),
                            icon_button(lucide::x()).on_press(Message::ShowDialog(pid))
                        ]
                        .align_y(Center)
                        .spacing(8)
                        .padding(8),
                    )
                    .width(Fill)
                    .style(if is_selected {
                        button::primary
                    } else {
                        button::text
                    })
                    .on_press(Message::ProcessSelected(pid)),
                    rule::horizontal(1)
                ]
                .into()
            })))
            .width(Fill)
            .height(Fill),
        ]];

        if let Some((pid, process)) = self
            .pid_to_kill
            .and_then(|pid| snapshot.processes.get(&pid).map(|process| (pid, process)))
        {
            let dialog = container(
                column![
                    row![
                        text("Kill process?").size(32),
                        space::horizontal(),
                        icon_button(lucide::x().size(24)).on_press(Message::DismissDialog),
                    ]
                    .align_y(Center)
                    .padding(12),
                    rule::horizontal(1),
                    column![
                        text(format!(
                            "Are you sure you want to kill {} (pid {})?",
                            process.name, pid
                        )),
                        text("This action cannot be reverted").style(text::danger)
                    ]
                    .padding(16),
                    rule::horizontal(1),
                    row![
                        button("Cancel")
                            .style(rounded(button::text))
                            .on_press(Message::DismissDialog),
                        space::horizontal(),
                        button("Kill")
                            .style(rounded(button::danger))
                            .on_press(Message::KillProcess(pid)),
                    ]
                    .spacing(8)
                    .padding(8),
                ]
                .width(Fill),
            )
            .width(420)
            .style(container::rounded_box);

            stack = stack.push(opaque(
                container(dialog)
                    .center(Fill)
                    .width(Fill)
                    .height(Fill)
                    .style(|theme: &iced::Theme| {
                        let mut background = theme.palette().background;
                        background.a = 0.8;

                        container::Style::default().background(background)
                    }),
            ));
        }

        stack.into()
    }
}
