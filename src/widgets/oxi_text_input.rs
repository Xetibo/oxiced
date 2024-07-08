use iced::{
    border::Radius,
    widget::text_input::{Status, Style},
    Border, Theme,
};

use crate::Message;

pub fn text_input_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced::Background::Color(palette.primary.base.color),
        border: Border {
            color: palette.background.strong.color,
            width: 3.0,
            radius: Radius::from(10),
        },
        icon: palette.background.base.text,
        placeholder: palette.primary.weak.color,
        value: palette.background.base.text,
        selection: palette.primary.strong.color,
    };
    match status {
        Status::Active => style,
        Status::Hovered => {
            style.background = iced::Background::Color(palette.primary.base.color);
            style
        }
        Status::Focused => {
            style.background = iced::Background::Color(palette.primary.base.color);
            style
        }
        Status::Disabled => {
            style.background = iced::Background::Color(palette.primary.base.color);
            style
        }
    }
}

pub fn text_input<'a>(
    placeholder: &str,
    value: &str,
    on_text_changed: impl Fn(String) -> Message + 'a,
) -> iced::widget::TextInput<'a, Message> {
    iced::widget::text_input(placeholder, value)
        .padding(10)
        .on_input(on_text_changed)
        .style(text_input_style)
}
