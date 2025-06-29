use iced::{
    Element, Length,
    widget::{Column, column, pick_list, rich_text, row, scrollable, span, text::Rich},
};
use rusqlite::Result;

use crate::{
    App, Message,
    db::{Song, Sort},
    style,
    widget::{button, sbutton, text},
};

#[derive(Debug, Clone, Copy)]
pub enum Content {
    Preview,
    Direct,
}

impl App {
    pub fn view_control(&self) -> Element<'_, Message> {
        let index = column![
            button("Settings", self).on_press(Message::OpenSettings),
            text("Library", self),
            pick_list(Sort::ALL, self.sort, Message::SortChanged)
                .style(style::theme_pick_list)
                .width(Length::FillPortion(20))
                .padding(self.set.spacing),
            scrollable(self.load_index().expect("ERROR: Failed to load index"))
                .width(Length::FillPortion(20))
                .height(Length::Fill)
                .spacing(self.set.spacing),
        ];
        let preview = self
            .load_song(Content::Preview)
            .expect("ERROR: Failed to load preview")
            .height(Length::Fill)
            .width(Length::FillPortion(30));
        let direct = self
            .load_song(Content::Direct)
            .expect("ERROR: Failed to load direct")
            .height(Length::Fill)
            .width(Length::FillPortion(30));
        let service = text("TODO", self).width(Length::FillPortion(20));
        let main = row![index, preview, direct, service]
            .spacing(self.set.spacing)
            .padding(5);

        Element::from(main)
    }

    fn load_index(&self) -> Result<Column<'_, Message>> {
        // Query database
        let mut index = column![];
        let mut query = self.db.prepare(match self.sort.unwrap() {
            Sort::Default => {
                "SELECT s.id, b.name, s.number, s.title
                FROM songs s
                LEFT JOIN books b
                ON s.book = b.id
                GROUP BY s.id
                ORDER BY CASE WHEN b.name IS NULL THEN s.title ELSE b.name END,
                         CASE WHEN b.name IS NULL THEN '' ELSE s.number END;
                "
            }
            Sort::Title => "SELECT id, title FROM songs ORDER BY title;",
            Sort::Songbook => {
                "SELECT s.id, b.name, s.number, s.title
                FROM songs s
                JOIN books b
                ON s.book = b.id
                ORDER BY CASE WHEN b.name IS NULL THEN 1 ELSE 0 END, b.name;"
            }
            Sort::Author => {
                "SELECT s.id, a.name, s.title
                FROM authors_songs asng
                JOIN authors a ON a.id = asng.author_id
                JOIN songs s ON s.id = asng.song_id
                ORDER BY a.name,s.title;"
            }
        })?;
        let mut iterator = query.query([])?;
        //  Create widgets
        while let Ok(Some(i)) = iterator.next() {
            if let Some(sort) = self.sort {
                index = index.push(
                    sbutton(
                        match sort {
                            Sort::Default => format!(
                                "{}{}",
                                if let Ok(book) = i.get::<_, String>(1) {
                                    format!("{} {:03}  ", book, i.get::<_, u16>(2).unwrap_or(0))
                                } else {
                                    String::new()
                                },
                                i.get::<_, String>(3)? // Title
                            ),
                            Sort::Title => i.get::<_, String>(1)?,
                            Sort::Songbook => {
                                format!(
                                    "{} {:03}  {}",
                                    i.get::<_, String>(1)?,
                                    i.get::<_, u16>(2).unwrap_or(0),
                                    i.get::<_, String>(3)?
                                )
                            }
                            Sort::Author => {
                                format!("{} ({})", i.get::<_, String>(1)?, i.get::<_, String>(2)?)
                            }
                        },
                        self,
                    )
                    .width(Length::Fill)
                    .on_press(Message::OpenSong(
                        i.get::<_, u16>(0).unwrap().clone(),
                        Content::Preview,
                    )),
                )
            };
        }
        Ok(index)
    }

    fn load_song(&self, content: Content) -> Result<Rich<'_, Message, Message>> {
        let content = match content {
            Content::Preview => self.preview,
            Content::Direct => self.direct,
        };
        if content.is_none() {
            return Ok(rich_text![span("No song selected")]);
        }
        let mut query = self
            .db
            .prepare("SELECT id, title, lyrics, book, number FROM songs WHERE id = ?;")?;
        let song: Song = query.query_one([content], |row| row.try_into())?;
        Ok(rich_text![span(if song.book.is_some() {
            let book = song.book(&self.books);
            format!("{} {}", book, song)
        } else {
            format!("{}", song)
        })])
    }
}
/*
FULL:
                "SELECT b.name, s.number, s.title, GROUP_CONCAT(a.name, ', ') AS authors
                FROM songs s
                JOIN authors_songs asng ON s.id = asng.song_id
                JOIN authors a ON asng.author_id = a.id
                LEFT JOIN books b
                ON s.book = b.id
                GROUP BY s.id
                ORDER BY s.title;
                "
*/
