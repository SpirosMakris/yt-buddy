use async_trait::async_trait;
use std::io;

pub use llm_chain::schema::Document;

#[derive(thiserror::Error, Debug)]
pub enum LoaderError {
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Error reading file: {0}")]
    FileReadError(String),
    #[error("Source read error: {0}")]
    SourceReadError(String),
}

#[async_trait]
pub trait DocumentLoader<Metadata>
where
    Metadata: serde::Serialize + serde::de::DeserializeOwned,
{
    async fn load(&self) -> Result<Vec<Document<Metadata>>, LoaderError>;

    // fn load_and_split(&self, text_splitter: Option<TextSplitter>) -> Vec<Document>;
}
