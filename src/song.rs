use quick_xml::{Reader, events::Event};
use rusqlite::Row;
use std::fmt::Display;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Song {
    pub id: u16,
    pub title: String,
    pub lyrics: Vec<(Verse, String)>,
    pub book: Option<u16>,
    pub number: Option<u16>,
    pub current: usize,
}

impl Song {
    fn parse_lyrics(string: String) -> Result<Vec<(Verse, String)>, quick_xml::Error> {
        let mut lyrics = vec![];
        let (mut verse, mut nb, mut txt) = (VerseType::Other, 0, String::new());
        let mut reader = Reader::from_reader(string.as_bytes());
        reader.config_mut().trim_text(true);
        loop {
            match reader.read_event() {
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
                    lyrics.push((Verse::new(verse, nb), txt));
                    (verse, nb, txt) = (VerseType::Other, 0, String::new());
                }
                _ => (),
            }
        }
        Ok(lyrics)
    }

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
        title += &self.book(books);
        if self.number.is_some() {
            if !title.is_empty() {
                title += " ";
            }
            title += &format!("{:03}", self.number.unwrap());
        }
        if !title.is_empty() {
            title += "  ";
        }
        title += &self.title;
        title
    }

    pub fn set_current(&mut self, verse: usize) {
        if verse < self.lyrics.len() {
            self.current = verse;
        }
    }

    pub fn set_previous(&mut self) {
        if self.current > 0 {
            self.current -= 1;
        }
    }

    pub fn set_next(&mut self) {
        if self.current + 1 < self.lyrics.len() {
            self.current += 1;
        }
    }

    pub fn set_next_chorus(&mut self) {
        self.set_next_type(VerseType::Chorus);
    }

    pub fn set_next_verse(&mut self) {
        self.set_next_type(VerseType::Verse);
    }

    // Go to the next verse of vtype even if it is before current position
    fn set_next_type(&mut self, vtype: VerseType) {
        let lyrics = &self.lyrics;
        // End of the song
        for id in self.current + 1..self.lyrics.len() {
            if lyrics[id].0.0 == vtype {
                self.current = id;
                return;
            }
        }
        // Continue at the beginning if not found in the end
        for id in 0..self.current {
            if lyrics[id].0.0 == vtype {
                self.current = id;
                return;
            }
        }
    }

    pub fn get(&self, index: usize) -> String {
        if index < self.lyrics.len() {
            self.lyrics[index].1.to_string()
        } else {
            String::new()
        }
    }
}

impl TryFrom<&Row<'_>> for Song {
    type Error = rusqlite::Error;
    fn try_from(value: &Row<'_>) -> std::result::Result<Self, Self::Error> {
        Ok(Song {
            id: value.get(0)?,
            title: value.get(1)?,
            lyrics: Song::parse_lyrics(value.get::<_, String>(2)?)
                .expect("ERROR: Failed to parse lyrics"),
            book: value.get(3)?,
            number: value.get(4)?,
            current: 0,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verse(VerseType, u8);

impl Verse {
    pub fn new(versetype: VerseType, nb: u8) -> Self {
        Self(versetype, nb)
    }
}

impl Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.0, self.1)
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

impl Display for VerseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VerseType::Intro => "I",
                VerseType::Verse => "V",
                VerseType::PreChorus => "P",
                VerseType::Chorus => "C",
                VerseType::Bridge => "B",
                VerseType::End => "E",
                VerseType::Other => "O",
            }
        )
    }
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
