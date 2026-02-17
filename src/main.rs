#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use etcetera::BaseStrategy;
use iced::{Element, Size, Task, Theme, widget::container, window};
use pdfium_render::prelude::Pdfium;

use crate::{
    content::{
        Content, db,
        panel::{Panel, Panels, Status},
    },
    library::{LibDoc, LibSong, Library, LibraryPage, Sort},
    service::{Action, Service},
};

mod content;
mod control;
mod display;
mod library;
mod service;
mod settings;
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
    library: Library,
    service: Service,
    panels: [Panel; 2],
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(window::Id),
    DisplayResolution(Size),
    Close(window::Id),
    GoSearch,
    SearchChanged(String),
    ExitSearch,
    LibraryPageChanged(LibraryPage),
    SortChanged(Sort),
    SelectSong(u16),
    OpenSong(u16, Panels),
    ServiceAction(Action),
    AddToService,
    ServiceChangeContent(usize),
    ChangeScreenStatus(Panels, Status),
    ChangeIndex(Panels, usize),
    Previous(Panels),
    Next(Panels),
    NextChorus(Panels),
    NextVerse(Panels),
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
        let index = db::load_index(&db, Sort::default(), &String::new())
            .expect("ERROR: Failed to load index");

        let libdoc = LibDoc {
            index: vec![],
            current: 0,
            pdfium: Pdfium::default(),
            cache_path: {
                let mut path = etcetera::base_strategy::choose_native_strategy()
                    .expect("ERROR: Can't get cache path")
                    .cache_dir();
                path.push(NAME);
                match std::fs::exists(&path) {
                    Ok(true) => (),
                    _ => std::fs::create_dir(&path).expect("ERROR: Can't create cache directory"),
                }
                path
            },
        };
        (
            Self {
                window: WId {
                    control: control_id,
                    display: display_id,
                    settings: None,
                },
                resolution: Size::new(1920.0, 1080.0), // Tempopary value
                set: settings,
                library: Library {
                    page: LibraryPage::default(),
                    songs: LibSong {
                        db: db,
                        db_select: 0,
                        sort: Sort::default(),
                        index: index,
                        books: books,
                        search: String::new(),
                    },
                    docs: libdoc,
                },
                service: Service::new(),
                panels: [Panel::default(), Panel::default()],
            },
            Task::batch([
                control.map(Message::WindowOpened),
                display.map(Message::WindowOpened),
                window::size(display_id).map(Message::DisplayResolution),
            ]),
        )
    }

    fn title(&self, _id: window::Id) -> String {
        String::from(NAME)
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use iced::keyboard::{Event, Key, key::Named, listen};
        iced::Subscription::batch([
            listen().filter_map(|event| match event {
                Event::KeyPressed { key, modifiers, .. } => {
                    if modifiers.is_empty() {
                        match key.as_ref() {
                            Key::Character("c") => Some(Message::NextChorus(Panels::Direct)),
                            Key::Character("v") => Some(Message::NextVerse(Panels::Direct)),
                            Key::Named(Named::Enter) => Some(Message::AddToService),
                            Key::Named(Named::ArrowUp) => Some(Message::Previous(Panels::Direct)),
                            Key::Named(Named::ArrowDown) => Some(Message::Next(Panels::Direct)),
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
                }
                _ => None,
            }),
            window::close_events().map(Message::Close),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let songs = &mut self.library.songs;
        match message {
            Message::WindowOpened(_id) => Task::none(),
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
                    if let Some(settings) = self.window.settings {
                        if id == settings {
                            self.window.settings = None;
                        }
                    }
                    Task::none()
                }
            }
            Message::GoSearch => iced::widget::operation::focus("search"),
            Message::SearchChanged(search) => {
                songs.db_select = 0;
                songs.search = search;
                songs.index = db::load_index(&songs.db, songs.sort, &songs.search)
                    .expect("ERROR: Failed to load index");
                Task::none()
            }
            Message::ExitSearch => {
                if !songs.index.is_empty() {
                    songs.db_select = songs.index[0].0;
                }
                iced::widget::operation::focus("none")
            }
            Message::LibraryPageChanged(lib) => {
                self.library.page = lib;
                Task::none()
            }
            Message::SortChanged(sort) => {
                songs.db_select = 0;
                songs.sort = sort;
                songs.index = db::load_index(&songs.db, songs.sort, &songs.search)
                    .expect("ERROR: Failed to load index");
                Task::none()
            }
            Message::SelectSong(id) => {
                songs.db_select = id;
                Task::none()
            }
            Message::OpenSong(id, panels) => {
                if let Some(song) = db::load_song(&songs.db, id).ok() {
                    self.panels[panels as usize].change_content(Content::Song(song));
                }
                Task::none()
            }
            Message::ServiceAction(action) => self.service.perform(action),
            Message::AddToService => {
                if let Some(song) = db::load_song(&songs.db, songs.db_select).ok() {
                    self.service.add(Content::Song(song));
                }
                Task::none()
            }
            Message::ServiceChangeContent(index) => {
                self.service.set_current(index);
                Task::none()
            }
            Message::ChangeScreenStatus(panels, status) => {
                self.panels[panels as usize].set_status(status);
                Task::none()
            }
            Message::ChangeIndex(panels, current) => {
                self.panels[panels as usize].set_current(current);
                Task::none()
            }
            Message::Previous(panels) => self.panels[panels as usize].set(Content::set_previous),
            Message::Next(panels) => self.panels[panels as usize].set(Content::set_next),
            Message::NextChorus(panels) => {
                self.panels[panels as usize].set(Content::set_next_chorus)
            }
            Message::NextVerse(panels) => self.panels[panels as usize].set(Content::set_next_verse),
            // Settings
            Message::OpenSettings => {
                if self.window.settings.is_some() {
                    return Task::none();
                }
                let (settings_id, settings) = window::open(window::Settings::default());
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
            self.view_display(Panels::Direct)
        };
        if self.set.debug_layout {
            screen = screen.explain(iced::Color::WHITE);
        }
        container(screen).into()
    }

    fn theme(&self, id: window::Id) -> Theme {
        if id == self.window.display || self.set.dark_theme {
            Theme::Dark
        } else {
            Theme::Light
        }
    }
}

#[derive(Debug)]
struct WId {
    control: window::Id,
    display: window::Id,
    settings: Option<window::Id>,
}
