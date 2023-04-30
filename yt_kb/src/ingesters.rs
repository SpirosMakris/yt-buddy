use std::sync::Arc;

use async_trait::async_trait;
use llm_chain::tokens::TokenizerError;
use llm_chain::traits::{VectorStore, VectorStoreError};
use llm_chain::vectorstores::qdrant::Qdrant;
use llm_chain::TextSplitter;
use llm_chain::{schema::EmptyMetadata, traits::Embeddings};

use qdrant_client::qdrant::{CreateCollection, Distance, VectorParams, VectorsConfig};
use yt_kb_core::{
    DocumentLoader, LoaderError, RSBertEmbeddings, RSBertError, RsBertTextSplitter,
    YoutubeCaptionsLoader,
};

use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};

use crate::traits::Ingester;

type YTIngestMetadata = EmptyMetadata;

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
    client: Arc<QdrantClient>,
    vector_store: Qdrant<RSBertEmbeddings, YTIngestMetadata>,
    embeddings_size: u64,
}

impl YoutubeCaptionsIngester {
    pub async fn new(
        video_id: String,
        qdrant_config: QdrantClientConfig,
        collection_name: String,
        embeddings: RSBertEmbeddings,
    ) -> Result<Self, YoutubeCaptionsIngesterError> {
        let client = Arc::new(
            QdrantClient::new(Some(qdrant_config))
                .await
                .map_err(|e| YoutubeCaptionsIngesterError::VectorStoreError(e.to_string()))?,
        );

        // Encode a single dummy entry to get embeddings length
        // We need this for initializing the Qdrant collection if
        // it doesn't exist.
        let embeddings_size = embeddings
            .get_model()
            .lock()
            .map_err(|e| YoutubeCaptionsIngesterError::ModelError(e.to_string()))?
            .encode(&["dummy val"])
            .map_err(|e| YoutubeCaptionsIngesterError::ModelError(e.to_string()))?
            .get(0)
            .ok_or(YoutubeCaptionsIngesterError::ModelError(
                "Unable to fetch encoding. This may indicate problems with the model".to_string(),
            ))?
            .len() as u64;

        debug_assert_eq!(384, embeddings_size);

        let qdrant: Qdrant<RSBertEmbeddings, YTIngestMetadata> = Qdrant::new(
            client.clone(),
            collection_name.clone(),
            embeddings,
            None,
            None,
        );

        Ok(Self {
            video_id,
            client,
            collection_name,
            vector_store: qdrant,
            embeddings_size,
        })
    }

    pub async fn ensure_collection(&self) -> Result<(), YoutubeCaptionsIngesterError> {
        if !self
            .client
            .has_collection(self.collection_name.clone())
            .await
            .map_err(|e| {
                YoutubeCaptionsIngesterError::VectorStoreError(format!(
                    "Unable to determine if collection exists through client: {e:?}"
                ))
            })?
        {
            // Create the collection
            self.client
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

        // Embedding create for each document
        // let embedded_texts = embeddings.embed_texts(split_texts).await?;

        // dbg!(&embedded_texts);
        // dbg!("num embedded texts: ", &embedded_texts.len());

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
