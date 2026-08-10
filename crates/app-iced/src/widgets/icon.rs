use iced::{
    Font,
    widget::{
        text::Shaping,
        text_input::{Icon, Side},
    },
};

pub(crate) trait IntoTextInputIcon {
    fn into_text_input_icon(self) -> Icon<Font>;
}

impl IntoTextInputIcon for (String, Font, Shaping) {
    fn into_text_input_icon(self) -> Icon<Font> {
        let (content, font, _shaping) = self;

        Icon {
            font,
            code_point: content
                .parse()
                .expect("icon fonts must return exactly one character"),
            size: Some(16.0.into()),
            spacing: 8.0,
            side: Side::Left,
        }
    }
}
