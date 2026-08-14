use iced::{
    Length::Fill,
    theme::Theme,
    widget::{
        button::{Status, Style},
        container,
    },
};

pub fn rounded_full(style: impl Fn(&Theme, Status) -> Style) -> impl Fn(&Theme, Status) -> Style {
    move |theme, status| {
        let mut style = style(theme, status);
        style.border.radius = 9999.0.into();
        style
    }
}

pub fn rounded(style: impl Fn(&Theme, Status) -> Style) -> impl Fn(&Theme, Status) -> Style {
    move |theme, status| {
        let mut style = style(theme, status);
        style.border.radius = 8.0.into();
        style
    }
}

pub fn icon_button<'a, Message>(
    icon: impl Into<iced::Element<'a, Message>>,
) -> iced::widget::Button<'a, Message>
where
    Message: 'a,
{
    iced::widget::button(container(icon).center(Fill))
        .width(36)
        .height(36)
        .style(rounded_full(iced::widget::button::text))
}
