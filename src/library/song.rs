use rusqlite::Connection;

use crate::content::song::Book;

#[derive(Debug)]
pub struct LibSong {
    pub db: Connection,
    pub db_select: u16,
    pub sort: Sort,
    pub index: Vec<(u16, String)>,
    pub books: Vec<Book>,
    pub search: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    #[default]
    Default = 0,
    Title = 1,
    Songbook = 2,
    Author = 3,
}

impl Sort {
    pub const ALL: [Sort; 4] = [Sort::Default, Sort::Title, Sort::Songbook, Sort::Author];
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
