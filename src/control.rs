use iced::{
    Alignment, Element, Font, Length, Theme,
    widget::{
        Column, Container, button, column, container, horizontal_rule, pick_list, row, scrollable,
    },
};
use rusqlite::Result;

use crate::{
    App, Message,
    db::{Sort, load_index},
    style,
    widget::{BOLD, tbutton, ttext},
};

#[derive(Debug, Clone, Copy)]
pub enum Content {
    Preview = 0,
    Direct = 1,
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
        let mut index = column![];
        for (id, title) in load_index(&self.db, self.sort.unwrap()).unwrap() {
            index = index.push(
                tbutton(title, self)
                    .width(Length::Fill)
                    .on_press(Message::OpenSong(id, Content::Preview)),
            );
        }
        Ok(index)
    }

    fn view_song(&self, content: Content) -> Container<'_, Message, Theme> {
        let song = &self.song[content as usize];
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
