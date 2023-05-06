use std::sync::Arc;

use async_trait::async_trait;
use llm_chain::schema::EmptyMetadata;
use llm_chain::tokens::TokenizerError;
use llm_chain::traits::VectorStore;
use llm_chain::vectorstores::qdrant::Qdrant;
use llm_chain::TextSplitter;

use qdrant_client::qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig};
use yt_buddy_core::{
    DocumentLoader, LoaderError, RSBertEmbeddings, RSBertError, RsBertTextSplitter,
    YoutubeCaptionsLoader,
};

use qdrant_client::prelude::QdrantClient;

use crate::traits::Ingester;

pub type YTIngestMetadata = EmptyMetadata;

#[derive(Debug, thiserror::Error)]
pub enum YoutubeCaptionsIngesterError {
    #[error(transparent)]
    LoaderError(#[from] LoaderError),
    #[error(transparent)]
    TokenizerError(#[from] TokenizerError),
    #[error(transparent)]
    EmbeddingError(#[from] RSBertError),
    #[error("VectorStore/Client error: {0}")]
    VectorStoreError(String),
    #[error("Model Error: {0}")]
    ModelError(String),
}

pub struct YoutubeCaptionsIngester {
    video_id: String,
    collection_name: String,
    qdrant_client: Arc<QdrantClient>,
    vector_store: Arc<Qdrant<RSBertEmbeddings, YTIngestMetadata>>,
    embeddings_size: u64,
}

impl YoutubeCaptionsIngester {
    pub async fn new(
        video_id: String,
        qdrant_client: Arc<QdrantClient>,
        vector_store: Arc<Qdrant<RSBertEmbeddings, YTIngestMetadata>>,
        collection_name: String,
        embeddings_size: u64,
    ) -> Result<Self, YoutubeCaptionsIngesterError> {
        Ok(Self {
            video_id,
            qdrant_client,
            collection_name,
            vector_store,
            embeddings_size,
        })
    }

    pub async fn ensure_collection(&self) -> Result<(), YoutubeCaptionsIngesterError> {
        if !self
            .qdrant_client
            .has_collection(self.collection_name.clone())
            .await
            .map_err(|e| {
                YoutubeCaptionsIngesterError::VectorStoreError(format!(
                    "Unable to determine if collection exists through client: {e:?}"
                ))
            })?
        {
            // Create the collection
            self.qdrant_client
                .create_collection(&CreateCollection {
                    collection_name: self.collection_name.clone(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                            VectorParams {
                                size: self.embeddings_size,
                                distance: Distance::Cosine.into(),
                                hnsw_config: None,
                                quantization_config: None,
                            },
                        )),
                    }),
                    ..Default::default()
                })
                .await
                .map_err(|e| {
                    YoutubeCaptionsIngesterError::VectorStoreError(format!(
                        "Failed to create collection: {e:?}"
                    ))
                })?;
        }

        Ok(())
    }
}

#[async_trait]
impl Ingester for YoutubeCaptionsIngester {
    type Embeddings = RSBertEmbeddings;
    type VecStore = Qdrant<Self::Embeddings, YTIngestMetadata>;
    type Error = YoutubeCaptionsIngesterError;

    async fn ingest(&self) -> Result<(), Self::Error> {
        // Get captions text from video id
        let loader = YoutubeCaptionsLoader::new(self.video_id.clone());
        let docs = loader.load().await?;

        // Split text into documents
        // Creating the embeddings first will give access to the model
        let embeddings = RSBertEmbeddings::new().expect("Failed to create RsBertEmbeddings");

        let splitter = RsBertTextSplitter::new(embeddings.get_model());
        let split_texts = splitter.split_text(&docs.get(0).unwrap().page_content, 384, 16)?;

        dbg!(&split_texts);

        // Add to vectorstore
        let doc_ids = self
            .vector_store
            .add_texts(split_texts)
            .await
            .map_err(|e| {
                YoutubeCaptionsIngesterError::VectorStoreError(format!(
                    "Failed to add texts to vector store: {e:?}"
                ))
            })?;

        dbg!("Vectors store under IDs: {:?}", doc_ids);

        Ok(())
    }
}
