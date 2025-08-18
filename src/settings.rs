use iced::{
    Element,
    widget::{column, slider, toggler},
};
use serde::{Deserialize, Serialize};

use crate::{App, Message, widget::ttext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub font_size: f32,
    pub spacing: f32,
    pub debug_layout: bool,
    pub dark_theme: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            font_size: 16.0,
            spacing: 2.0,
            debug_layout: false,
            dark_theme: true,
        }
    }
}

impl App {
    pub fn view_settings(&self) -> Element<'_, Message> {
        let set = &self.set;
        let theme = toggler(set.dark_theme)
            .label("Dark theme")
            .on_toggle(|_| Message::ThemeSelected)
            .size(set.font_size)
            .text_size(set.font_size);

        let font_size_header = ttext("Text size", self);
        let font_size_slidder = slider(10.0..=30.0, set.font_size, Message::TextFontChanged);

        let spacing_header = ttext("Spacing", self);
        let spacing_slider = slider(1.0..=15.0, set.spacing, Message::SpacingChanged);

        let debug_layout = toggler(set.debug_layout)
            .label("Debug layout")
            .on_toggle(|_| Message::DebugToggle)
            .size(set.font_size)
            .text_size(set.font_size);

        let settings = column![
            theme,
            font_size_header,
            font_size_slidder,
            spacing_header,
            spacing_slider,
            debug_layout,
        ];
        Element::from(settings)
    }
}
