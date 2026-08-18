use std::time::Duration;

use iced::{
    Element, Fill, Font, Subscription, Task,
    widget::{column, container, row, space, text},
};
use system::memory::Memory;
use system::monitor::{SystemMonitor, SystemSnapshot};
use tracing::error;

use crate::{
    config::Config,
    screen::{
        charts::{self, ChartsScreen},
        processes::{self, ProcessesScreen},
        settings::{self, SettingsScreen},
    },
    state::history::{DiskHistory, History},
    widgets::{
        shell,
        sidebar::{self, Sidebar},
    },
};

const POLL_INTERVAL: Duration = Duration::from_millis(500);

#[derive(Debug)]
pub struct App {
    pub config: Config,
    monitor: Option<Box<SystemMonitor>>,
    snapshot: SystemSnapshot,
    cpu_history: History<f32>,
    memory_history: History<Memory>,
    disks_history: Vec<DiskHistory>,
    current_screen: Screen,
    processes: ProcessesScreen,
    settings: SettingsScreen,
    charts: ChartsScreen,
    sidebar: Sidebar,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Processes,
    Charts,
    Settings,
}

#[derive(Debug)]
pub enum Message {
    Sidebar(sidebar::Message),
    Processes(processes::Message),
    Charts(charts::Message),
    Settings(settings::Message),
    PollRequested,
    MonitorUpdated {
        monitor: Box<SystemMonitor>,
        snapshot: SystemSnapshot,
    },
}

impl App {
    pub fn new() -> Self {
        let monitor = SystemMonitor::default();
        let snapshot = monitor.snapshot();
        let config = confy::load("taskforge", None).unwrap_or_else(|e| {
            error!("Failed to load config, using default: {:#?}", e);
            Config::default()
        });

        Self {
            monitor: Some(Box::new(monitor)),
            cpu_history: History::new(),
            memory_history: History::new(),
            disks_history: Vec::new(),
            snapshot,
            current_screen: Screen::default(),
            processes: ProcessesScreen::default(),
            charts: ChartsScreen::default(),
            settings: SettingsScreen,
            config,
            sidebar: Sidebar::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::Navigate(screen)) => {
                self.current_screen = screen;
            }
            Message::Sidebar(m) => self.sidebar.update(m),
            Message::Processes(message) => match self.processes.update(message) {
                processes::Action::None => {}
                processes::Action::KillProcess(pid) => {
                    if let Some(monitor) = self.monitor.as_mut() {
                        monitor.kill_process(pid);
                        monitor.refresh();
                        self.snapshot = monitor.snapshot();
                    }
                }
            },
            Message::Charts(message) => self.charts.update(message),
            Message::Settings(settings::Message::ThemeChange(t)) => {
                self.config.theme = t;
                confy::store("taskforge", None, self.config.clone())
                    .unwrap_or_else(|e| error!("Failed to load config: {:#?}", e));
            }
            Message::PollRequested => {
                let Some(mut monitor) = self.monitor.take() else {
                    return Task::none();
                };

                return Task::perform(
                    async move {
                        monitor.refresh();
                        let snapshot = monitor.snapshot();

                        (monitor, snapshot)
                    },
                    |(monitor, snapshot)| Message::MonitorUpdated { monitor, snapshot },
                );
            }
            Message::MonitorUpdated { monitor, snapshot } => {
                self.monitor = Some(monitor);
                self.cpu_history.push_back(snapshot.cpu_usage.total);
                self.memory_history.push_back(snapshot.memory_usage);
                for disk in &snapshot.disks {
                    if let Some(history) = self
                        .disks_history
                        .iter_mut()
                        .find(|history| history.name == disk.name)
                    {
                        history.usage.push_back(disk.usage);
                    } else {
                        self.disks_history
                            .push(DiskHistory::new(disk.name.clone(), disk.usage));
                    }
                }
                self.snapshot = snapshot;
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let theme = iced::Theme::from(self.config.theme.clone());
        let sidebar = self.sidebar.view(self.current_screen).map(Message::Sidebar);

        let screen = match self.current_screen {
            Screen::Processes => self.processes.view(&self.snapshot).map(Message::Processes),
            Screen::Charts => self
                .charts
                .view(
                    &self.cpu_history,
                    &self.memory_history,
                    &self.disks_history,
                    self.snapshot.total_memory,
                    &theme,
                )
                .map(Message::Charts),
            Screen::Settings => self
                .settings
                .view(&self.config.theme)
                .map(Message::Settings),
        };

        container(row![
            sidebar,
            column![
                self.header(),
                container(screen)
                    .width(Fill)
                    .height(Fill)
                    .clip(true)
                    .style(shell::content)
            ]
            .width(Fill)
            .height(Fill)
        ])
        .width(Fill)
        .height(Fill)
        .style(shell::background)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(POLL_INTERVAL).map(|_| Message::PollRequested),
            self.sidebar.subscription().map(Message::Sidebar),
            self.processes.subscription().map(Message::Processes),
        ])
    }

    fn header(&self) -> Element<'_, Message> {
        let mut header = row![
            text(match self.current_screen {
                Screen::Processes => "Processes",
                Screen::Charts => "Charts",
                Screen::Settings => "Settings",
            })
            .size(24)
            .font(Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            })
        ]
        .padding(8)
        .width(Fill);

        if self.current_screen == Screen::Processes {
            header = header.push(space::horizontal());
            header = header.push(self.processes.header_actions().map(Message::Processes));
        }

        header.into()
    }
}
