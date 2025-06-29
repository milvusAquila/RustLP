use rusqlite::{Connection, Result, Row};
use std::env;

pub fn connect_db() -> Result<Connection> {
    let db = Connection::open(format!(
        "{}/Documents/songs.sqlite",
        env::var("HOME").unwrap_or(String::new())
    ))?;
    db.execute_batch(
        "CREATE TABLE IF NOT EXISTS songs (
                id     INTEGER PRIMARY KEY AUTOINCREMENT,
                title  VARCHAR(255),
                lyrics TEXT NOT NULL,
                book   INTEGER,
                number INTEGER
        );
        CREATE TABLE IF NOT EXISTS authors (
                id     INTEGER PRIMARY KEY,
                name   VARCHAR(255)
        );
        CREATE TABLE IF NOT EXISTS authors_songs (
                author_id INTEGER NOT NULL,
                song_id   INTEGER NOT NULL
        );
        CREATE TABLE IF NOT EXISTS books (
                id   INTEGER PRIMARY KEY,
                name VARCHAR(255)
        );",
    )?;
    Ok(db)
}
pub fn load_songbooks(db: &Connection) -> Result<Vec<Book>> {
    let mut query = db.prepare("SELECT * FROM books;")?;
    let mut iterator = query.query([])?;
    let mut books: Vec<Book> = Vec::new();
    while let Ok(Some(book)) = iterator.next() {
        books.push(book.try_into()?);
    }
    Ok(books)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    #[default]
    Default,
    Title,
    Songbook,
    Author,
}
impl Sort {
    pub const ALL: [Sort; 4] = [Sort::Default, Sort::Title, Sort::Songbook, Sort::Author];
}
impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub struct Song {
    pub id: u16,
    pub title: String,
    pub lyrics: String,
    pub book: Option<u16>,
    pub number: Option<u16>,
}

impl Song {
    pub fn book(&self, list: &Vec<Book>) -> String {
        if self.book.is_none() {
            return String::new();
        }
        let id = self.book.unwrap();
        let book = list
            .iter()
            .find(|book| book.id == id)
            .expect("ERROR: failed to find book");
        book.clone().name
    }
}

impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}\n{}", self.title, self.lyrics)
    }
}

impl TryFrom<&Row<'_>> for Song {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(Song {
            id: value.get(0)?,
            title: value.get(1)?,
            lyrics: value.get(2)?,
            book: value.get::<_, Option<u16>>(3)?,
            number: value.get::<_, Option<u16>>(4)?,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Book {
    pub id: u16,
    pub name: String,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)
    }
}

impl TryFrom<&Row<'_>> for Book {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(Book {
            id: value.get(0)?,
            name: value.get(1)?,
        })
    }
}
