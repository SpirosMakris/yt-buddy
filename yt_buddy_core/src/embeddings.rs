use std::sync::Mutex;

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
    #[error("Model Mutex is poisoned")]
    MutexPoisonError,
    #[error("Empty embeddings returned")]
    EmptyEmbeddings,
}

impl traits::EmbeddingsError for RSBertError {}

pub struct RSBertEmbeddings {
    model: Mutex<SentenceEmbeddingsModel>,
}

impl RSBertEmbeddings {
    pub fn new() -> Result<Self, RSBertError> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()?;

        Ok(Self {
            model: Mutex::new(model),
        })
    }

    pub fn from_model(model_type: SentenceEmbeddingsModelType) -> Result<Self, RSBertError> {
        let model = SentenceEmbeddingsBuilder::remote(model_type).create_model()?;

        Ok(Self {
            model: Mutex::new(model),
        })
    }

    pub fn get_model(&self) -> &Mutex<SentenceEmbeddingsModel> {
        &self.model
    }
}

#[async_trait]
impl traits::Embeddings for RSBertEmbeddings {
    type Error = RSBertError;

    async fn embed_texts(&self, texts: Vec<String>) -> Result<Vec<Vec<f32>>, Self::Error> {
        let encoded = self
            .model
            .lock()
            .map_err(|_e| RSBertError::MutexPoisonError)?
            .encode(texts.as_slice())?;

        Ok(encoded)
    }

    async fn embed_query(&self, query: String) -> Result<Vec<f32>, Self::Error> {
        let encoded = self
            .model
            .lock()
            .map_err(|_e| RSBertError::MutexPoisonError)?
            .encode(&[query])?
            .get(0)
            .cloned()
            .ok_or(RSBertError::EmptyEmbeddings)?;

        Ok(encoded)
    }
}
