use iced::{
    widget::{button, pick_list},
    Theme,
};

pub fn header_button(theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: Some(iced::Background::Color(theme.palette().background)),
        text_color: theme.palette().text,
        ..Default::default()
    }
}
pub fn themed_pick_list(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();
    pick_list::Style {
        background: iced::Background::Color(palette.background.base.color),
        text_color: palette.background.base.text,
        placeholder_color: theme.palette().success,
        handle_color: match status {
            pick_list::Status::Hovered | pick_list::Status::Opened {..}=> palette.background.weak.color,
            pick_list::Status::Active => palette.background.base.color,
        },
        border: iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.weak.color,
        },
    }
}
