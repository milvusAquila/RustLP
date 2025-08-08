use iced::{
    Color, Element, Font, Length, Size, Theme, Vector,
    advanced::{
        Layout, Text, Widget,
        image::{Bytes, Handle},
        layout, renderer,
        widget::Tree,
    },
    alignment::Vertical,
    mouse,
    widget::{image, text},
};

use crate::{App, Message, control::Content, widget::BOLD};

const DEFAULT_IMAGE: &[u8] = include_bytes!("../cross.jpg");

impl App {
    pub fn view_display(&self, content: Content) -> Element<'_, Message> {
        if let Some(song) = &self.songs[content as usize].current_song() {
            let title = song.title(&self.books);
            Display::new(
                &song.lyrics.get(song.current).as_str(),
                &title,
                self.resolution,
            )
            .into()
        } else {
            image(Handle::from_bytes(Bytes::from_static(DEFAULT_IMAGE))).into()
        }
    }
}

struct Display {
    resolution: Size,
    title: String,
    lyrics: String,
    font_size: f32,
    image: Handle,
}

impl Display {
    fn new(lyrics: &str, title: &str, resolution: Size) -> Self {
        Self {
            resolution: resolution,
            title: title.to_string(),
            lyrics: lyrics.to_string(),
            font_size: 40.0,
            image: Handle::from_bytes(Bytes::from_static(DEFAULT_IMAGE)),
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
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(iced::ContentFit::Contain.fit(self.resolution, limits.max()))
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
        use std::time::Instant;
        let t1 = Instant::now();
        // Background
        image::draw(
            renderer,
            layout,
            viewport,
            &self.image,
            iced::ContentFit::Contain,
            image::FilterMethod::Linear,
            iced::Rotation::default(),
            1.0,
            1.0,
        );
        println!("t1: {}ms", t1.elapsed().as_millis());
        let bounds = layout.bounds();
        let scale_factor = bounds.width / self.resolution.width;
        // Title
        renderer.fill_text(
            Text {
                content: self.title.clone(),
                bounds: bounds.size(),
                size: (self.font_size * scale_factor / 2.0).into(),
                line_height: text::LineHeight::default(),
                font: BOLD,
                align_x: text::Alignment::Left,
                align_y: Vertical::Bottom,
                shaping: text::Shaping::default(),
                wrapping: text::Wrapping::default(),
            },
            iced::Point {
                x: layout.position().x,
                y: layout.position().y + bounds.height,
            },
            Color::WHITE,
            *viewport,
        );
        let lyrics = Text {
            content: self.lyrics.clone(),
            bounds: bounds.size(),
            size: (self.font_size * scale_factor).into(),
            line_height: text::LineHeight::Relative(1.5),
            font: BOLD,
            align_x: text::Alignment::Center,
            align_y: Vertical::Center,
            shaping: text::Shaping::default(),
            wrapping: text::Wrapping::default(),
        };
        // Stroke
        renderer.with_translation(Vector::new(-0.5, -0.5), |renderer| {
            renderer.fill_text(lyrics.clone(), bounds.center(), Color::BLACK, *viewport)
        });
        renderer.with_translation(Vector::new(1.0, 1.0), |renderer| {
            renderer.fill_text(lyrics.clone(), bounds.center(), Color::BLACK, *viewport)
        });
        // Lyrics
        renderer.fill_text(lyrics, bounds.center(), Color::WHITE, *viewport);
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
