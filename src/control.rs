use iced::{
    Length, Widget,
    widget::{
        Column, button, column, container, mouse_area, pick_list, row, rule, scrollable, space,
        text_input,
    },
};

use crate::{
    App, Message,
    content::{Content, document::Document, panel::Panels},
    library::{LibraryPage, Sort},
    service::Action,
    style,
    widget::{icon, ttext},
};

impl App {
    pub fn view_control(&self) -> impl iced::Widget<Message> {
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
                LibraryPage::Songs => column![self.view_song_index()],
                LibraryPage::Documents => column![self.view_document_index()],
            },
        ]
        .width(Length::FillPortion(18))
        .spacing(self.set.spacing);
        let preview = container(self.preview_content(Panels::Preview))
            .height(Length::Fill)
            .width(Length::FillPortion(32));
        let direct = container(self.preview_content(Panels::Direct))
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
    }

    fn view_song_index(&self) -> impl iced::Widget<Message> {
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
                        .on_press(Message::SelectSong(*id)),
                    //TODO: .on_double_click(Message::OpenSong(*id, Panels::Preview)),
                )
                .on_middle_press(Message::OpenSong(*id, Panels::Direct)),
            );
        }
        header.push(scrollable(index).height(Length::Fill).boxed())
    }

    fn view_document_index(&self) -> impl iced::Widget<Message> {
        // TODO
        column!["TODO"]
    }

    fn preview_content(&self, panels: Panels) -> impl iced::Widget<Message> {
        match &self.panels[panels as usize].content {
            Content::Empty => ttext("No song selected", self)
                .width(Length::Fill)
                .center()
                .boxed(),
            Content::Song(song) => song.preview(self, panels).boxed(),
            Content::Document(Document::Pdf(pdf)) => pdf.preview(self).boxed(),
        }
    }

    fn view_service(&self) -> impl iced::Widget<Message> {
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
                    // TODO .on_double_click(Message::ServiceChangeContent(content.0))
                    .width(Length::Fill)
                    .style(if content.0 == current {
                        style::border_secondary
                    } else {
                        style::border_text
                    }),
            );
        }
        column![control, titles]
    }
}
