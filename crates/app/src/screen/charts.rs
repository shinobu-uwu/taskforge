use iced::{
    Color, Element, Fill, Font,
    Length::Shrink,
    Theme, border,
    widget::{Text, button, column, container, row, rule, scrollable, text},
};

use crate::{
    state::history::{DiskHistory, History, TimedSample},
    widgets::chart::{ChartSettings, cpu_chart, disk_chart, memory_chart},
};
use system::{cpu::CpuInfo, memory::Memory, snapshot::SystemSnapshot};

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
        cpu_info: &'a CpuInfo,
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
            self.cpu_content(snapshot, cpu_info, cpu_history, theme)
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
        cpu_info: &'a CpuInfo,
        history: &'a History<TimedSample<f32>>,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let secs = snapshot.uptime.as_secs();
        let days = secs / (24 * 60 * 60);
        let hours = (secs % (24 * 60 * 60)) / (60 * 60);
        let minutes = (secs % (60 * 60)) / 60;
        let seconds = secs % 60;
        let system_details = column![
            row![
                column![
                    self.label("Usage"),
                    self.label("Speed"),
                    self.label("Processes"),
                    self.label("Threads"),
                    self.label("Handles"),
                ]
                .spacing(4)
                .width(160),
                column![
                    self.value(format!("{:.0}%", snapshot.cpu_usage.total)),
                    self.value(format!("{:.2}GHz", snapshot.cpu_usage.frequency.ghz_f64())),
                    self.value(snapshot.processes.len()),
                    self.value(match snapshot.thread_count {
                        Some(t) => t.to_string(),
                        None => "Unknown".to_string(),
                    }),
                    self.value(match snapshot.handle_count {
                        Some(h) => h.to_string(),
                        None => "Unknown".to_string(),
                    }),
                ]
                .spacing(4),
            ]
            .spacing(16),
            rule::horizontal(1),
            row![
                column![
                    self.label("Base speed"),
                    self.label("Sockets"),
                    self.label("Logical processors"),
                    self.label("Virtualization"),
                ]
                .spacing(4)
                .width(160),
                column![
                    self.value(format!(
                        "{:.2}GHz",
                        match cpu_info.base_frequency {
                            Some(f) => format!("{:.2}GHz", f.ghz_f64()),
                            None => "Uknown".to_string(),
                        }
                    )),
                    self.value(match cpu_info.socket_count {
                        Some(s) => s.to_string(),
                        None => "Unknown".to_string(),
                    }),
                    self.value(match snapshot.logical_processor_count {
                        Some(n) => n.to_string(),
                        None => "Unknown".to_string(),
                    }),
                    self.value(match cpu_info.virtualization_enabled {
                        Some(v) =>
                            if v {
                                "Enabled".to_string()
                            } else {
                                "Disabled".to_string()
                            },
                        None => "Unknown".to_string(),
                    }),
                ]
                .spacing(4),
            ]
            .spacing(16),
        ]
        .spacing(8)
        .width(Shrink);

        column![
            text(cpu_info.name.as_deref().unwrap_or("Unknown"))
                .size(24)
                .font(Font {
                    weight: iced::font::Weight::Semibold,
                    ..Default::default()
                }),
            row![
                cpu_chart(history, theme, ChartSettings::detailed()),
                container(system_details),
            ]
            .spacing(4),
        ]
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn label<'a>(&'a self, label: impl text::IntoFragment<'a>) -> Text<'a> {
        text(label).style(text::secondary).size(16)
    }

    fn value<'a>(&'a self, value: impl text::IntoFragment<'a>) -> Text<'a> {
        text(value).size(16)
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
