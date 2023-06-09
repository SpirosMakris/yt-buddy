use async_trait::async_trait;
use llm_chain::traits::VectorStore;
use llm_chain::{schema::Document, traits::Embeddings};
use llm_chain_qdrant::Qdrant;
use qdrant_client::qdrant::Value;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::convert::TryFrom;
use std::marker::{Send, Sync};

use crate::{Retriever, RetrieverError};

pub trait QdrantMetadata:
    TryFrom<Value> + Into<Value> + Send + Sync + Serialize + DeserializeOwned
{
}

pub struct VectorStoreRetriever<E, M>
where
    E: Embeddings + Send + Sync,
    M: QdrantMetadata,
{
    vector_store: Qdrant<E, M>,
}

impl<E, M> VectorStoreRetriever<E, M>
where
    E: Embeddings + Send + Sync,
    M: QdrantMetadata,
{
    // @TODO: impl new() ?
}

#[async_trait]
impl<E: Embeddings + Send + Sync, M: QdrantMetadata> Retriever<M> for VectorStoreRetriever<E, M> {
    async fn get_relevant_documents(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<Document<M>>, RetrieverError> {
        Ok(self
            .vector_store
            .similarity_search(query.to_string(), limit)
            .await
            .map_err(|e| RetrieverError::VectorStoreError(e.to_string()))?)
    }
}
