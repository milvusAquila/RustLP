#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Element, Size, Task, Theme, widget::container, window};
use rusqlite::Connection;

use crate::{
    control::Content,
    db::{SAction, Service, Status, load_index, load_song},
    song::{Book, Song},
};

mod control;
mod db;
mod display;
mod settings;
mod song;
mod style;
mod widget;

const NAME: &str = "RustLP";

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
    window: WId,
    resolution: Size,
    set: settings::Settings,
    db: Connection,
    db_select: u16,
    sort: Option<db::Sort>,
    index: Vec<(u16, String)>,
    service: Service,
    books: Vec<Book>,
    search: String,
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(window::Id),
    DisplayResolution(Size),
    Close(window::Id),
    SortChanged(db::Sort),
    SelectSong(u16),
    OpenSong(u16, Content),
    ServiceAction(SAction),
    AddToService,
    ChangeCurrent(usize),
    ChangeScreen(Status, Content),
    ChangeVerse(Content, usize),
    Previous(Content),
    Next(Content),
    NextChorus(Content),
    NextVerse(Content),
    GoSearch,
    SearchChanged(String),
    ExitSearch,
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
        let settings = confy::load(NAME, None).expect("ERROR: Failed to load settings");
        let db = db::connect_db().expect("ERROR: Failed to connect database");
        let books = db::load_songbooks(&db).expect("ERROR: Failed to load books");
        let index = load_index(&db, db::Sort::default(), &String::new())
            .expect("ERROR: Failed to load index");
        (
            Self {
                window: WId {
                    control: control_id,
                    display: display_id,
                    settings: None,
                },
                resolution: Size::new(1920.0, 1080.0), // Tempopary value
                set: settings,
                db: db,
                db_select: 0,
                sort: Some(db::Sort::default()),
                index: index,
                service: Service::new(),
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
        String::from(NAME)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use iced::keyboard::{Key, key::Named, on_key_press};
        iced::Subscription::batch([
            on_key_press(|key, modifiers| {
                if modifiers.is_empty() {
                    match key.as_ref() {
                        Key::Character("c") => Some(Message::NextChorus(Content::Direct)),
                        Key::Character("v") => Some(Message::NextVerse(Content::Direct)),
                        Key::Named(Named::Enter) => Some(Message::AddToService),
                        Key::Named(Named::ArrowUp) => Some(Message::Previous(Content::Direct)),
                        Key::Named(Named::ArrowDown) => Some(Message::Next(Content::Direct)),
                        _ => None,
                    }
                } else if modifiers.command() {
                    match key.as_ref() {
                        Key::Character(",") => Some(Message::OpenSettings),
                        Key::Character("f") => Some(Message::GoSearch),
                        _ => None,
                    }
                } else {
                    None
                }
            }),
            window::close_events().map(Message::Close),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => {
                if id == self.window.display {
                    window::get_size(id).map(Message::DisplayResolution)
                } else {
                    Task::none()
                }
            }
            Message::DisplayResolution(size) => {
                self.resolution = size;
                Task::none()
            }
            Message::Close(id) => {
                if id == self.window.control {
                    confy::store(NAME, None, self.set.clone())
                        .is_err()
                        .then(|| println!("ERROR: Failed to save settings"));
                    Task::batch([
                        if let Some(settings) = self.window.settings {
                            window::close(settings)
                        } else {
                            Task::none()
                        },
                        window::close(self.window.display),
                        iced::exit(),
                    ])
                } else {
                    Task::none()
                }
            }
            Message::SortChanged(sort) => {
                self.sort = Some(sort);
                self.index = load_index(&self.db, self.sort.unwrap(), &self.search).unwrap();
                Task::none()
            }
            Message::SelectSong(id) => {
                self.db_select = id;
                Task::none()
            }
            Message::OpenSong(id, content) => {
                self.service.add(load_song(&self.db, id).ok(), content);
                Task::none()
            }
            Message::ServiceAction(saction) => self.service.perform(saction),
            Message::AddToService => {
                self.service
                    .push_maybe(load_song(&self.db, self.db_select).ok());
                Task::none()
            }
            Message::ChangeCurrent(index) => {
                self.service.set_current_song(index);
                Task::none()
            }
            Message::ChangeScreen(status, content) => {
                self.service.status[content as usize] = status;
                Task::none()
            }
            Message::ChangeVerse(content, verse) => {
                if let Some(song) = &mut self.service.current_song_mut(content) {
                    song.set_current(verse);
                }
                Task::none()
            }
            Message::Previous(content) => self.service.change(content, Song::set_previous),
            Message::Next(content) => self.service.change(content, Song::set_next),
            Message::NextChorus(content) => self.service.change(content, Song::set_next_chorus),
            Message::NextVerse(content) => self.service.change(content, Song::set_next_verse),
            Message::GoSearch => iced::widget::text_input::focus("search"),
            Message::SearchChanged(search) => {
                self.db_select = 0;
                self.search = search;
                self.index = load_index(&self.db, self.sort.unwrap(), &self.search).unwrap();
                Task::none()
            }
            Message::ExitSearch => {
                if !self.index.is_empty() {
                    self.db_select = self.index[0].0;
                }
                iced::widget::text_input::focus("none")
            }
            // Settings
            Message::OpenSettings => {
                if self.window.settings.is_some() {
                    return Task::none();
                }
                let (settings_id, settings) = window::open(window::Settings {
                    maximized: false,
                    size: Size {
                        width: 300.0,
                        height: 400.0,
                    },
                    ..Default::default()
                });
                self.window.settings = Some(settings_id);
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
        let mut screen = if id == self.window.control {
            self.view_control()
        } else if Some(id) == self.window.settings {
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
        if id == self.window.display {
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

// Control, Display, Settings
#[derive(Debug)]
struct WId {
    control: window::Id,
    display: window::Id,
    settings: Option<window::Id>,
}
