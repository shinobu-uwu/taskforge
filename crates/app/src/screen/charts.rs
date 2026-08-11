use iced::{
    Element, Fill,
    widget::{column, text},
};
use system::monitor::SystemSnapshot;

#[derive(Debug, Default)]
pub(crate) struct ChartsScreen;

#[derive(Debug, Clone)]
pub(crate) enum Message {}

impl ChartsScreen {
    pub(crate) fn update(&mut self, message: Message) {
        match message {}
    }

    pub(crate) fn view(&self, _snapshot: &SystemSnapshot) -> Element<'_, Message> {
        column![text("Charts!")].width(Fill).height(Fill).into()
    }
}
