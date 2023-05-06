use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use llm_chain::traits;
use rust_bert::{
    pipelines::sentence_embeddings::{
        SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
    },
    RustBertError,
};

use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub enum RSBertError {
    #[error(transparent)]
    BertError(#[from] RustBertError),
    #[error("Model Mutex is poisoned: {0}")]
    MutexPoisonError(String),
    #[error("Model error: {0}")]
    ModelError(String),
    #[error("Empty embeddings returned")]
    EmptyEmbeddings,
}

impl traits::EmbeddingsError for RSBertError {}

pub struct RSBertEmbeddings {
    model: Arc<Mutex<SentenceEmbeddingsModel>>,
    embeddings_size: u64,
}

impl RSBertEmbeddings {
    pub fn new() -> Result<Self, RSBertError> {
        let model =
            SentenceEmbeddingsBuilder::local("resources/all-MiniLM-L12-v2").create_model()?;

        let embeddings_size = Self::init_embeddings_size(&model)?;

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            embeddings_size,
        })
    }

    pub fn from_model(model_type: SentenceEmbeddingsModelType) -> Result<Self, RSBertError> {
        let model =
            SentenceEmbeddingsBuilder::local("resources/all-MiniLM-L12-v2").create_model()?;

        let embeddings_size = Self::init_embeddings_size(&model)?;

        Ok(Self {
            model: Arc::new(Mutex::new(model)),
            embeddings_size,
        })
    }

    pub fn get_model(&self) -> Arc<Mutex<SentenceEmbeddingsModel>> {
        self.model.clone()
    }

    pub fn get_embeddings_size(&self) -> u64 {
        self.embeddings_size
    }

    fn init_embeddings_size(model: &SentenceEmbeddingsModel) -> Result<u64, RSBertError> {
        // Encode a single dummy entry to get embeddings length
        // We need this for initializing the Qdrant collection if
        // it doesn't exist.
        Ok(model
            // .lock()
            // .map_err(|e| RSBertError::MutexPoisonError(e.to_string()))?
            .encode(&["dummy val"])
            .map_err(|e| RSBertError::ModelError(e.to_string()))?
            .get(0)
            .ok_or(RSBertError::ModelError(
                "Unable to fetch encoding. This may indicate problems with the model".to_string(),
            ))?
            .len() as u64)
    }
}

#[async_trait]
impl traits::Embeddings for RSBertEmbeddings {
    type Error = RSBertError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        let encoded = self
            .model
            .lock()
            .map_err(|e| RSBertError::MutexPoisonError(e.to_string()))?
            .encode(texts.as_slice())?;

        Ok(encoded)
    }

    async fn embed_query(&self, query: String) -> Result<Vec<f32>, Self::Error> {
        let encoded = self
            .model
            .lock()
            .map_err(|e| RSBertError::MutexPoisonError(e.to_string()))?
            .encode(&[query])?
            .get(0)
            .cloned()
            .ok_or(RSBertError::EmptyEmbeddings)?;

        Ok(encoded)
    }
}
