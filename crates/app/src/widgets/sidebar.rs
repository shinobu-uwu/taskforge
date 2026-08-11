use iced::{
    Element, Fill,
    widget::{button, column, container, row, rule, space},
};

use crate::app::Screen;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    Navigate(Screen),
    OpenWebsite,
}

pub(crate) fn view(current_screen: Screen) -> Element<'static, Message> {
    let processes = button("Processes")
        .width(Fill)
        .style(if current_screen == Screen::Processes {
            button::primary
        } else {
            button::secondary
        })
        .on_press(Message::Navigate(Screen::Processes));

    // let charts = button("Charts")
    //     .width(Fill)
    //     .style(if current_screen == Screen::Charts {
    //         button::primary
    //     } else {
    //         button::secondary
    //     })
    //     .on_press(Message::Navigate(Screen::Charts));
    let settings = button("Settings")
        .width(Fill)
        .style(if current_screen == Screen::Settings {
            button::primary
        } else {
            button::secondary
        })
        .on_press(Message::Navigate(Screen::Settings));

    row![
        container(
            column![
                button("Taskforge")
                    .style(button::text)
                    .on_press(Message::OpenWebsite),
                processes,
                // charts,
                space::vertical(),
                settings,
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
