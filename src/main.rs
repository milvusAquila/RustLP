#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{widget::container, window, Element, Task, Theme};

mod control;
mod display;
mod settings;
mod style;

const FONT_SIZE: f32 = 16.0;

fn main() -> iced::Result {
    iced::daemon(App::title, App::update, App::view)
        .subscription(App::subscription)
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Debug)]
struct App {
    debug_layout: bool,
    control: (window::Id, control::Control),
    display: (window::Id, display::Display),
}

#[derive(Debug, Clone)]
enum Message {
    DebugToggle,
    WindowOpened(window::Id),
    Close(window::Id),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        // Create the two windows
        let (control_id, control) = window::open(window::Settings {
            ..Default::default()
        });
        let (display_id, display) = window::open(window::Settings {
            decorations: false,
            level: window::Level::AlwaysOnTop,
            exit_on_close_request: false,
            ..Default::default()
        });
        (
            Self {
                debug_layout: false,
                control: (control_id, control::Control::new()),
                display: (display_id, display::Display::new()),
            },
            Task::batch([
                control.map(Message::WindowOpened),
                display.map(Message::WindowOpened),
            ]),
        )
    }

    fn title(&self, _id: window::Id) -> String {
        String::from("Rust-LP")
    }

    fn subscription(&self) -> iced::Subscription<Message> {
        use iced::keyboard::*;
        iced::Subscription::batch([
            on_key_press(|key, _modifiers| {
                match key.as_ref() {
                    // Key::Character("o") if modifiers.command() => Some(Message::OpenFile), // Ctrl + o
                    // Key::Named(keyboard::key::Named::Enter) => Some(Message::Enter),       // Enter
                    _ => None,
                }
            }),
            window::close_events().map(Message::Close),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DebugToggle => {
                self.debug_layout = !self.debug_layout;
                Task::none()
            }
            Message::WindowOpened(id) => window::maximize(id, true),
            Message::Close(id) => {
                if id == self.control.0 {
                    iced::exit()
                } else {
                    Task::none()
                }
            }
        }
    }

    fn view(&self, id: window::Id) -> Element<'_, Message> {
        let mut screen = if id == self.control.0 {
            self.control.1.view(id, self.debug_layout)
        } else {
            self.display.1.view(id)
        };
        if self.debug_layout {
            screen = screen.explain(iced::Color::WHITE);
        }
        container(screen).into()
    }

    fn theme(&self, _id: window::Id) -> Theme {
        Theme::Dark
    }
}
