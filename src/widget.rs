use crate::{App, Message};
use iced::{
    Background, Font, Theme,
    alignment::Vertical,
    border,
    widget::{Button, button, text},
};

pub fn ttext<'a>(
    text: impl text::IntoFragment<'a>,
    app: &App,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    iced::widget::text(text)
        .size(app.set.font_size)
        .align_y(Vertical::Center)
}

pub fn tbutton<'a>(
    txt: impl text::IntoFragment<'a>,
    app: &App,
) -> Button<'a, Message, Theme, iced::Renderer> {
    button(ttext(txt, app)).style(button::text)
}

pub const BOLD: Font = Font {
    weight: iced::font::Weight::Bold,
    ..Font::DEFAULT
};

pub fn primary(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let text = button::text(theme, status);
    button::Style {
        background: Some(Background::Color(palette.background.weak.color)),
        border: border::width(2).color(palette.background.strong.color),
        ..text
    }
}
pub fn secondary(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let text = button::text(theme, status);
    button::Style {
        border: border::width(2).color(palette.background.weakest.color),
        ..text
    }
}
