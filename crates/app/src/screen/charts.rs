use iced::{
    Element, Fill, Font,
    Length::Shrink,
    Theme, border,
    widget::{Text, button, column, row, rule, space, text},
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
                lucide::cpu(),
                cpu_title,
                cpu_chart(cpu_history, theme, self.preview_chart_settings(),),
                Chart::Cpu,
            ),
            self.chart_button(
                lucide::memory_stick(),
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
                lucide::hard_drive(),
                &disk.name,
                disk_chart(&disk.usage, theme, self.preview_chart_settings()),
                Chart::Disk(disk.name.clone()),
            ));
        }

        row![
            charts.spacing(8).width(240).height(Fill),
            self.cpu_content(snapshot, cpu_history, theme)
        ]
        .padding(16)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn chart_button<'a>(
        &'a self,
        icon: Text<'a>,
        label: impl text::IntoFragment<'a>,
        chart_element: Element<'a, Message>,
        chart: Chart,
    ) -> Element<'a, Message> {
        let is_selected = self.selected_chart == chart;

        button(
            row![
                icon.size(24),
                column![text(label).size(20), chart_element].spacing(4)
            ]
            .spacing(8),
        )
        .style(move |theme, status| {
            let mut base = button::subtle(theme, status);
            let palette = theme.palette();

            if is_selected {
                base.border = border::rounded(8).color(palette.primary).width(1)
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
