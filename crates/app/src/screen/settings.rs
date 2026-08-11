use iced::{
    Element, Fill, Theme,
    theme::Base,
    widget::{button, column, scrollable, text},
};

#[derive(Debug, Default)]
pub(crate) struct SettingsScreen;

#[derive(Debug, Clone)]
pub(crate) enum Message {
    ThemeChange(Theme),
}

impl SettingsScreen {
    pub(crate) fn update(&mut self, message: Message) {}

    pub(crate) fn view(&self, current_theme: &Theme) -> Element<'_, Message> {
        column![
            text("Theme:"),
            scrollable(column(Theme::ALL.iter().map(|t| {
                button(text(t.name()))
                    .width(Fill)
                    .style(if current_theme == t {
                        button::primary
                    } else {
                        button::secondary
                    })
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
