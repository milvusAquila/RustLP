use iced::{
    Element, Length,
    widget::{Column, button, column, pick_list, row, scrollable, text},
};
use rusqlite::Result;

use crate::{App, style};
use crate::{Message, db::Sort};

impl App {
    pub fn view_control(&self) -> Element<'_, Message> {
        let index = column![
            button(text("Settings")).on_press(Message::OpenSettings),
            text("Library"),
            pick_list(Sort::ALL, self.sort, Message::SortChanged)
                .style(style::themed_pick_list)
                .width(Length::FillPortion(20)),
            scrollable(self.load_index().expect("ERROR: Failed to load index"))
                .width(Length::FillPortion(20))
                .height(Length::Fill),
        ];
        let preview = text("TODO").width(Length::FillPortion(30));
        let direct = text("TODO").width(Length::FillPortion(30));
        let service = text("TODO").width(Length::FillPortion(20));
        let main = row![index, preview, direct, service].spacing(10).padding(5);

        Element::from(main)
    }
    fn load_index(&self) -> Result<Column<'_, Message>> {
        // Query database
        let mut index = column![];
        let mut query = self.db.prepare(match self.sort.unwrap() {
            Sort::Default => {
                "SELECT b.name, s.number, s.title
                FROM songs s
                LEFT JOIN books b
                ON s.book = b.id
                GROUP BY s.id
                ORDER BY CASE WHEN b.name IS NULL THEN s.title ELSE b.name END,
                         CASE WHEN b.name IS NULL THEN '' ELSE s.number END;
                "
            }
            Sort::Title => "SELECT title FROM songs ORDER BY title;",
            Sort::Songbook => {
                "SELECT b.name, s.number, s.title
                FROM songs s
                JOIN books b
                ON s.book = b.id
                ORDER BY CASE WHEN b.name IS NULL THEN 1 ELSE 0 END, b.name;"
            }
            Sort::Author => {
                "SELECT a.name,s.title FROM authors_songs asng
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
                    button(text(match sort {
                        Sort::Default => format!(
                            "{}{}",
                            if let Ok(book) = i.get::<_, String>(0) {
                                format!("{} {:03}  ", book, i.get::<_, u16>(1).unwrap_or(0))
                            } else {
                                String::new()
                            },
                            i.get::<_, String>(2)? // Title
                        ),
                        Sort::Title => i.get::<_, String>(0)?,
                        Sort::Songbook => {
                            format!(
                                "{} {:03}  {}",
                                i.get::<_, String>(0)?,
                                i.get::<_, u16>(1).unwrap_or(0),
                                i.get::<_, String>(2)?
                            )
                        }
                        Sort::Author => format!("{} ({})", i.get::<_, String>(0)?, i.get::<_, String>(1)?),
                    }))
                    .style(style::header_button),
                )
            };
        }
        Ok(index)
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
