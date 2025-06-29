use crate::{App, Message};
use iced::{Theme, alignment, widget};
// use std::time::{Duration, Instant};

pub fn text<'a>(
    text: impl widget::text::IntoFragment<'a>,
    app: &App,
) -> widget::Text<'a, Theme, iced::Renderer> {
    widget::text(text)
        .size(app.set.font_size)
        .height(app.set.font_size + 2.0 * app.set.spacing)
        .align_y(alignment::Vertical::Center)
}

pub fn button<'a>(
    txt: impl widget::text::IntoFragment<'a>,
    app: &App,
) -> widget::Button<'a, Message, Theme, iced::Renderer> {
    widget::button(text(txt, app)).padding(app.set.spacing)
}

pub fn sbutton<'a>(
    txt: impl widget::text::IntoFragment<'a>,
    app: &App,
) -> widget::Button<'a, Message, Theme, iced::Renderer> {
    widget::button(text(txt, app))
        .padding(app.set.spacing)
        .style(widget::button::text)
}

/* pub enum ClickType {
    Single,
    Double,
}
pub struct ClickTimer {
    idx: usize, // id of the item being clicked
    time: Instant,
}
impl ClickTimer {
    pub fn click(self: &mut Self, idx: usize) -> ClickType {
        let time = Instant::now();
        if idx != self.idx || time - self.time > Duration::from_millis(300) {
            self.idx = idx;
            self.time = time;
            return ClickType::Single;
        }
        self.idx = idx;
        self.time = time;
        return ClickType::Double;
    }
} */
