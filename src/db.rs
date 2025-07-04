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
    pub current: Option<Verse>,
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

    pub fn set_current(song: &mut Option<Self>, verse: Verse) {
        if let Some(song) = song {
            song.current = Some(verse);
        }
    }
}

impl TryFrom<&Row<'_>> for Song {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        let lyrics: Lyrics = value.get::<_, String>(2)?.try_into().unwrap();
        let current = Some(lyrics.0[0].0);
        Ok(Song {
            id: value.get(0)?,
            title: value.get(1)?,
            lyrics: lyrics,
            book: value.get::<_, Option<u16>>(3)?,
            number: value.get::<_, Option<u16>>(4)?,
            current: current,
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
    pub fn get_verse(&self, verse: Verse) -> String {
        let result = self.0.iter().find(|x| x.0 == verse);
        if let Some(lyrics) = result {
            lyrics.1.clone()
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
                                match a.decode_and_unescape_value(decoder)?.to_string().as_str() {
                                    "i" => verse = VerseType::Intro,
                                    "v" => verse = VerseType::Verse,
                                    "p" => verse = VerseType::PreChorus,
                                    "c" => verse = VerseType::Chorus,
                                    "b" => verse = VerseType::Bridge,
                                    "e" => verse = VerseType::End,
                                    "o" => verse = VerseType::Other,
                                    _ => panic!(),
                                }
                            }
                            "label" => {
                                nb = a
                                    .decode_and_unescape_value(decoder)?
                                    .parse::<u8>()
                                    .expect("ERROR: Non valid number")
                            }
                            _ => panic!(),
                        }
                    }
                }
                Ok(Event::CData(raw)) => {
                    txt = raw.decode()?.to_string();
                }
                _ => (),
            }
            if nb != 0 && txt != String::new() {
                lyrics.0.push((Verse::new(verse, nb), txt));
                nb = 0;
                txt = String::new();
            }
            buf.clear();
        }
        Ok(lyrics)
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
