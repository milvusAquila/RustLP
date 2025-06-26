#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Element, Task, Theme, widget::container, window};
use rusqlite::Connection;

mod control;
mod db;
mod display;
mod settings;
mod style;

fn main() -> iced::Result {
    iced::daemon(App::new, App::update, App::view)
        .title(App::title)
        .subscription(App::subscription)
        .theme(App::theme)
        .run()
}

#[derive(Debug)]
struct App {
    control: window::Id,
    display: window::Id,
    set: settings::Settings,
    db: Connection,
    sort: Option<db::Sort>,
}

#[derive(Debug, Clone)]
enum Message {
    WindowOpened(window::Id),
    Close(window::Id),
    SortChanged(db::Sort),
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
        let (control_id, control) = window::open(window::Settings::default());
        let (display_id, display) = window::open(window::Settings {
            fullscreen: true,
            level: window::Level::AlwaysOnTop,
            exit_on_close_request: false,
            ..Default::default()
        });
        (
            Self {
                control: control_id,
                display: display_id,
                set: settings::Settings::default(),
                db: db::connect_db().expect("ERROR: Failed to connect database"),
                sort: Some(db::Sort::default()),
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
                _ => None,
            }),
            window::close_events().map(Message::Close),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => window::maximize(id, true),
            Message::Close(id) => {
                if id == self.control {
                    Task::batch([window::close(self.display), iced::exit()])
                } else {
                    Task::none()
                }
            }
            Message::SortChanged(sort) => {
                self.sort = Some(sort);
                Task::none()
            }
            // Settings
            Message::OpenSettings => {
                let (settings_id, settings) = window::open(window::Settings {
                    maximized: false,
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
            self.view_display()
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
