use iced::{widget::{image, stack, text}, Element};

use crate::{App, Message};

impl App {
    pub fn view_display(&self) -> Element<Message> {
        stack!(
        image("cross.jpg"),
            text("WORLD HELLO"),
        ).into()
    }
}
