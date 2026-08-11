use std::time::Duration;

use iced::{
    Element, Fill, Subscription, Task, Theme,
    widget::{container, row},
};
use system::monitor::{SystemMonitor, SystemSnapshot};
use tracing::error;

use crate::{
    screen::{
        charts::{self, ChartsScreen},
        process::{self, ProcessScreen},
        settings::{self, SettingsScreen},
    },
    widgets::sidebar,
};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct App {
    pub current_theme: Theme,
    monitor: Option<Box<SystemMonitor>>,
    snapshot: SystemSnapshot,
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

        Self {
            monitor: Some(Box::new(monitor)),
            snapshot,
            current_screen: Screen::default(),
            processes: ProcessScreen::default(),
            charts: ChartsScreen,
            settings: SettingsScreen,
            current_theme: Theme::Dracula,
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
            Message::Settings(settings::Message::ThemeChange(t)) => self.current_theme = t,
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
                self.snapshot = snapshot;
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let sidebar = sidebar::view(self.current_screen).map(Message::Sidebar);

        let screen = match self.current_screen {
            Screen::Processes => self.processes.view(&self.snapshot).map(Message::Processes),
            Screen::Charts => self.charts.view(&self.snapshot).map(Message::Charts),
            Screen::Settings => self
                .settings
                .view(&self.current_theme)
                .map(Message::Settings),
        };

        row![sidebar, container(screen).width(Fill).height(Fill)]
            .width(Fill)
            .height(Fill)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        iced::time::every(POLL_INTERVAL).map(|_| Message::PollRequested)
    }
}
