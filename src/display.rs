use iced::{
    Element,
    widget::{image, stack, text},
};

use crate::{App, Message};

impl App {
    pub fn view_display(&self) -> Element<'_, Message> {
        stack!(image("cross.jpg"), text("WORLD HELLO"),).into()
    }
}
