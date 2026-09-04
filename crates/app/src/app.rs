use std::time::Instant;

use iced::{
    Element, Fill, Font, Subscription, Task, border,
    theme::palette,
    widget::{column, container, row, space, stack, text},
};
use system::{cpu::CpuInfo, memory::Memory, monitor::SystemMonitor, snapshot::SystemSnapshot};
use tracing::error;

use crate::{
    config::Config,
    screen::{
        charts::{self, ChartsScreen},
        processes::{self, ProcessesScreen},
        settings::{self, SettingsScreen},
    },
    state::history::{DiskHistory, History, TimedSample},
    widgets::{
        shell,
        sidebar::{self, Sidebar},
    },
};

#[derive(Debug)]
pub struct App {
    pub config: Config,
    monitor: Option<Box<SystemMonitor>>,
    snapshot: SystemSnapshot,
    cpu_history: History<TimedSample<f32>>,
    memory_history: History<TimedSample<Memory>>,
    disks_history: Vec<DiskHistory>,
    current_screen: Screen,
    processes: ProcessesScreen,
    settings: SettingsScreen,
    charts: ChartsScreen,
    sidebar: Sidebar,
    show_fps: bool,
    fps: u32,
    frame_count: u32,
    last_fps_update: Instant,
    cpu_info: CpuInfo,
    startup_time: Instant,
    first_frame_time: Option<Instant>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Screen {
    Processes,
    #[default]
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
    Frame(Instant),
}

impl App {
    pub fn new(startup_time: Instant) -> Self {
        let monitor = SystemMonitor::default();
        let snapshot = monitor.snapshot();
        let config = confy::load("taskforge", None).unwrap_or_else(|e| {
            error!("Failed to load config, using default: {:#?}", e);
            Config::default()
        });
        let expanded_sidebar = config.expanded_sidebar;
        let show_fps = std::env::var_os("TASKFORGE_SHOW_FPS").is_some_and(|v| v == "1");
        let cpu_info = monitor.cpu_info();

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
            sidebar: Sidebar::new(expanded_sidebar),
            show_fps,
            fps: 0,
            frame_count: 0,
            last_fps_update: Instant::now(),
            cpu_info,
            startup_time,
            first_frame_time: None,
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
            Message::Settings(settings::Message::ConfigChanged(change)) => {
                self.config.apply(change);
                self.sidebar.set_expanded(self.config.expanded_sidebar);
                confy::store("taskforge", None, &self.config)
                    .unwrap_or_else(|e| error!("Failed to save config: {:#?}", e));
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
                self.cpu_history.push_back(TimedSample::new(
                    snapshot.captured_at,
                    snapshot.cpu_usage.total,
                ));
                self.memory_history.push_back(TimedSample::new(
                    snapshot.captured_at,
                    snapshot.memory_usage,
                ));
                for disk in &snapshot.disks {
                    if let Some(history) = self
                        .disks_history
                        .iter_mut()
                        .find(|history| history.name == disk.name)
                    {
                        history
                            .usage
                            .push_back(TimedSample::new(snapshot.captured_at, disk.usage));
                    } else {
                        self.disks_history.push(DiskHistory::new(
                            disk.name.clone(),
                            snapshot.captured_at,
                            disk.usage,
                        ));
                    }
                }
                self.snapshot = snapshot;
            }
            Message::Frame(now) => {
                if self.first_frame_time.is_none() {
                    dbg!(now.duration_since(self.startup_time).as_millis());
                    self.first_frame_time = Some(now)
                }

                self.frame_count += 1;

                if now.duration_since(self.last_fps_update).as_secs_f32() >= 1.0 {
                    self.fps = self.frame_count;
                    self.frame_count = 0;
                    self.last_fps_update = now;
                }
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let theme = iced::Theme::from(self.config.theme.clone());
        let sidebar = self.sidebar.view(self.current_screen).map(Message::Sidebar);

        let screen = match self.current_screen {
            Screen::Processes => self
                .processes
                .view(&self.snapshot, self.config.process_cpu_display_mode)
                .map(Message::Processes),
            Screen::Charts => self
                .charts
                .view(
                    &self.snapshot,
                    &self.cpu_info,
                    &self.cpu_history,
                    &self.memory_history,
                    &self.disks_history,
                    self.snapshot.total_memory,
                    &theme,
                )
                .map(Message::Charts),
            Screen::Settings => self.settings.view(&self.config).map(Message::Settings),
        };

        let mut stack = stack![
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
        ];

        if self.show_fps {
            let fps = container(text(format!("FPS: {}", self.fps)))
                .padding(8)
                .style(|theme: &iced::Theme| {
                    let background = theme.palette().success.scale_alpha(0.8);

                    container::Style::default()
                        .background(background)
                        .color(palette::readable(background, theme.palette().text))
                        .border(border::rounded(6))
                });

            stack = stack.push(container(fps).padding(8).align_right(Fill).align_top(Fill));
        }

        stack.into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            iced::time::every(self.config.refresh_rate.duration()).map(|_| Message::PollRequested),
            self.sidebar.subscription().map(Message::Sidebar),
            self.processes.subscription().map(Message::Processes),
            iced::window::frames().map(Message::Frame),
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
        .padding(16)
        .width(Fill);

        if self.current_screen == Screen::Processes {
            header = header.push(space::horizontal());
            header = header.push(self.processes.header_actions().map(Message::Processes));
        }

        header.into()
    }
}
