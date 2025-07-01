use iced::{
    Color, Element, Font, Length, Size, Theme, Vector,
    advanced::{Layout, Text, Widget, layout, renderer, widget::Tree},
    alignment::Vertical,
    mouse,
    widget::{image, text},
};

use crate::{widget::BOLD, App, Message};

impl App {
    pub fn view_display(&self) -> Element<'_, Message> {
        if let Some(song) = &self.direct {
            let title = song.title(&self.books);
            Display::new(&song.lyrics, &title, "cross.jpg").into()
        } else {
            image("cross.jpg").into()
        }
    }
}

struct Display {
    title: String,
    lyrics: String,
    font_size: f32,
    image: String,
}

impl Display {
    fn new(lyrics: &str, title: &str, image: &str) -> Self {
        Self {
            title: title.to_string(),
            lyrics: lyrics.to_string(),
            font_size: 40.0,
            image: image.to_string(),
        }
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Display
where
    Renderer: iced::advanced::Renderer
        + iced::advanced::text::Renderer<Font = Font>
        + iced::advanced::image::Renderer<Handle = image::Handle>,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(limits.max().width, limits.max().height))
    }

    fn draw(
        &self,
        _state: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        // Background
        image::draw(
            renderer,
            layout,
            viewport,
            &image::Handle::from_path(&self.image),
            iced::ContentFit::Fill,
            image::FilterMethod::Linear,
            iced::Rotation::default(),
            1.0,
            1.0,
        );
        let bounds = layout.bounds();
        // Title
        renderer.fill_text(
            Text {
                content: self.title.clone(),
                bounds: bounds.size(),
                size: (self.font_size / 2.0).into(),
                line_height: text::LineHeight::default(),
                font: BOLD,
                align_x: text::Alignment::Left,
                align_y: Vertical::Bottom,
                shaping: text::Shaping::default(),
                wrapping: text::Wrapping::default(),
            },
            iced::Point {
                x: 0.0,
                y: bounds.y + bounds.height,
            },
            Color::WHITE,
            *viewport,
        );
        let lyrics = Text {
            content: self.lyrics.clone(),
            bounds: bounds.size(),
            size: self.font_size.into(),
            line_height: text::LineHeight::Relative(1.5),
            font: BOLD,
            align_x: text::Alignment::Center,
            align_y: Vertical::Center,
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
        };
        // Stroke
        renderer.with_translation(Vector::new(2.0, 2.0), |renderer| {
            renderer.fill_text(
                lyrics.clone(),
                bounds.center(),
                Color::BLACK,
                *viewport,
            )
        });
        // Lyrics
        renderer.fill_text(
            lyrics,
            bounds.center(),
            Color::WHITE,
            *viewport,
        );
    }
}

impl<'a, Message, Renderer> From<Display> for iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer
        + iced::advanced::text::Renderer<Font = Font>
        + iced::advanced::image::Renderer<Handle = image::Handle>,
{
    fn from(widget: Display) -> Self {
        Self::new(widget)
    }
}
