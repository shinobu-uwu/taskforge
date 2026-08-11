use std::time::Duration;

use iced::{
    Alignment::Center,
    Element,
    Length::Shrink,
    widget::{column, row, rule, text},
};

#[derive(Debug)]
pub enum Message {}

pub fn update(_message: Message) {}

pub fn view<'a>(
    process_count: usize,
    uptime: Duration,
    memory_usage: u64,
    total_memory: u64,
) -> Element<'a, Message> {
    let total = uptime.as_secs();
    let days = total / 86_400;
    let hours = total / 3_600 % 24;
    let minutes = total / 60 % 60;
    let seconds = total % 60;

    let memory_usage_gb = memory_usage as f64 / (1024 * 1024 * 1024) as f64;
    let total_memory_gb = total_memory as f64 / (1024 * 1024 * 1024) as f64;

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
            text(format!(
                "Memory: {:.2} / {:.2} GiB",
                memory_usage_gb, total_memory_gb
            )),
        ]
        .spacing(8)
        .padding(8)
        .align_y(Center)
    ]
    .height(Shrink)
    .align_x(Center)
    .into()
}
