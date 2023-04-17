use std::io;

use crate::schema::Document;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoaderError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Error reading file: {0}")]
    FileReadError(String),
}

pub trait DocumentLoader {
    type Metadata;

    fn load(&self) -> Result<Vec<Document<'_, Self::Metadata>>, LoaderError>;

    // fn load_and_split(&self, text_splitter: Option<TextSplitter>) -> Vec<Document>;
}
