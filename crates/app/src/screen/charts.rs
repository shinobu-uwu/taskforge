use iced::{
    Element, Fill, Theme,
    widget::{button, column, container, grid},
};

use crate::{
    state::history::{DiskHistory, History},
    widgets::{
        button::rounded,
        chart::{cpu_chart, disk_chart, memory_chart},
    },
};
use system::memory::Memory;

#[derive(Debug, Default)]
pub struct ChartsScreen {
    selected_chart: Option<Chart>,
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
    pub(crate) fn update(&mut self, message: Message) {
        match message {
            Message::SelectChart(c) => self.selected_chart = Some(c),
        }
    }

    pub(crate) fn view<'a>(
        &self,
        cpu_history: &'a History<f32>,
        memory_history: &'a History<Memory>,
        disks_history: &'a [DiskHistory],
        total_memory: Memory,
        theme: &Theme,
    ) -> Element<'a, Message> {
        let mut charts = grid![
            button(column!["CPU", cpu_chart(cpu_history, theme)].spacing(4))
                .style(rounded(button::subtle))
                .on_press(Message::SelectChart(Chart::Cpu)),
            button(column!["Memory", memory_chart(memory_history, total_memory, theme)].spacing(4))
                .style(rounded(button::subtle))
                .on_press(Message::SelectChart(Chart::Memory)),
        ];

        for disk in disks_history {
            charts = charts.push(
                button(
                    column![
                        iced::widget::text(&disk.name),
                        disk_chart(&disk.usage, theme)
                    ]
                    .spacing(4),
                )
                .style(rounded(button::subtle))
                .on_press(Message::SelectChart(Chart::Disk(disk.name.clone()))),
            );
        }

        container(charts.columns(2).spacing(12).height(Fill))
            .padding(16)
            .width(Fill)
            .height(Fill)
            .into()
    }
}
