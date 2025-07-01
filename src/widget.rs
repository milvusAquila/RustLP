use crate::{App, Message};
use iced::{
    alignment::Vertical, widget::{button, text, Button}, Font, Theme
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
