use crate::{App, Message};
use iced::{Theme, alignment, widget};

pub fn text<'a>(
    text: impl widget::text::IntoFragment<'a>,
    app: &App,
) -> widget::Text<'a, Theme, iced::Renderer> {
    widget::text(text)
        .size(app.set.font_size)
        .align_y(alignment::Vertical::Center)
}

pub fn tbutton<'a>(
    txt: impl widget::text::IntoFragment<'a>,
    app: &App,
) -> widget::Button<'a, Message, Theme, iced::Renderer> {
    widget::button(text(txt, app)).style(widget::button::text)
}
