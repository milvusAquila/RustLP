use crate::App;
use iced::{Font, Theme, alignment::Vertical, widget::text};

pub fn ttext<'a>(
    text: impl text::IntoFragment<'a>,
    app: &App,
) -> iced::widget::Text<'a, Theme, iced::Renderer> {
    iced::widget::text(text)
        .size(app.set.font_size)
        .align_y(Vertical::Center)
}

pub const BOLD: Font = Font {
    weight: iced::font::Weight::Bold,
    ..Font::DEFAULT
};
