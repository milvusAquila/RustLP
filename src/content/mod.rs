pub mod db;
pub mod document;
pub mod panel;
pub mod song;

use document::Document;
use song::{Book, Song};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub enum Content {
    #[default]
    Empty,
    Song(Song),
    Document(Document),
}

impl Content {
    pub fn title(&self, books: &Vec<Book>) -> String {
        match self {
            Content::Empty => String::new(),
            Content::Song(song) => song.title(books),
            Content::Document(Document::Pdf(pdf)) => pdf.title(),
        }
    }

    pub fn get_current(&self) -> Option<usize> {
        match self {
            Content::Empty => None,
            Content::Song(song) => Some(song.current),
            Content::Document(Document::Pdf(pdf)) => pdf.current,
        }
    }

    pub fn set_current(&mut self, current: usize) {
        match self {
            Content::Empty => (),
            Content::Song(song) => song.set_current(current),
            Content::Document(Document::Pdf(pdf)) => pdf.current = Some(current),
        }
    }

    pub fn set_previous(&mut self) {
        match self {
            Content::Empty => (),
            Content::Song(song) => song.set_previous(),
            Content::Document(Document::Pdf(pdf)) => pdf.set_previous(),
        }
    }
    pub fn set_next(&mut self) {
        match self {
            Content::Empty => (),
            Content::Song(song) => song.set_next(),
            Content::Document(Document::Pdf(pdf)) => pdf.set_next(),
        }
    }

    pub fn set_next_verse(&mut self) {
        match self {
            Content::Song(song) => song.set_next_verse(),
            _ => (),
        }
    }

    pub fn set_next_chorus(&mut self) {
        match self {
            Content::Song(song) => song.set_next_chorus(),
            _ => (),
        }
    }
}
