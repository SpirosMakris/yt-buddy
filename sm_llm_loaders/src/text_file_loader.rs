use async_trait::async_trait;
use llm_chain::schema::Document;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{DocumentLoader, LoaderError};

pub struct TextFileLoader {
    pub path: String,
}

impl TextFileLoader {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

type TextFileLoaderMetadata = Vec<(String, String)>;

/// This is a DocumentLoader implementation for simple text
/// files. Metadata added is the file source path and the
/// filename.
/// Currently supports UTF-8 encoded text files only.
/// Invalid UTF-8 characters in source file error out.
#[async_trait]
impl DocumentLoader<TextFileLoaderMetadata> for TextFileLoader {
    async fn load(&self) -> Result<Vec<Document<TextFileLoaderMetadata>>, LoaderError> {
        let content = read_text_file(&self.path)?;
        let metadata = vec![("source_file".to_string(), self.path.clone())];

        let doc = Document {
            page_content: content,
            metadata: Some(metadata),
        };

        Ok(vec![doc])
    }
}

fn read_text_file<P: AsRef<Path>>(path: P) -> Result<String, LoaderError> {
    let mut file = File::open(path)?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    Ok(content)
}

// @TODO: add some basic tests for TextFileLoader
