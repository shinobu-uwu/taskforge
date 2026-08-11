use enum_iterator::all;
use iced::{
    Element, Fill,
    widget::{button, column, scrollable, text},
};

use crate::config::Theme;

#[derive(Debug, Default)]
pub(crate) struct SettingsScreen;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    ThemeChange(Theme),
}

impl SettingsScreen {
    pub(crate) fn view(&self, current_theme: &Theme) -> Element<'_, Message> {
        column![
            text("Theme:"),
            scrollable(column(all::<Theme>().map(|t| {
                button(text(t.name()))
                    .width(Fill)
                    .style(if *current_theme == t {
                        button::primary
                    } else {
                        button::text
                    })
                    .padding(8)
                    .on_press(Message::ThemeChange(t.clone()))
                    .into()
            })))
        ]
        .padding(8)
        .width(Fill)
        .height(Fill)
        .into()
    }
}
