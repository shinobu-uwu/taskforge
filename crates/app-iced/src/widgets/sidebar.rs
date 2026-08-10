use iced::{
    Element, Fill,
    widget::{button, column, container, row, rule},
};

use crate::app::Screen;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Screen),
    OpenWebsite,
}

pub(crate) fn view(current_screen: Screen) -> Element<'static, Message> {
    let processes_is_active = current_screen == Screen::Processes;
    let charts_is_active = current_screen == Screen::Charts;

    let processes = button("Processes")
        .width(Fill)
        .style(if processes_is_active {
            button::primary
        } else {
            button::secondary
        })
        .on_press(Message::Navigate(Screen::Processes));

    let charts = button("Charts")
        .width(Fill)
        .style(if charts_is_active {
            button::primary
        } else {
            button::secondary
        })
        .on_press(Message::Navigate(Screen::Charts));

    row![
        container(
            column![
                button("Taskforge")
                    .style(button::text)
                    .on_press(Message::OpenWebsite),
                processes,
                charts
            ]
            .width(Fill)
            .spacing(12),
        )
        .width(199)
        .height(Fill)
        .padding(16),
        rule::vertical(1),
    ]
    .width(200)
    .height(Fill)
    .into()
}
