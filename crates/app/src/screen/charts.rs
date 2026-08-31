use iced::{
    Background, Color, Element, Fill, Font,
    Length::Shrink,
    Theme, border,
    theme::palette::{darken, lighten},
    widget::{Text, button, column, container, row, rule, scrollable, space, text},
};
use iced_fonts::lucide;

use crate::{
    state::history::{DiskHistory, History, TimedSample},
    widgets::chart::{ChartSettings, cpu_chart, disk_chart, memory_chart},
};
use system::{memory::Memory, monitor::SystemSnapshot};

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
        column![
            text("CPU").size(20).font(Font {
                weight: iced::font::Weight::Semibold,
                ..Default::default()
            }),
            cpu_chart(history, theme, ChartSettings::detailed()),
            rule::horizontal(1),
            row![
                space::horizontal(),
                column![
                    row![
                        self.content_field("Usage", format!("{:.0}%", snapshot.cpu_usage.total)),
                        self.content_field(
                            "Frequency",
                            format!("{:.2} GHz", snapshot.cpu_usage.frequency as f64 / 1000.0)
                        ),
                    ]
                    .spacing(8)
                ]
                .padding(8)
                .spacing(8),
                rule::vertical(1),
                column![self.content_field("Usage", format!("{:.0}%", snapshot.cpu_usage.total)),]
                    .padding(8)
                    .spacing(8),
                space::horizontal(),
            ]
            .height(Shrink),
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
