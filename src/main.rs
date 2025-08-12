#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Element, Size, Task, Theme, widget::container, window};
use rusqlite::Connection;

use crate::{
    control::Content,
    db::{Book, Service, Song, load_song},
};

mod control;
mod db;
mod display;
mod settings;
mod style;
mod widget;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .theme(App::theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .run()
}

#[derive(Debug)]
struct App {
    control: window::Id,
    display: window::Id,
    resolution: Size,
    set: settings::Settings,
    db: Connection,
    db_select: u16,
    sort: Option<db::Sort>,
    service: Service,
    songs: [Option<Song>; 2],
    books: Vec<Book>,
    search: String,
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(window::Id),
    Close(window::Id),
    SortChanged(db::Sort),
    SelectSong(u16),
    OpenSong(u16, Content),
    AddToService,
    ChangeCurrent(usize),
    ChangeVerse(Content, usize),
    Previous(Content),
    Next(Content),
    NextChorus(Content),
    NextVerse(Content),
    SearchChanged(String),
    // Settings
    OpenSettings,
    SpacingChanged(f32),
    TextFontChanged(f32),
    ThemeSelected,
    DebugToggle,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        // Create the two windows
        let (control_id, control) = window::open(window::Settings {
            maximized: true,
            ..Default::default()
        });
        let (display_id, display) = window::open(window::Settings {
            fullscreen: true,
            level: window::Level::AlwaysOnTop,
            exit_on_close_request: false,
            ..Default::default()
        });
        let db = db::connect_db().expect("ERROR: Failed to connect database");
        let books = db::load_songbooks(&db).expect("ERROR: Failed to load books");
        (
            Self {
                control: control_id,
                display: display_id,
                resolution: Size::new(1920.0, 1080.0),
                set: settings::Settings::default(),
                db: db,
                db_select: 0,
                sort: Some(db::Sort::default()),
                service: Service::new(),
                songs: [None, None],
                books: books,
                search: String::new(),
            },
            Task::batch([
                control.map(Message::WindowOpened),
                display.map(Message::WindowOpened),
            ]),
        )
    }

    fn title(&self, _id: window::Id) -> String {
        String::from("RustLP")
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use iced::keyboard::*;
        iced::Subscription::batch([
            on_key_press(|key, modifiers| match key.as_ref() {
                Key::Character(",") if modifiers.command() => Some(Message::OpenSettings),
                Key::Character("c") if modifiers.is_empty() => {
                    Some(Message::NextChorus(Content::Direct))
                }
                Key::Character("v") if modifiers.is_empty() => {
                    Some(Message::NextVerse(Content::Direct))
                }
                Key::Named(key::Named::Enter) if modifiers.is_empty() => {
                    Some(Message::AddToService)
                }
                Key::Named(key::Named::ArrowUp) => Some(Message::Previous(Content::Direct)),
                Key::Named(key::Named::ArrowDown) => Some(Message::Next(Content::Direct)),
                _ => None,
            }),
            window::close_events().map(Message::Close),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(_id) => Task::none(),
            Message::Close(id) => {
                if id == self.control {
                    Task::batch([
                        if let Some(settings) = self.set.window {
                            window::close(settings)
                        } else {
                            Task::none()
                        },
                        window::close(self.display),
                        iced::exit(),
                    ])
                } else {
                    Task::none()
                }
            }
            Message::SortChanged(sort) => {
                self.sort = Some(sort);
                Task::none()
            }
            Message::SelectSong(id) => {
                self.db_select = id;
                Task::none()
            }
            Message::OpenSong(id, content) => {
                self.songs[content as usize] = load_song(&self.db, id).ok();
                Task::none()
            }
            Message::AddToService => {
                self.service
                    .push_maybe(load_song(&self.db, self.db_select).ok());
                Task::none()
            }
            Message::ChangeCurrent(index) => {
                self.service.set_current_song(index);
                self.songs[1] = self.service.current_song().cloned();
                Task::none()
            }
            Message::ChangeVerse(content, verse) => {
                if let Some(song) = &mut self.songs[content as usize] {
                    song.set_current(verse);
                }
                Task::none()
            }
            Message::Previous(content) => {
                if let Some(song) = &mut self.songs[content as usize] {
                    song.set_previous();
                }
                Task::none()
            }
            Message::Next(content) => {
                if let Some(song) = &mut self.songs[content as usize] {
                    song.set_next();
                }
                Task::none()
            }
            Message::NextChorus(content) => {
                if let Some(song) = &mut self.songs[content as usize] {
                    song.set_next_chorus();
                }
                Task::none()
            }
            Message::NextVerse(content) => {
                if let Some(song) = &mut self.songs[content as usize] {
                    song.set_next_verse();
                }
                Task::none()
            }
            Message::SearchChanged(search) => {
                self.db_select = 0;
                self.search = search;
                Task::none()
            }
            // Settings
            Message::OpenSettings => {
                let (settings_id, settings) = window::open(window::Settings {
                    maximized: false,
                    size: Size {
                        width: 300.0,
                        height: 400.0,
                    },
                    ..Default::default()
                });
                self.set.window = Some(settings_id);
                settings.map(Message::WindowOpened)
            }
            Message::SpacingChanged(size) => {
                self.set.spacing = size;
                Task::none()
            }
            Message::TextFontChanged(size) => {
                self.set.font_size = size;
                Task::none()
            }
            Message::ThemeSelected => {
                self.set.dark_theme = !self.set.dark_theme;
                Task::none()
            }
            Message::DebugToggle => {
                self.set.debug_layout = !self.set.debug_layout;
                Task::none()
            }
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        let mut screen = if id == self.control {
            self.view_control()
        } else if Some(id) == self.set.window {
            self.view_settings()
        } else {
            self.view_display(Content::Direct)
        };
        if self.set.debug_layout {
            screen = screen.explain(iced::Color::WHITE);
        }
        container(screen).into()
    }

    fn theme(&self, id: window::Id) -> Theme {
        if id == self.display {
            Theme::Dark
        } else {
            if self.set.dark_theme {
                Theme::Dark
            } else {
                Theme::Light
            }
        }
    }
}
