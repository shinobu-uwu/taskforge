use std::{cmp::Ordering, time::Instant};

use iced::theme::palette::lighten;
use iced::widget::button::Status;
use iced::{
    Animation, Background, Center, Element, Fill,
    Length::Shrink,
    Subscription, Theme,
    animation::Easing,
    border,
    widget::{
        button, column, container, opaque, row, rule, scrollable, space, stack, text, text_input,
    },
};
use iced_fonts::lucide::{self, advanced_text::search};
use system::{
    process::Pid,
    snapshot::{ProcessSnapshot, SystemSnapshot},
};

use crate::config::ProcessCpuDisplayMode;
use crate::widgets::shell::SHELL_BORDER_RADIUS;
use crate::widgets::{
    button::{icon_button, rounded, rounded_full},
    fade,
    icon::IntoTextInputIcon,
};

const CPU_COLUMN_WIDTH: u32 = 120;
const MEMORY_COLUMN_WIDTH: u32 = 200;
const USERNAME_COLUMN_WIDTH: u32 = 200;
const HEADER_PADDING: f32 = 4.0;

#[derive(Debug)]
pub struct ProcessesScreen {
    search_query: String,
    selected_process: Option<Pid>,
    pid_to_kill: Option<Pid>,
    dialog_visible: Animation<bool>,
    now: Instant,
    sort_column: SortColumn,
    sort_direction: SortDirection,
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchChanged(String),
    ProcessSelected(Pid),
    KillProcess(Pid),
    ShowDialog(Pid),
    DismissDialog,
    Frame(Instant),
    SortBy(SortColumn),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortColumn {
    Pid,
    Name,
    Cpu,
    Memory,
    Username,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum SortDirection {
    #[default]
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    None,
    KillProcess(Pid),
}

impl ProcessesScreen {
    pub fn update(&mut self, message: Message) -> Action {
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
                self.selected_process = None;
                self.now = Instant::now();
                self.dialog_visible.go_mut(false, self.now);
                Action::KillProcess(pid)
            }
            Message::ShowDialog(pid) => {
                self.pid_to_kill = Some(pid);
                self.now = Instant::now();
                self.dialog_visible.go_mut(true, self.now);
                Action::None
            }
            Message::DismissDialog => {
                self.now = Instant::now();
                self.dialog_visible.go_mut(false, self.now);
                Action::None
            }
            Message::Frame(now) => {
                self.now = now;

                if !self.dialog_visible.value() && !self.dialog_visible.is_animating(now) {
                    self.pid_to_kill = None;
                }

                Action::None
            }
            Message::SortBy(s) => {
                if s == self.sort_column {
                    self.sort_direction = match self.sort_direction {
                        SortDirection::Ascending => SortDirection::Descending,
                        SortDirection::Descending => SortDirection::Ascending,
                    };

                    return Action::None;
                }

                self.sort_column = s;
                self.sort_direction = SortDirection::default();

                Action::None
            }
        }
    }

    pub fn view(
        &self,
        snapshot: &SystemSnapshot,
        process_cpu_display_mode: ProcessCpuDisplayMode,
    ) -> Element<'_, Message> {
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
        pids.sort_unstable_by(|left_pid, right_pid| {
            let left = &snapshot.processes[left_pid];
            let right = &snapshot.processes[right_pid];

            let ordering = match self.sort_column {
                SortColumn::Pid => left.pid.cmp(&right.pid),
                SortColumn::Name => compare_ignore_ascii_case(&left.name, &right.name),
                SortColumn::Cpu => left.cpu_usage.total_cmp(&right.cpu_usage),
                SortColumn::Memory => left.memory.cmp(&right.memory),
                SortColumn::Username => compare_ignore_ascii_case(
                    left.username.as_deref().unwrap_or("unknown"),
                    right.username.as_deref().unwrap_or("unknown"),
                ),
            };

            let ordering = match self.sort_direction {
                SortDirection::Ascending => ordering,
                SortDirection::Descending => ordering.reverse(),
            };

            ordering.then_with(|| left.pid.cmp(&right.pid))
        });

        let mut stack = stack![column![
            self.header(),
            rule::horizontal(1),
            scrollable(column(pids.into_iter().map(|pid| {
                let process = snapshot.processes.get(&pid).expect("Cannot find process");
                self.process_row(
                    process,
                    snapshot.cpu_usage.cpus.len(),
                    process_cpu_display_mode,
                )
            })))
            .width(Fill)
            .height(Fill)
        ]];

        if let Some((pid, process)) = self
            .pid_to_kill
            .and_then(|pid| snapshot.processes.get(&pid).map(|process| (pid, process)))
        {
            let opacity = self.dialog_visible.interpolate(0.0_f32, 1.0_f32, self.now);

            let dialog = container(
                column![
                    row![
                        text("Kill process?").size(32),
                        space::horizontal(),
                        icon_button(lucide::x().size(24))
                            .style(fade::button(rounded_full(button::text), opacity))
                            .on_press(Message::DismissDialog),
                    ]
                    .align_y(Center)
                    .padding(12),
                    fade::horizontal_rule(1, opacity),
                    column![
                        text(format!(
                            "Are you sure you want to kill {} (pid {})?",
                            process.name, pid
                        )),
                        text("This action cannot be reverted")
                            .style(fade::text(text::danger, opacity))
                    ]
                    .padding(16),
                    fade::horizontal_rule(1, opacity),
                    row![
                        button("Cancel")
                            .style(fade::button(rounded(button::text), opacity))
                            .on_press(Message::DismissDialog),
                        space::horizontal(),
                        button("Kill")
                            .style(fade::button(rounded(button::danger), opacity))
                            .on_press(Message::KillProcess(pid)),
                    ]
                    .spacing(8)
                    .padding(8),
                ]
                .width(Fill),
            )
            .width(420)
            .style(move |theme| {
                fade::container(
                    container::rounded_box(theme).border(border::rounded(8.0)),
                    opacity,
                )
            });

            stack = stack.push(opaque(
                container(dialog)
                    .center(Fill)
                    .width(Fill)
                    .height(Fill)
                    .style(move |theme: &Theme| {
                        container::Style::default()
                            .background(theme.palette().background.scale_alpha(0.9 * opacity))
                    }),
            ));
        }

        stack.into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.dialog_visible.is_animating(self.now) {
            iced::window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        }
    }

    pub fn header_actions(&self) -> Element<'_, Message> {
        row![
            text_input("Search processes...", &self.search_query)
                .width(240)
                .on_input(Message::SearchChanged)
                .icon(search().into_text_input_icon())
                .style(crate::widgets::text_input::primary),
            button(
                row![lucide::x().size(16), text("Kill")]
                    .align_y(Center)
                    .spacing(4)
            )
            .on_press_maybe(self.selected_process.map(Message::ShowDialog))
            .style(rounded(button::primary))
        ]
        .align_y(Center)
        .spacing(8)
        .into()
    }

    fn header(&self) -> Element<'_, Message> {
        let header = row![
            button(self.header_label("Name", SortColumn::Name))
                .width(Fill)
                .style(|theme, status| self.header_button(theme, status, SortColumn::Name))
                .padding(HEADER_PADDING)
                .on_press(Message::SortBy(SortColumn::Name)),
            rule::vertical(1),
            button(self.header_label("CPU", SortColumn::Cpu))
                .width(CPU_COLUMN_WIDTH)
                .style(|theme, status| self.header_button(theme, status, SortColumn::Cpu))
                .padding(HEADER_PADDING)
                .on_press(Message::SortBy(SortColumn::Cpu)),
            rule::vertical(1),
            button(self.header_label("Memory", SortColumn::Memory))
                .width(MEMORY_COLUMN_WIDTH)
                .style(|theme, status| self.header_button(theme, status, SortColumn::Memory))
                .padding(HEADER_PADDING)
                .on_press(Message::SortBy(SortColumn::Memory)),
        ];

        #[cfg(not(target_os = "windows"))]
        let header = header.push(rule::vertical(1)).push(
            button(self.header_label("Username", SortColumn::Username))
                .width(USERNAME_COLUMN_WIDTH)
                .style(|theme, status| self.header_button(theme, status, SortColumn::Username))
                .padding(HEADER_PADDING)
                .on_press(Message::SortBy(SortColumn::Username)),
        );

        header.height(Shrink).width(Fill).into()
    }

    fn header_label(&self, label: &'static str, column: SortColumn) -> Element<'_, Message> {
        const FONT_SIZE: f32 = 14.0;
        let mut content = row![text(label).size(FONT_SIZE)].spacing(4).align_y(Center);

        if self.sort_column == column {
            let icon = match self.sort_direction {
                SortDirection::Ascending => lucide::arrow_up_narrow_wide(),
                SortDirection::Descending => lucide::arrow_down_wide_narrow(),
            };

            content = content.push(icon.size(16));
        }

        content.into()
    }

    fn header_button(
        &self,
        theme: &Theme,
        status: button::Status,
        column: SortColumn,
    ) -> button::Style {
        let mut base = button::text(theme, status);

        if column == SortColumn::Name {
            base.border = border::rounded(border::top_left(SHELL_BORDER_RADIUS));
        }

        if self.sort_column == column {
            let palette = theme.palette();
            return base.with_background(Background::Color(lighten(palette.background, 0.05)));
        }

        base
    }

    fn process_row(
        &self,
        process: &ProcessSnapshot,
        cpus_len: usize,
        process_cpu_display_mode: ProcessCpuDisplayMode,
    ) -> Element<'_, Message> {
        const FONT_SIZE: f32 = 14.0;
        const CONTAINER_PADDING: [f32; 2] = [0.0, 8.0];
        let is_selected = self.selected_process == Some(process.pid);
        let row = row![
            container(text(process.name.clone()).size(FONT_SIZE))
                .padding(CONTAINER_PADDING)
                .width(Fill),
            container(
                text(format!(
                    "{:.2}%",
                    match process_cpu_display_mode {
                        ProcessCpuDisplayMode::TotalCapacity => {
                            process.cpu_usage / cpus_len as f32
                        }
                        ProcessCpuDisplayMode::PerCore => process.cpu_usage,
                    }
                ))
                .size(FONT_SIZE)
            )
            .padding(CONTAINER_PADDING)
            .width(CPU_COLUMN_WIDTH),
            container(text(format!("{:.2}MB", process.memory.as_mib_f64())).size(FONT_SIZE))
                .padding(CONTAINER_PADDING)
                .width(MEMORY_COLUMN_WIDTH),
        ]
        .padding([12, 0])
        .align_y(Center)
        .width(Fill);

        #[cfg(not(target_os = "windows"))]
        let row = row.push(
            container(text(process.username.clone().unwrap_or("-".to_owned())).size(FONT_SIZE))
                .padding(CONTAINER_PADDING)
                .width(USERNAME_COLUMN_WIDTH),
        );

        button(row)
            .padding(0)
            .width(Fill)
            .style(if is_selected {
                button::primary
            } else {
                process_row
            })
            .on_press(Message::ProcessSelected(process.pid))
            .into()
    }
}

fn process_row(theme: &Theme, status: Status) -> button::Style {
    let palette = theme.extended_palette();
    let base = button::text(theme, status);

    match status {
        Status::Active => base,
        Status::Hovered => base.with_background(palette.background.stronger.color),
        Status::Pressed => base.with_background(palette.background.weaker.color),
        Status::Disabled => base,
    }
}

fn compare_ignore_ascii_case(left: &str, right: &str) -> Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase())
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase()))
}

impl Default for ProcessesScreen {
    fn default() -> Self {
        Self {
            search_query: String::new(),
            selected_process: None,
            pid_to_kill: None,
            dialog_visible: Animation::new(false).very_quick().easing(Easing::EaseOut),
            now: Instant::now(),
            sort_column: SortColumn::Pid,
            sort_direction: SortDirection::Descending,
        }
    }
}
