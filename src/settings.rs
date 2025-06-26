use iced::{
    Element,
    widget::{column, slider, text, toggler},
    window,
};

use crate::{App, Message};

#[derive(Debug, Clone)]
pub struct Settings {
    pub window: Option<window::Id>,
    pub font_size: f32,
    pub spacing: f32,
    pub debug_layout: bool,
    pub dark_theme: bool,
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            window: None,
            font_size: 16.0,
            spacing: 10.0,
            debug_layout: false,
            dark_theme: true,
        }
    }
}

impl App {
    pub fn view_settings(&self) -> Element<'_, Message> {
        let set = &self.set;
        let theme: iced::widget::Toggler<Message> = toggler(set.dark_theme)
            .label("Theme")
            .on_toggle(|_| Message::ThemeSelected)
            .size(set.font_size)
            .text_size(set.font_size);

        let font_size_header = text("Text size").size(set.font_size);
        let font_size_slidder = slider(10.0..=50.0, set.font_size, Message::TextFontChanged);

        let spacing_header = text("Spacing").size(set.font_size);
        let spacing_slider = slider(0.0..=20.0, set.spacing, Message::SpacingChanged);

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
            debug_layout
        ];
        Element::from(settings)
    }
}
