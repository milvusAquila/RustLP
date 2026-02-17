use crate::{Message, content::Content};
use iced::Task;

#[derive(Debug, Default, Clone)]
pub struct Service {
    list: Vec<Content>,
    current: usize,
}

impl Service {
    pub fn new() -> Self {
        Service {
            list: Vec::with_capacity(10),
            current: 0,
        }
    }

    pub fn renew(&mut self) {
        self.list = Vec::with_capacity(10);
        self.current = 0;
    }

    pub fn add(&mut self, content: Content) {
        self.list.push(content);
    }

    pub fn set_current(&mut self, index: usize) {
        if index < self.list.len() {
            self.current = index;
        }
    }

    pub fn get_current(&self) -> Option<usize> {
        if !self.list.is_empty() {
            Some(self.current)
        } else {
            None
        }
    }

    pub fn perform(&mut self, action: Action) -> Task<Message> {
        match action {
            Action::New => self.renew(),
            // OpenLP service files are zip files what contains a service_data.osj file (in json)
            Action::Open => todo!(),
            Action::Save => todo!(),
        }
        Task::none()
    }

    /*
    %USERPROFILE%\AppData\Local\RustLP\
    Compress-Archive file1 file2 archive.zip
    Expand-Archive archive.zip target
     */
}

impl IntoIterator for Service {
    type IntoIter = std::vec::IntoIter<Content>;
    type Item = Content;
    fn into_iter(self) -> Self::IntoIter {
        self.list.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    New,
    Open,
    Save,
}
