use iced::{
    Color, Element, Fill,
    Length::Shrink,
    Theme, border,
    widget::{button, column, container, row, rule, scrollable, space, text},
};

use crate::{
    state::history::{DiskHistory, History, TimedSample},
    widgets::chart::{ChartSettings, cpu_chart, disk_chart, memory_chart},
};
use system::{memory::Memory, snapshot::SystemSnapshot};

#[derive(Debug, Default)]
pub struct ChartsScreen {
    selected_chart: Chart,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectChart(Chart),
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Chart {
    #[default]
    Cpu,
    Memory,
    Disk(String),
}

#[derive(Debug, Clone, Copy)]
enum ChartColor {
    Primary,
    Success,
    Warning,
}

impl ChartColor {
    fn resolve(self, theme: &Theme) -> Color {
        let palette = theme.palette();

        match self {
            Self::Primary => palette.primary,
            Self::Success => palette.success,
            Self::Warning => palette.warning,
        }
    }
}

impl Chart {
    const fn color(&self) -> ChartColor {
        match self {
            Self::Cpu => ChartColor::Primary,
            Self::Memory => ChartColor::Success,
            Self::Disk(_) => ChartColor::Warning,
        }
    }
}

impl ChartsScreen {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectChart(c) => self.selected_chart = c,
        }
    }

    pub fn view<'a>(
        &'a self,
        snapshot: &SystemSnapshot,
        cpu_history: &'a History<TimedSample<f32>>,
        memory_history: &'a History<TimedSample<Memory>>,
        disks_history: &'a [DiskHistory],
        total_memory: Memory,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let cpu_title = format!("CPU {:.0}%", snapshot.cpu_usage.total);
        let mut charts = column![
            self.chart_button(
                cpu_title,
                cpu_chart(cpu_history, theme, self.preview_chart_settings(),),
                Chart::Cpu,
            ),
            self.chart_button(
                "Memory",
                memory_chart(
                    memory_history,
                    total_memory,
                    theme,
                    self.preview_chart_settings(),
                ),
                Chart::Memory
            )
        ];

        for disk in disks_history {
            charts = charts.push(self.chart_button(
                &disk.name,
                disk_chart(&disk.usage, theme, self.preview_chart_settings()),
                Chart::Disk(disk.name.clone()),
            ));
        }

        row![
            scrollable(charts.spacing(8).width(240).height(Fill)).spacing(8),
            self.cpu_content(snapshot, cpu_history, theme)
        ]
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn chart_button<'a>(
        &'a self,
        label: impl text::IntoFragment<'a>,
        chart_element: Element<'a, Message>,
        chart: Chart,
    ) -> Element<'a, Message> {
        let is_selected = self.selected_chart == chart;
        let chart_color = chart.color();

        button(column![text(label).size(20), chart_element].spacing(4))
            .style(move |theme, status| {
                let mut base = button::subtle(theme, status);
                let palette = theme.palette();

                if is_selected {
                    base.border = border::rounded(8)
                        .color(chart_color.resolve(theme))
                        .width(1)
                } else {
                    base.border = border::rounded(8).color(palette.background).width(1)
                }

                base
            })
            .height(120)
            .on_press(Message::SelectChart(chart))
            .into()
    }

    fn cpu_content<'a>(
        &'a self,
        snapshot: &SystemSnapshot,
        history: &'a History<TimedSample<f32>>,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let secs = snapshot.uptime.as_secs();
        let days = secs / (24 * 60 * 60);
        let hours = (secs % (24 * 60 * 60)) / (60 * 60);
        let minutes = (secs % (60 * 60)) / 60;
        let seconds = secs % 60;

        let summary = column![
            row![
                self.content_field("Usage", format!("{:.0}%", snapshot.cpu_usage.total)),
                space::horizontal(),
                self.content_field(
                    "Frequency",
                    format!("{:.2} GHz", snapshot.cpu_usage.frequency as f64 / 1000.0)
                ),
            ],
            rule::horizontal(1),
            row![
                self.content_field("Processes", snapshot.processes.len()),
                self.content_field(
                    "Threads",
                    match snapshot.logical_processor_count {
                        Some(t) => t.to_string(),
                        None => "Unknown".to_string(),
                    }
                ),
                self.content_field(
                    "File descriptors",
                    match snapshot.descriptors_count {
                        Some(d) => d.to_string(),
                        None => "Unknown".to_string(),
                    }
                )
            ]
            .spacing(8),
            rule::horizontal(1),
            self.content_field(
                "Uptime",
                format!("{}d {}:{:02}:{:02}", days, hours, minutes, seconds)
            )
        ]
        .padding(8)
        .spacing(8);

        let details = row![
            column![
                text("Base speed").style(text::secondary),
                text("Sockets").style(text::secondary),
                text("Cores").style(text::secondary),
                text("Logical processors").style(text::secondary),
                text("Virtualization").style(text::secondary),
                text("L1 cache").style(text::secondary),
                text("L2 cache").style(text::secondary),
                text("L3 cache").style(text::secondary),
            ]
            .spacing(4),
            column![
                text("3.60 GHz"),
                text("1"),
                text(match snapshot.core_count {
                    Some(c) => c.to_string(),
                    None => "Unknown".to_string(),
                }),
                text(match snapshot.logical_processor_count {
                    Some(c) => c.to_string(),
                    None => "Unknown".to_string(),
                }),
                text("Enabled"),
                text("640 KB"),
                text("10.0 MB"),
                text("32.0 MB"),
            ]
            .spacing(4),
        ]
        .spacing(24)
        .padding(8);

        let system_details = row![summary, rule::vertical(1), details]
            .width(Shrink)
            .height(Shrink);

        column![
            cpu_chart(history, theme, ChartSettings::detailed()),
            container(system_details).center_x(Fill),
        ]
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn content_field<'a>(
        &'a self,
        label: impl text::IntoFragment<'a>,
        content: impl text::IntoFragment<'a>,
    ) -> Element<'a, Message> {
        column![text(label).style(text::secondary), text(content).size(20),]
            .spacing(4)
            .into()
    }

    const fn preview_chart_settings(&self) -> ChartSettings {
        ChartSettings {
            axes: false,
            x_mesh: false,
            y_mesh: false,
            ..ChartSettings::compact()
        }
    }
}
