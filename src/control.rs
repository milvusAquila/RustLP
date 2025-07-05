use iced::{
    Alignment, Element, Font, Length, Theme,
    widget::{
        Column, Container, button, column, container, horizontal_rule, pick_list, row, scrollable,
    },
};
use rusqlite::Result;

use crate::{
    App, Message,
    db::Sort,
    style,
    widget::{BOLD, tbutton, ttext},
};

#[derive(Debug, Clone, Copy)]
pub enum Content {
    Preview,
    Direct,
}

fn icon<'a>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("icons");
    iced::widget::text(codepoint).font(ICON_FONT).into()
}

impl App {
    pub fn view_control(&self) -> Element<'_, Message> {
        let index = column![
            ttext("Library", self),
            pick_list(Sort::ALL, self.sort, Message::SortChanged)
                .text_size(self.set.font_size)
                .style(style::theme_pick_list)
                .width(Length::FillPortion(20))
                .padding(self.set.spacing),
            scrollable(self.view_index().expect("ERROR: Failed to load index"))
                .width(Length::FillPortion(20))
                .height(Length::Fill),
        ]
        .spacing(self.set.spacing);
        let preview = self
            .view_song(Content::Preview)
            .height(Length::Fill)
            .width(Length::FillPortion(35));
        let direct = self
            .view_song(Content::Direct)
            .height(Length::Fill)
            .width(Length::FillPortion(35));
        let service = column![
            row![
                // New 0e801
                // File 0e802
                // Save 0e803
                button(icon('\u{0e800}')).on_press(Message::OpenSettings)
            ],
            ttext("TODO", self)
        ]
        .width(Length::FillPortion(10));

        row![index, preview, direct, service]
            .spacing(self.set.spacing)
            .padding(5)
            .into()
    }

    fn view_index(&self) -> Result<Column<'_, Message>> {
        // Query database
        let mut index = column![];
        let mut query = self.db.prepare(Sort::QUERYS[self.sort.unwrap() as usize])?;
        let mut iterator = query.query([])?;
        //  Create widgets
        while let Ok(Some(i)) = iterator.next() {
            if let Some(sort) = self.sort {
                index = index.push(
                    tbutton(
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
                                    i.get::<_, String>(1)?, // Songbook
                                    i.get::<_, u16>(2).unwrap_or(0), // Number
                                    i.get::<_, String>(3)?, // Title
                                )
                            }
                            Sort::Author => format!(
                                "{} ({})",
                                i.get::<_, String>(1)?, // Author
                                i.get::<_, String>(2)?, // Title
                            ),
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

    fn view_song(&self, content: Content) -> Container<'_, Message, Theme> {
        let song = match content {
            Content::Preview => &self.preview,
            Content::Direct => &self.direct,
        };
        if let Some(song) = song {
            let mut lyrics = column![];
            for verse in song.lyrics.clone().into_iter() {
                lyrics = lyrics.push(
                    tbutton(verse.1, self)
                        .on_press(Message::ChangeVerse(content, verse.0))
                        .width(Length::Fill),
                );
            }
            container(
                column![
                    ttext(song.title(&self.books), self)
                        .font(BOLD)
                        .align_x(Alignment::Center)
                        .width(Length::Fill),
                    scrollable(lyrics).width(Length::Fill).height(Length::Fill),
                    horizontal_rule(2),
                    self.view_display(content),
                ]
                .spacing(self.set.spacing),
            )
        } else {
            container(ttext("No song selected", self).center())
        }
    }
}
