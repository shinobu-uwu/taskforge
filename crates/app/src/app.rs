use std::time::Duration;

use circular_buffer::FixedCircularBuffer;
use iced::{
    Element, Fill, Subscription, Task,
    widget::{column, container, row},
};
use system::monitor::{SystemMonitor, SystemSnapshot};
use tracing::error;

use crate::{
    config::Config,
    screen::{
        charts::{self, ChartsScreen},
        process::{self, ProcessScreen},
        settings::{self, SettingsScreen},
    },
    widgets::{footer, sidebar},
};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct App {
    pub config: Config,
    monitor: Option<Box<SystemMonitor>>,
    snapshot: SystemSnapshot,
    cpu_history: FixedCircularBuffer<f32, 128>,
    memory_history: FixedCircularBuffer<u64, 128>,
    current_screen: Screen,
    processes: ProcessScreen,
    settings: SettingsScreen,
    charts: ChartsScreen,
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
    Footer(footer::Message),
    Processes(process::Message),
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
            cpu_history: FixedCircularBuffer::new(),
            memory_history: FixedCircularBuffer::new(),
            snapshot,
            current_screen: Screen::default(),
            processes: ProcessScreen::default(),
            charts: ChartsScreen,
            settings: SettingsScreen,
            config,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::Navigate(screen)) => {
                self.current_screen = screen;
            }
            Message::Sidebar(sidebar::Message::OpenWebsite) => {
                open::that("https://github.com/shinobu-uwu/taskforge")
                    .unwrap_or_else(|e| error!("Error opening home URL: {:#?}", e));
            }
            Message::Footer(m) => footer::update(m),
            Message::Processes(message) => match self.processes.update(message) {
                process::Action::None => {}
                process::Action::KillProcess(pid) => {
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
                self.cpu_history.push_back(snapshot.cpu_usage);
                self.memory_history.push_back(snapshot.memory_usage);
                self.snapshot = snapshot;
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar::view(self.current_screen).map(Message::Sidebar);
        let footer = footer::view(
            self.snapshot.processes.len(),
            self.snapshot.uptime,
            self.snapshot.memory_usage,
            self.snapshot.total_memory,
        )
        .map(Message::Footer);

        let screen = match self.current_screen {
            Screen::Processes => self.processes.view(&self.snapshot).map(Message::Processes),
            Screen::Charts => self
                .charts
                .view(
                    &self.cpu_history,
                    &self.memory_history,
                    self.snapshot.total_memory,
                )
                .map(Message::Charts),
            Screen::Settings => self
                .settings
                .view(&self.config.theme)
                .map(Message::Settings),
        };

        column![
            row![sidebar, container(screen).width(Fill).height(Fill)],
            footer
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([iced::time::every(POLL_INTERVAL).map(|_| Message::PollRequested)])
    }
}
