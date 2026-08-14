use std::time::Duration;

use iced::{
    Alignment::Center,
    Element,
    Length::Shrink,
    widget::{column, row, rule, text},
};
use system::memory::Memory;

#[derive(Debug)]
pub enum Message {}

pub fn update(_message: Message) {}

pub fn view<'a>(
    process_count: usize,
    uptime: Duration,
    memory_usage: Memory,
    total_memory: Memory,
) -> Element<'a, Message> {
    let total = uptime.as_secs();
    let days = total / 86_400;
    let hours = total / 3_600 % 24;
    let minutes = total / 60 % 60;
    let seconds = total % 60;
    let memory_usage = memory_usage.as_gib_f64();
    let total_memory = total_memory.as_gib_f64();

    column![
        rule::horizontal(1),
        row![
            text(format!(
                "Uptime: {} days {}:{}:{}",
                days, hours, minutes, seconds
            )),
            rule::vertical(1),
            text(format!("Processes: {}", process_count)),
            rule::vertical(1),
            text(format!("Memory: {memory_usage:.2} / {total_memory:.2}")),
        ]
        .spacing(8)
        .padding(8)
        .align_y(Center)
    ]
    .height(Shrink)
    .align_x(Center)
    .into()
}
