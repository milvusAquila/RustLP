use quick_xml::{Reader, events::Event};
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

pub fn load_index(db: &Connection, sort: Sort) -> Result<Vec<(u16, String)>> {
    // Query database
    let mut index = vec![];
    let mut query = db.prepare(Sort::QUERYS[sort as usize])?;
    let mut iterator = query.query([])?;
    //  Create widgets
    while let Ok(Some(i)) = iterator.next() {
        index.push((
            i.get::<_, u16>(0).unwrap(),
            match sort {
                Sort::Default => format!(
                    "{}{}",
                    if let Ok(book) = i.get::<_, String>(1) {
                        format!("{} {:03}  ", book, i.get::<_, u16>(2).unwrap_or(0))
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
            GROUP BY s.id
            ORDER BY CASE WHEN b.name IS NULL THEN s.title ELSE b.name END,
                     CASE WHEN b.name IS NULL THEN '' ELSE s.number END;",
        "SELECT s.id, s.title, GROUP_CONCAT(a.name, ', ') AS authors
            FROM songs s
            JOIN authors_songs asng ON s.id = asng.song_id
            JOIN authors a ON asng.author_id = a.id
            GROUP BY s.id
            ORDER BY s.title;",
        "SELECT s.id, b.name, s.number, s.title
            FROM songs s
            JOIN books b
            ON s.book = b.id
            ORDER BY CASE WHEN b.name IS NULL THEN 1 ELSE 0 END, b.name;",
        "SELECT s.id, a.name, s.title
            FROM authors_songs asng
            JOIN authors a ON a.id = asng.author_id
            JOIN songs s ON s.id = asng.song_id
            ORDER BY a.name,s.title;",
    ];
}

impl std::fmt::Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
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

#[derive(Debug, Clone)]
pub struct Song {
    pub id: u16,
    pub title: String,
    pub lyrics: Lyrics,
    pub book: Option<u16>,
    pub number: Option<u16>,
    pub current: usize,
}

impl Song {
    pub fn book(&self, books: &Vec<Book>) -> String {
        if self.book.is_none() {
            return String::new();
        }
        let id = self.book.unwrap();
        let book = books
            .iter()
            .find(|book| book.id == id)
            .expect("ERROR: failed to find book");
        book.clone().name
    }

    pub fn title(&self, books: &Vec<Book>) -> String {
        let mut title = String::new();
        if self.book.is_some() {
            title += &self.book(books);
            if self.number.is_some() {
                title += &format!(" {:03}  ", self.number.unwrap());
            } else {
                title += "  ";
            }
        }
        title += &self.title;
        title
    }

    pub fn set_current(&mut self, verse: usize) {
        if verse < self.lyrics.0.len() {
            self.current = verse;
        }
    }

    pub fn set_previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn set_next(&mut self) {
        if self.current + 1 < self.lyrics.0.len() {
            self.current += 1;
        }
    }
}

impl TryFrom<&Row<'_>> for Song {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        let lyrics: Lyrics = value.get::<_, String>(2)?.try_into().unwrap();
        Ok(Song {
            id: value.get(0)?,
            title: value.get(1)?,
            lyrics: lyrics,
            book: value.get::<_, Option<u16>>(3)?,
            number: value.get::<_, Option<u16>>(4)?,
            current: 0,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerseType {
    Intro,
    Verse,
    PreChorus,
    Chorus,
    Bridge,
    End,
    Other,
}

impl TryFrom<&str> for VerseType {
    type Error = ();
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        match value {
            "i" => Ok(VerseType::Intro),
            "v" => Ok(VerseType::Verse),
            "p" => Ok(VerseType::PreChorus),
            "c" => Ok(VerseType::Chorus),
            "b" => Ok(VerseType::Bridge),
            "e" => Ok(VerseType::End),
            "o" => Ok(VerseType::Other),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verse(VerseType, u8);

impl Verse {
    pub fn new(versetype: VerseType, nb: u8) -> Self {
        Self(versetype, nb)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lyrics(Vec<(Verse, String)>);

impl Lyrics {
    /* pub fn get_verse(&self, verse: Verse) -> String {
        let result = self.0.iter().find(|x| x.0 == verse);
        if let Some(lyrics) = result {
            lyrics.1.clone()
        } else {
            String::new()
        }
    } */

    pub fn get(&self, index: usize) -> String {
        if index < self.0.len() {
            self.0[index].1.to_string()
        } else {
            String::new()
        }
    }
}

impl IntoIterator for Lyrics {
    type Item = (Verse, String);
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<String> for Lyrics {
    type Error = quick_xml::Error;
    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        let mut lyrics = Lyrics(vec![]);
        let (mut verse, mut nb, mut txt) = (VerseType::Other, 0, String::new());
        let mut reader = Reader::from_reader(value.as_bytes());
        let mut buf = vec![];
        reader.config_mut().trim_text(true);
        loop {
            match reader.read_event_into(&mut buf) {
                Err(e) => eprintln!(
                    "ERROR: Unable to read XML: position: {}, error: {}",
                    reader.error_position(),
                    e
                ),
                Ok(Event::Eof) => break,
                Ok(Event::Start(element)) if element.name().as_ref() == b"verse" => {
                    for attr_result in element.attributes() {
                        let decoder = reader.decoder();
                        let a = attr_result.unwrap();
                        match decoder
                            .decode(a.key.local_name().as_ref())?
                            .to_string()
                            .as_str()
                        {
                            "type" => {
                                verse = a
                                    .decode_and_unescape_value(decoder)?
                                    .to_string()
                                    .as_str()
                                    .try_into()
                                    .unwrap();
                            }
                            "label" => {
                                nb = a
                                    .decode_and_unescape_value(decoder)?
                                    .parse::<u8>()
                                    .expect("ERROR: Non valid number")
                            }
                            other => println!("Unknown attribute: {}", other),
                        }
                    }
                }
                Ok(Event::CData(raw)) => {
                    txt = raw.decode()?.to_string();
                }
                Ok(Event::End(element)) if element.name().as_ref() == b"verse" => {
                    lyrics.0.push((Verse::new(verse, nb), txt));
                    (verse, nb, txt) = (VerseType::Other, 0, String::new());
                }
                _ => (),
            }
            buf.clear();
        }
        Ok(lyrics)
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

    pub fn get_current_song(&mut self) -> Option<&mut Song> {
        if !self.0.is_empty() {
            Some(&mut self.0[self.1])
        } else {
            None
        }
    }

    pub fn set_current_verse(&mut self, verse: usize) {
        if let Some(song) = self.get_current_song() {
            song.set_current(verse);
        }
    }

    pub fn set_previous_verse(&mut self) {
        if let Some(song) = self.get_current_song() {
            song.set_previous();
        }
    }

    pub fn set_next_verse(&mut self) {
        if let Some(song) = self.get_current_song() {
            song.set_next();
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
