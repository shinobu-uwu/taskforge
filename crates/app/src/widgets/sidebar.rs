use std::time::Instant;

use iced::{
    Alignment::Center,
    Animation, Element, Fill, Subscription,
    widget::{button, column, container, row, rule, space},
};
use iced_fonts::lucide;

use crate::{
    app::Screen,
    widgets::button::{icon_button, rounded},
};

const EXPANDED_WIDTH: u32 = 200;
const BUTTON_SIZE: u32 = 36;
const PADDING: f32 = 16.0;
const COLLAPSED_WIDTH: u32 = PADDING as u32 * 2 + BUTTON_SIZE;

#[derive(Debug, Clone)]
pub struct Sidebar {
    expanded: Animation<bool>,
    now: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    Navigate(Screen),
    Frame(Instant),
}

impl Sidebar {
    pub fn new(expanded_sidebar: bool) -> Self {
        Self {
            expanded: Animation::new(expanded_sidebar),
            now: Instant::now(),
        }
    }
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Navigate(_) => {} // handled by app
            Message::Frame(i) => self.now = i,
        }
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        if self.expanded.value() != expanded {
            self.now = Instant::now();
            self.expanded.go_mut(expanded, self.now);
        }
    }

    pub fn view(&self, current_screen: Screen) -> Element<'static, Message> {
        let width =
            self.expanded
                .interpolate(COLLAPSED_WIDTH as f32, EXPANDED_WIDTH as f32, self.now);
        let expanded = self.expanded.value();

        let processes = if expanded {
            button(
                row![lucide::sliders_horizontal(), "Processes"]
                    .width(Fill)
                    .spacing(4)
                    .height(Fill)
                    .align_y(Center),
            )
        } else {
            icon_button(lucide::sliders_horizontal())
        }
        .height(BUTTON_SIZE)
        .style(rounded(if current_screen == Screen::Processes {
            button::primary
        } else {
            button::secondary
        }))
        .on_press(Message::Navigate(Screen::Processes));

        let charts = if expanded {
            button(
                row![lucide::chart_area(), "Performance"]
                    .width(Fill)
                    .spacing(4)
                    .height(Fill)
                    .align_y(Center),
            )
        } else {
            icon_button(lucide::chart_area())
        }
        .height(BUTTON_SIZE)
        .style(rounded(if current_screen == Screen::Charts {
            button::primary
        } else {
            button::secondary
        }))
        .on_press(Message::Navigate(Screen::Charts));

        let settings = if expanded {
            button(
                row![lucide::settings(), "Settings"]
                    .width(Fill)
                    .spacing(4)
                    .height(Fill)
                    .align_y(Center),
            )
        } else {
            icon_button(lucide::settings())
        }
        .height(BUTTON_SIZE)
        .style(rounded(if current_screen == Screen::Settings {
            button::primary
        } else {
            button::secondary
        }))
        .on_press(Message::Navigate(Screen::Settings));

        container(
            column![
                processes,
                charts,
                space::vertical(),
                rule::horizontal(1),
                settings,
            ]
            .width(Fill)
            .spacing(12),
        )
        .width(width)
        .height(Fill)
        .padding(PADDING)
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        if self.expanded.is_animating(self.now) {
            iced::window::frames().map(Message::Frame)
        } else {
            Subscription::none()
        }
    }
}
