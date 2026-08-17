use iced::{Theme, border, theme::palette, widget::container};

pub fn background(theme: &Theme) -> container::Style {
    let color = palette::darken(theme.palette().background, 0.05);

    container::Style::default()
        .background(color)
        .color(palette::readable(color, theme.palette().text))
}

pub fn content(theme: &Theme) -> container::Style {
    container::Style::default()
        .background(theme.palette().background)
        .color(theme.palette().text)
        .border(border::rounded(border::top_left(16)))
}
