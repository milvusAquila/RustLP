use iced::{
    Alignment, Element, Font, Length, Theme,
    alignment::Vertical,
    widget::{
        Column, Container, button, column, container, horizontal_rule, horizontal_space,
        mouse_area, pick_list, row, scrollable, text_input, vertical_rule, vertical_space,
    },
};
use rusqlite::Result;

use crate::{
    App, Message,
    db::{SAction, Sort, Status},
    style,
    widget::{BOLD, ttext},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            pick_list(Sort::ALL, Some(self.sort), Message::SortChanged)
                .text_size(self.set.font_size)
                .style(style::theme_pick_list)
                .width(Length::FillPortion(18))
                .padding(self.set.spacing),
            text_input("Search", &self.search)
                .id("search")
                .on_input(Message::SearchChanged)
                .on_submit(Message::ExitSearch),
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
            self.view_service(),
            vertical_space(),
            row![
                horizontal_space(),
                button(icon('\u{0e800}')).on_press(Message::OpenSettings)
            ],
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
        let mut index = Column::with_capacity(2000);
        for (id, title) in &self.index {
            index = index.push(
                mouse_area(
                    button(ttext(title, self))
                        .style(if *id == self.db_select {
                            button::secondary
                        } else {
                            button::text
                        })
                        .width(Length::Fill)
                        .on_press(Message::SelectSong(*id))
                        .on_double_click(Message::OpenSong(*id, Content::Preview)),
                )
                .on_middle_press(Message::OpenSong(*id, Content::Direct)),
            );
        }
        Ok(index)
    }

    fn view_song(&self, content: Content) -> Container<'_, Message, Theme> {
        let Some(song) = &self.service.current_song(content) else {
            return container(ttext("No song selected", self).width(Length::Fill).center());
        };
        let mut lyrics = column![horizontal_rule(1).style(style::soft_rule)];
        for (index, verse) in song.lyrics.clone().into_iter().enumerate() {
            lyrics = lyrics
                .push(
                    row![
                        vertical_rule(1).style(style::soft_rule),
                        // Verse id
                        ttext(format!("{}", verse.0), self).style(style::soft_text),
                        vertical_rule(1).style(style::soft_rule),
                        // Lyrics
                        button(ttext(verse.1, self))
                            .on_press(Message::ChangeVerse(content, index))
                            .width(Length::Fill)
                            .style(if index == song.current {
                                button::secondary
                            } else {
                                button::text
                            }),
                        vertical_rule(1).style(style::soft_rule),
                    ]
                    .height(Length::Shrink)
                    .align_y(Vertical::Center),
                )
                .push(horizontal_rule(1).style(style::soft_rule));
        }
        let options = row![
            button(icon('\u{0e804}')).on_press(Message::ChangeScreen(Status::DarkScreen, content)),
            button(icon('\u{0e805}')).on_press(Message::ChangeScreen(Status::WhiteScreen, content)),
            button(icon('\u{0e806}')).on_press(Message::ChangeScreen(Status::Song, content)),
        ];
        container(
            column![
                ttext(song.title(&self.books), self)
                    .font(BOLD)
                    .align_x(Alignment::Center)
                    .width(Length::Fill),
                scrollable(lyrics).width(Length::Fill).height(Length::Fill),
                horizontal_rule(2),
                options,
                horizontal_rule(2),
                self.view_display(content),
            ]
            .spacing(self.set.spacing),
        )
    }

    fn view_service(&self) -> Container<'_, Message, Theme> {
        let control = row![
            button(icon('\u{0e801}')).on_press(Message::ServiceAction(SAction::New)),
            button(icon('\u{0e802}')).on_press(Message::ServiceAction(SAction::Open)),
            button(icon('\u{0e803}')).on_press(Message::ServiceAction(SAction::Save)),
        ];
        let mut titles = Column::with_capacity(10);
        let current = self.service.current_song_index().unwrap_or(0);
        for song in self.service.clone().into_iter().enumerate() {
            titles = titles.push(
                button(ttext(song.1.title(&self.books), self))
                    .on_double_click(Message::ChangeCurrentSong(song.0))
                    .width(Length::Fill)
                    .style(if song.0 == current {
                        style::border_secondary
                    } else {
                        style::border_text
                    }),
            );
        }
        container(column![control, titles])
    }
}
