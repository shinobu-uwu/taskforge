use iced::{
    Background, Border, Theme,
    widget::text_input::{Status, Style},
};

pub fn primary(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let active = Style {
        background: Background::Color(palette.background.base.color),
        border: Border {
            radius: 8.0.into(),
            width: 1.0,
            color: palette.background.strong.color,
        },
        icon: palette.background.weak.text,
        placeholder: palette.secondary.base.color,
        value: palette.background.base.text,
        selection: palette.primary.weak.color,
    };

    match status {
        Status::Active => active,
        Status::Hovered => Style {
            border: Border {
                color: palette.background.base.text,
                ..active.border
            },
            ..active
        },
        Status::Focused { .. } => Style {
            border: Border {
                color: palette.primary.strong.color,
                ..active.border
            },
            ..active
        },
        Status::Disabled => Style {
            background: Background::Color(palette.background.weak.color),
            value: active.placeholder,
            placeholder: palette.background.strongest.color,
            ..active
        },
    }
}
