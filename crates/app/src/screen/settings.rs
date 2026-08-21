use enum_iterator::all;
use iced::{
    Element, Fill,
    widget::{button, column, scrollable, text},
};

use crate::config::Theme;

#[derive(Debug, Default)]
pub struct SettingsScreen;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeChange(Theme),
}

impl SettingsScreen {
    pub fn view(&self, current_theme: &Theme) -> Element<'_, Message> {
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
