use iced::{
    Color, Theme, border,
    widget::{self, button, pick_list},
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

pub fn border_secondary(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: border::width(1).color(soft(theme)),
        ..button::secondary(theme, status)
    }
}

pub fn border_text(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: border::width(1).color(soft(theme)),
        ..button::text(
            theme,
            if status != button::Status::Disabled {
                status
            } else {
                button::Status::Active
            },
        )
    }
}

pub fn soft(theme: &Theme) -> Color {
    theme.extended_palette().secondary.base.color
}

pub fn soft_text(theme: &Theme) -> widget::text::Style {
    widget::text::Style {
        color: Some(soft(theme)),
    }
}

pub fn soft_rule(theme: &Theme) -> widget::rule::Style {
    widget::rule::Style {
        color: soft(theme),
        ..widget::rule::default(theme)
    }
}
