use iced::{
    advanced::widget::text, alignment, widget::button, Length, Pixels, Renderer, Theme
};

pub fn header_button(theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(theme.palette().background)),
            text_color: theme.palette().text,
            ..Default::default()
    }
}

pub fn style_text(
    text: text::Text<Theme, Renderer>,
    font_size: Pixels,
) -> text::Text<Theme, Renderer> {
    text.size(font_size)
        .height(Length::Fixed(font_size.0 * 1.3 + 10.0))
        // 1.3 is the default value for LineHeight // 2 * 5.0 is the padding
        .align_y(alignment::Vertical::Center)
}
