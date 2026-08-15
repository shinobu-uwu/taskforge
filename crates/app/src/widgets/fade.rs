use iced::{
    Background, Element, Theme,
    widget::{
        button as button_widget, container as container_widget, rule as rule_widget,
        text as text_widget,
    },
};

pub fn container(mut style: container_widget::Style, opacity: f32) -> container_widget::Style {
    style.background = style
        .background
        .map(|background| background_with_opacity(background, opacity));
    style.text_color = style.text_color.map(|color| color.scale_alpha(opacity));
    style.border.color = style.border.color.scale_alpha(opacity);
    style.shadow.color = style.shadow.color.scale_alpha(opacity);
    style
}

pub fn button(
    style: impl Fn(&Theme, button_widget::Status) -> button_widget::Style,
    opacity: f32,
) -> impl Fn(&Theme, button_widget::Status) -> button_widget::Style {
    move |theme, status| {
        let mut style = style(theme, status);
        style.background = style
            .background
            .map(|background| background_with_opacity(background, opacity));
        style.text_color = style.text_color.scale_alpha(opacity);
        style.border.color = style.border.color.scale_alpha(opacity);
        style.shadow.color = style.shadow.color.scale_alpha(opacity);
        style
    }
}

pub fn text(
    style: impl Fn(&Theme) -> text_widget::Style,
    opacity: f32,
) -> impl Fn(&Theme) -> text_widget::Style {
    move |theme| {
        let mut style = style(theme);
        style.color = style.color.map(|color| color.scale_alpha(opacity));
        style
    }
}

pub fn horizontal_rule<'a, Message: 'a>(width: u32, opacity: f32) -> Element<'a, Message> {
    rule_widget::horizontal(width)
        .style(move |theme: &Theme| {
            let mut style = rule_widget::default(theme);
            style.color = style.color.scale_alpha(opacity);
            style
        })
        .into()
}

fn background_with_opacity(background: Background, opacity: f32) -> Background {
    match background {
        Background::Color(color) => Background::Color(color.scale_alpha(opacity)),
        Background::Gradient(gradient) => Background::Gradient(gradient.scale_alpha(opacity)),
    }
}
