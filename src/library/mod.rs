mod document;
mod song;

pub use document::*;
pub use song::*;

#[derive(Debug)]
pub struct Library {
    pub page: LibraryPage,
    pub songs: LibSong,
    pub docs: LibDoc,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum LibraryPage {
    #[default]
    Songs = 0,
    Documents = 1,
}

impl LibraryPage {
    pub const ALL: [Self; 2] = [Self::Songs, Self::Documents];
}

impl std::fmt::Display for LibraryPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
