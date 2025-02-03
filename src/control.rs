use iced::{
    widget::{button, column, text, Space},
    window, Element, Length,
};
use iced_aw::menu::{self, Item};

use crate::style;
use crate::Message;

#[derive(Debug)]
pub struct Control {}

impl Control {
    pub fn new() -> Self {
        Control {}
    }
    pub fn view(&self, id: window::Id, debug_layout: bool) -> Element<Message> {
        // Header
        let menu_tpl = |items| {
            menu::Menu::new(items)
                .max_width(180.0)
                .offset(5.0)
                .spacing(5.0)
        };

        let open = button(text("Open")).style(style::header_button);

        let editor = button(text("Edit"))
            // .on_press(Message::OpenEditor)
            .style(style::header_button);

        #[rustfmt::skip]
        let header = iced_aw::menu_bar!(
            (button(text("File"))
                .style(style::header_button), // see in src/style.rs
            {
                menu_tpl(iced_aw::menu_items!(
                    (open)
                    (editor)
                )).width(Length::Shrink)
            })
            (button(text("Settings"))
                .style(style::header_button),
            {
                self.view_settings(debug_layout) // see in src/settings.rs
            })
        );

        // Main

        // Final
        let grid = column![
            // Header
            header,
            // Main
            Space::with_height(Length::Fill),
        ];
        Element::from(grid)
    }
}
