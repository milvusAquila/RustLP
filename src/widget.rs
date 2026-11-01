use crate::App;
use iced::{Font, Theme, alignment::Vertical, widget::text};

pub fn ttext<'a>(text: impl text::IntoFragment<'a>, app: &App) -> iced::widget::Text<'a, Theme> {
    iced::widget::text(text)
        .size(app.set.font_size)
        .align_y(Vertical::Center)
}

pub const BOLD: Font = Font {
    weight: iced::font::Weight::Bold,
    ..Font::DEFAULT
};

pub fn icon<'a>(codepoint: char) -> iced::widget::Text<'a> {
    const ICON_FONT: Font = Font::new("icons");
    iced::widget::text(codepoint).font(ICON_FONT)
}
