use iced::{
    Alignment, Element, Font, Length, Theme,
    widget::{
        Column, Container, button, column, container, horizontal_rule, mouse_area, pick_list, row,
        scrollable, text_input, vertical_rule,
    },
};
use rusqlite::Result;

use crate::{
    App, Message,
    db::{Sort, load_index},
    style,
    widget::{BOLD, ttext},
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
                .width(Length::FillPortion(18))
                .padding(self.set.spacing),
            text_input("Search", &self.search).on_input(Message::SearchChanged),
            scrollable(self.view_index().expect("ERROR: Failed to load index"))
                .width(Length::FillPortion(18))
                .height(Length::Fill),
        ]
        .spacing(self.set.spacing);
        let preview = self
            .view_song(Content::Preview)
            .height(Length::Fill)
            .width(Length::FillPortion(32));
        let direct = self
            .view_song(Content::Direct)
            .height(Length::Fill)
            .width(Length::FillPortion(32));
        let service = column![
            row![
                // New 0e801
                // File 0e802
                // Save 0e803
                button(icon('\u{0e800}')).on_press(Message::OpenSettings)
            ],
            self.view_service(),
        ]
        .width(Length::FillPortion(18));

        row![
            index,
            vertical_rule(2),
            preview,
            vertical_rule(2),
            direct,
            vertical_rule(2),
            service
        ]
        .spacing(self.set.spacing)
        .padding(5)
        .into()
    }

    fn view_index(&self) -> Result<Column<'_, Message>> {
        let mut index = column![];
        for (id, title) in load_index(&self.db, self.sort.unwrap(), &self.search).unwrap() {
            index = index.push(
                mouse_area(
                    button(ttext(title, self))
                        .style(if id == self.db_select {
                            button::secondary
                        } else {
                            button::text
                        })
                        .width(Length::Fill)
                        .on_press(Message::SelectSong(id))
                        .on_double_click(Message::OpenSong(id, Content::Preview)),
                )
                .on_middle_press(Message::OpenSong(id, Content::Direct)),
            );
        }
        Ok(index)
    }

    fn view_song(&self, content: Content) -> Container<'_, Message, Theme> {
        if let Some(song) = &self.songs[content as usize] {
            let mut lyrics = column![];
            for (index, verse) in song.lyrics.clone().into_iter().enumerate() {
                lyrics = lyrics.push(
                    button(ttext(verse.1, self))
                        .on_press(Message::ChangeVerse(content, index))
                        .width(Length::Fill)
                        .style(if index == song.current {
                            style::secondary
                        } else {
                            style::text
                        }),
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

    fn view_service(&self) -> Container<'_, Message, Theme> {
        let mut titles = column![];
        let current = self.service.current_song_index().unwrap_or(0);
        for song in self.service.clone().into_iter().enumerate() {
            titles = titles.push(
                button(ttext(song.1.title(&self.books), self))
                    .on_double_click(Message::ChangeCurrent(song.0))
                    .width(Length::Fill)
                    .style(if song.0 == current {
                        style::secondary
                    } else {
                        style::text
                    }),
            );
        }
        container(titles)
    }
}
