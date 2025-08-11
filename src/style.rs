use iced::{
    Theme, border,
    widget::{button, pick_list},
};

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

pub fn secondary(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let secondary = button::secondary(theme, status);
    button::Style {
        border: border::width(1).color(palette.secondary.base.color),
        ..secondary
    }
}
pub fn text(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();
    let text = button::text(theme, status);
    button::Style {
        border: border::width(1).color(palette.secondary.base.color),
        ..text
    }
}
