use std::path::PathBuf;

use pdfium_render::prelude::Pdfium;

use crate::content::document::Document;

#[derive(Debug)]
pub struct LibDoc {
    pub index: Vec<Document>,
    pub current: u16,
    pub pdfium: Pdfium,
    pub cache_path: PathBuf,
}

impl LibDoc {
    pub fn push(&mut self, doc: Document) {
        self.index.push(doc);
    }

    pub fn current(&self) -> &Document {
        &self.index[self.current as usize]
    }
}
