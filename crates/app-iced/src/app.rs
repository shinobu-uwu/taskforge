use std::time::Duration;

use iced::{
    Element, Fill, Subscription, Task,
    widget::{container, row},
};
use system::monitor::{SystemMonitor, SystemSnapshot};

use crate::{
    screen::{
        charts::{self, ChartsScreen},
        process::{self, ProcessScreen},
    },
    widgets::sidebar,
};

const POLL_INTERVAL: Duration = Duration::from_secs(1);

#[derive(Debug)]
pub struct App {
    monitor: Option<SystemMonitor>,
    snapshot: SystemSnapshot,
    current_screen: Screen,
    processes: ProcessScreen,
    charts: ChartsScreen,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Processes,
    Charts,
}

#[derive(Debug)]
pub enum Message {
    Sidebar(sidebar::Message),
    Processes(process::Message),
    Charts(charts::Message),
    PollRequested,
    MonitorUpdated {
        monitor: SystemMonitor,
        snapshot: SystemSnapshot,
    },
}

impl App {
    pub fn new() -> Self {
        let monitor = SystemMonitor::default();
        let snapshot = monitor.snapshot();

        Self {
            monitor: Some(monitor),
            snapshot,
            current_screen: Screen::default(),
            processes: ProcessScreen::default(),
            charts: ChartsScreen,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(sidebar::Message::Navigate(screen)) => {
                self.current_screen = screen;
            }
            Message::Sidebar(sidebar::Message::OpenWebsite) => {
                let _ = open::that("https://github.com/shinobu-uwu/taskforge");
            }
            Message::Processes(message) => self.processes.update(message),
            Message::Charts(message) => self.charts.update(message),
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
