use iced::{
    Element, Length, Theme,
    widget::{
        Column, Container, button, column, container, mouse_area, pick_list, row, rule, scrollable,
        space, text_input,
    },
};
use rusqlite::Result;

use crate::{
    App, Message,
    content::{Content, document::Document, panel::Panels},
    library::{LibraryPage, Sort},
    service::Action,
    style,
    widget::{icon, ttext},
};

impl App {
    pub fn view_control(&self) -> Element<'_, Message> {
        let index = column![
            pick_list(
                Some(self.library.page),
                LibraryPage::ALL,
                LibraryPage::to_string,
            )
            .on_select(Message::LibraryPageChanged)
            .text_size(self.set.font_size)
            .style(style::theme_pick_list)
            .width(Length::FillPortion(18))
            .padding(self.set.spacing),
            space::horizontal().height(5.),
            match self.library.page {
                LibraryPage::Songs => self
                    .view_song_index()
                    .expect("ERROR: Failed to load song index"),
                LibraryPage::Documents => self
                    .view_document_index()
                    .expect("ERROR: Failed to load document index"),
            },
        ]
        .spacing(self.set.spacing);
        let preview = self
            .preview_content(Panels::Preview)
            .height(Length::Fill)
            .width(Length::FillPortion(32));
        let direct = self
            .preview_content(Panels::Direct)
            .height(Length::Fill)
            .width(Length::FillPortion(32));
        let service = column![
            self.view_service(),
            space::vertical(),
            row![
                space::horizontal(),
                button(icon('\u{0e800}')).on_press(Message::OpenSettings)
            ],
        ]
        .width(Length::FillPortion(18));

        row![
            index,
            rule::vertical(2),
            preview,
            rule::vertical(2),
            direct,
            rule::vertical(2),
            service
        ]
        .spacing(self.set.spacing)
        .padding(5)
        .into()
    }

    fn view_song_index(&self) -> Result<Column<'_, Message>> {
        let songs = &self.library.songs;
        let header = column![
            pick_list(Some(songs.sort), Sort::ALL, Sort::to_string)
                .on_select(Message::SortChanged)
                .text_size(self.set.font_size)
                .style(style::theme_pick_list)
                .width(Length::FillPortion(18))
                .padding(self.set.spacing),
            text_input("Search", &songs.search)
                .id("search")
                .on_input(Message::SearchChanged)
                .on_submit(Message::ExitSearch),
        ]
        .spacing(self.set.spacing);
        let mut index = Column::with_capacity(songs.index.len());
        for (id, title) in &songs.index {
            index = index.push(
                mouse_area(
                    button(ttext(title, self))
                        .style(if *id == songs.db_select {
                            button::secondary
                        } else {
                            button::text
                        })
                        .width(Length::Fill)
                        .on_press(Message::SelectSong(*id))
                        .on_double_click(Message::OpenSong(*id, Panels::Preview)),
                )
                .on_middle_press(Message::OpenSong(*id, Panels::Direct)),
            );
        }
        Ok(header.push(
            scrollable(index)
                .width(Length::FillPortion(18))
                .height(Length::Fill),
        ))
    }

    fn view_document_index(&self) -> Result<Column<'_, Message>> {
        // TODO
        Ok(column![])
    }

    fn preview_content(&self, panels: Panels) -> Container<'_, Message, Theme> {
        match &self.panels[panels as usize].content {
            Content::Empty => {
                container(ttext("No song selected", self).width(Length::Fill).center())
            }
            Content::Song(song) => song.preview(self, panels),
            Content::Document(Document::Pdf(pdf)) => pdf.preview(self),
        }
    }

    fn view_service(&self) -> Container<'_, Message, Theme> {
        let control = row![
            button(icon('\u{0e801}')).on_press(Message::ServiceAction(Action::New)),
            button(icon('\u{0e802}')).on_press(Message::ServiceAction(Action::Open)),
            button(icon('\u{0e803}')).on_press(Message::ServiceAction(Action::Save)),
        ];
        let mut titles = Column::with_capacity(10);
        let current = self.service.get_current().unwrap_or(0);
        for content in self.service.clone().into_iter().enumerate() {
            titles = titles.push(
                button(ttext(content.1.title(&self.library.songs.books), self))
                    .on_double_click(Message::ServiceChangeContent(content.0))
                    .width(Length::Fill)
                    .style(if content.0 == current {
                        style::border_secondary
                    } else {
                        style::border_text
                    }),
            );
        }
        container(column![control, titles])
    }
}
