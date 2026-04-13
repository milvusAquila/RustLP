use std::path::PathBuf;

use pdfium_render::prelude::PdfRenderConfig;

use crate::{Message, library::LibDoc};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Document {
    Pdf(Pdf),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pdf {
    pub path: PathBuf,
    pub nb_pages: u16,
    pub name: String,
    pub current: Option<usize>,
}

impl Pdf {
    pub fn new(libdoc: &LibDoc, resolution: iced::Size, path: PathBuf) -> Self {
        let doc = libdoc.pdfium.load_pdf_from_file(&path, None).expect("PDF");
        let nb_pages = doc.pages().len();
        let name = path
            .file_name()
            .expect("ERROR: Should be a file not a folder")
            .display()
            .to_string();

        let render_config = PdfRenderConfig::new()
            .set_target_size(resolution.width as i32, resolution.height as i32);

        for (index, page) in doc.pages().iter().enumerate() {
            let mut current_path = libdoc.cache_path.clone();
            current_path.push(format!("{}-{}.png", name, index));
            page.render_with_config(&render_config)
                .expect("ERROR: Can't render pdf")
                .as_image()
                .expect("ERROR: Can't convert pdf into image")
                .save_with_format(current_path.as_os_str(), ::image::ImageFormat::Png)
                .expect("ERROR: Can't write image to cache");
        }
        Self {
            path: path,
            nb_pages: nb_pages as u16,
            name: name,
            current: None,
        }
    }

    pub fn title(&self) -> String {
        self.name.clone()
    }

    pub fn set_previous(&mut self) {
        if let Some(current) = self.current {
            if current > 0 {
                self.current = Some(current - 1)
            }
        }
    }

    pub fn set_next(&mut self) {
        if let Some(current) = self.current {
            if current < self.nb_pages as usize {
                self.current = Some(current + 1)
            }
        }
    }

    pub fn preview(&self, app: &crate::App) -> iced::widget::Container<'_, Message, iced::Theme> {
        use iced::widget::{Column, container, image};
        let mut col = Column::with_capacity(self.nb_pages as usize);
        for i in 0..self.nb_pages {
            let mut new_path = app.library.docs.cache_path.clone();
            new_path.push(format!("{}-{}.png", self.name, i));
            col = col.push(image(new_path)); //.content_fit(iced::ContentFit::Cover));
        }
        container(col)
    }
}

// fn open_file(
//     window: &dyn Window,
// ) -> impl Future<Output = Result<(PathBuf, Arc<String>), Error>> + use<> {
//     let dialog = rfd::AsyncFileDialog::new()
//         .set_title("Open a text file...")
//         .set_parent(&window);
//
//     async move {
//         let picked_file = dialog.pick_file().await.ok_or(Error::DialogClosed)?;
//
//         load_file(picked_file).await
//     }
// }
