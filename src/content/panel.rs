use iced::Task;

use crate::{Message, content::Content};

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Panel {
    pub content: Content,
    status: Status,
}

impl Panel {
    pub fn change_content(&mut self, content: Content) {
        self.content = content;
    }

    pub fn free_content(&mut self) {
        self.content = Content::Empty;
        if self.status == Status::Content {
            self.status = Status::default();
        }
    }

    pub fn set(&mut self, f: impl Fn(&mut Content)) -> Task<Message> {
        f(&mut self.content);
        Task::none()
    }

    pub fn get_status(&self) -> Status {
        return self.status;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_current(&mut self, current: usize) {
        self.content.set_current(current);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Status {
    #[default]
    DarkScreen,
    WhiteScreen,
    Content,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panels {
    Preview = 0,
    Direct = 1,
}
