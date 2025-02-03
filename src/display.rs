use iced::{widget::{image, stack, text}, window, Element};

use crate::Message;

#[derive(Debug)]
pub struct Display {}
impl Display {
    pub fn new() -> Self {
        Display {}
    }
    pub fn view(&self, id: window::Id) -> Element<Message> {
        stack!(
            image("cross.jpg"),
            text("HELLO WORLD"),
        ).into()
    }
}
