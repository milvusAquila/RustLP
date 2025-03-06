use iced::{widget::button, Theme};

pub fn header_button(theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(theme.palette().background)),
        text_color: theme.palette().text,
        ..Default::default()
    }
}
