use enum_iterator::all;
use iced::{
    Element, Fill,
    widget::{button, column, scrollable, text, toggler},
};

use crate::config::{Config, Theme};

#[derive(Debug, Default)]
pub struct SettingsScreen;

#[derive(Debug, Clone)]
pub enum Message {
    ConfigChanged(Config),
}

impl SettingsScreen {
    pub fn view<'a>(&self, config: &'a Config) -> Element<'a, Message> {
        let sidebar_config = config.clone();

        column![
            toggler(config.expanded_sidebar)
                .label("Expanded sidebar")
                .on_toggle(move |expanded_sidebar| {
                    let mut config = sidebar_config.clone();
                    config.expanded_sidebar = expanded_sidebar;
                    Message::ConfigChanged(config)
                }),
            text("Theme:"),
            scrollable(column(all::<Theme>().map(|t| {
                let is_current = config.theme == t;
                let mut config = config.clone();
                config.theme = t.clone();

                button(text(t.name()))
                    .width(Fill)
                    .style(if is_current {
                        button::primary
                    } else {
                        button::text
                    })
                    .padding(8)
                    .on_press(Message::ConfigChanged(config))
                    .into()
            })))
        ]
        .spacing(8)
        .padding(8)
        .width(Fill)
        .height(Fill)
        .into()
    }
}
