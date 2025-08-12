use rusqlite::{Connection, Result};
use std::{cmp::Ordering, env};

pub mod song;
use song::*;

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
    db.create_collation("NOACCENTS", noaccents)?;
    Ok(db)
}

fn noaccents(title: &str, query: &str) -> Ordering {
    use unidecode::unidecode;
    let title = unidecode(&title.to_lowercase());
    let query = unidecode(&query.to_lowercase());
    if title.contains(&query) {
        Ordering::Equal
    } else {
        Ordering::Greater
    }
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

pub fn load_index(db: &Connection, sort: Sort, search: &String) -> Result<Vec<(u16, String)>> {
    // Query database
    let mut index = vec![];
    let mut query = db.prepare(Sort::QUERYS[sort as usize])?;
    let mut iterator = query.query([search])?;
    //  Create widgets
    while let Ok(Some(i)) = iterator.next() {
        index.push((
            i.get::<_, u16>(0).unwrap(),
            match sort {
                Sort::Default => format!(
                    "{}{}",
                    if let Ok(book) = i.get::<_, String>(1) {
                        format!("{} {:03}  ", book, i.get::<_, u16>(2).unwrap_or(0)) // Songbook Number
                    } else {
                        String::new()
                    },
                    i.get::<_, String>(3)?, // Title
                ),
                Sort::Title => format!(
                    "{} ({})",
                    i.get::<_, String>(1)?, // Title
                    i.get::<_, String>(2)?, // Authors
                ),
                Sort::Songbook => {
                    format!(
                        "{} {:03}  {}",
                        i.get::<_, String>(1)?,          // Songbook
                        i.get::<_, u16>(2).unwrap_or(0), // Number
                        i.get::<_, String>(3)?,          // Title
                    )
                }
                Sort::Author => format!(
                    "{} ({})",
                    i.get::<_, String>(1)?, // Author
                    i.get::<_, String>(2)?, // Title
                ),
            },
        ));
    }
    Ok(index)
}

pub fn load_song(db: &Connection, id: u16) -> Result<Song> {
    let mut query =
        db.prepare("SELECT id, title, lyrics, book, number FROM songs WHERE id = ?;")?;
    query.query_one([id], |row| row.try_into())
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
    pub const QUERYS: [&str; 4] = [
        "SELECT s.id, b.name, s.number, s.title
            FROM songs s
            LEFT JOIN books b
            ON s.book = b.id
            WHERE s.title = ?1 COLLATE NOACCENTS OR s.number = ?1
            GROUP BY s.id
            ORDER BY CASE WHEN b.name IS NULL THEN s.title ELSE b.name END,
                     CASE WHEN b.name IS NULL THEN '' ELSE s.number END;",
        "SELECT s.id, s.title, GROUP_CONCAT(a.name, ', ') AS authors
            FROM songs s
            JOIN authors_songs asng ON s.id = asng.song_id
            JOIN authors a ON asng.author_id = a.id
            WHERE s.title = ?1 COLLATE NOACCENTS
            GROUP BY s.id
            ORDER BY s.title;",
        "SELECT s.id, b.name, s.number, s.title
            FROM songs s
            JOIN books b
            ON s.book = b.id
            WHERE s.number = ?1
            ORDER BY CASE WHEN b.name IS NULL THEN 1 ELSE 0 END, b.name;",
        "SELECT s.id, a.name, s.title
            FROM authors_songs asng
            JOIN authors a ON a.id = asng.author_id
            JOIN songs s ON s.id = asng.song_id
            WHERE a.name = ?1 COLLATE NOACCENTS
            ORDER BY a.name,s.title;",
    ];
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Service(Vec<Song>, usize);

impl Service {
    pub fn new() -> Self {
        Service(Vec::with_capacity(10), 0)
    }

    pub fn push_maybe(&mut self, song: Option<Song>) {
        if let Some(song) = song {
            self.0.push(song);
        }
    }

    pub fn set_current_song(&mut self, index: usize) {
        if index < self.0.len() {
            self.1 = index;
        }
    }

    pub fn current_song_index(&self) -> Option<usize> {
        if !self.0.is_empty() {
            Some(self.1)
        } else {
            None
        }
    }

    pub fn current_song(&self) -> Option<&Song> {
        if !self.0.is_empty() {
            Some(&self.0[self.1])
        } else {
            None
        }
    }
}

impl From<Vec<Song>> for Service {
    fn from(value: Vec<Song>) -> Self {
        Self(value, 0)
    }
}

impl IntoIterator for Service {
    type IntoIter = std::vec::IntoIter<Song>;
    type Item = Song;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let db = connect_db().unwrap();
        let books = load_songbooks(&db).unwrap();
        let mut query = db.prepare(Sort::QUERYS[0]).unwrap();
        let mut iterator = query.query([]).unwrap();
        let mut j = 0;
        while let Ok(Some(i)) = iterator.next() {
            let id = i.get(0).unwrap();
            let song = load_song(&db, id).unwrap();
            if j % 100 == 1 {
                println!("{}", song.title(&books));
                println!("{:#?}\n", song.lyrics);
            }
            j += 1;
        }
    }
}
