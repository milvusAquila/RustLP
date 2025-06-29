use iced::{Theme, widget::pick_list};

pub fn theme_pick_list(theme: &Theme, status: pick_list::Status) -> pick_list::Style {
    let palette = theme.extended_palette();
    pick_list::Style {
        background: iced::Background::Color(palette.background.base.color),
        text_color: palette.background.base.text,
        placeholder_color: theme.palette().success,
        handle_color: match status {
            pick_list::Status::Hovered | pick_list::Status::Opened { .. } => {
                palette.background.weak.color
            }
            pick_list::Status::Active => palette.background.base.color,
        },
        border: iced::Border {
            radius: 2.0.into(),
            width: 1.0,
            color: palette.background.weak.color,
        },
    }
}
