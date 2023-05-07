use std::path::PathBuf;

use async_trait::async_trait;

use crate::{Document, DocumentLoader, LoaderError};

pub struct PdfFileLoader {
    pub path: PathBuf,
}

impl PdfFileLoader {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        Self { path: path.into() }
    }
}

type PdfFileLoaderMetadata = Vec<(String, String)>;

#[async_trait]
impl DocumentLoader<PdfFileLoaderMetadata> for PdfFileLoader {
    async fn load(&self) -> Result<Vec<Document<PdfFileLoaderMetadata>>, LoaderError> {
        let bytes = std::fs::read(self.path.clone())?;
        let content = pdf_extract::extract_text_from_mem(&bytes)
            .map_err(|e| LoaderError::SourceReadError(e.to_string()))?;

        let metadata = vec![(
            "source_file".to_string(),
            self.path.to_string_lossy().to_string(),
        )];

        let doc = Document {
            page_content: content,
            metadata: Some(metadata),
        };

        Ok(vec![doc])
    }
}
