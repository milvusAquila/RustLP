use quick_xml::{Reader, events::Event};
use rusqlite::Row;
use std::fmt::Display;

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

    pub fn set_next_chorus(&mut self) {
        self.set_next_type(VerseType::Chorus);
    }

    pub fn set_next_verse(&mut self) {
        self.set_next_type(VerseType::Verse);
    }

    // Go to the next verse of vtype even if it is before current position
    fn set_next_type(&mut self, vtype: VerseType) {
        let lyrics = &self.lyrics.0;
        // End of the song
        for id in self.current + 1..self.lyrics.0.len() {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lyrics(Vec<(Verse, String)>);

impl Lyrics {
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
