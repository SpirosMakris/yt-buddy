use async_trait::async_trait;
use llm_chain::{
    schema::EmptyMetadata,
    traits::{Embeddings, VectorStore},
};

use serde::{de::DeserializeOwned, Serialize};

#[async_trait]
pub trait Ingester<M = EmptyMetadata> {
    type Embeddings: Embeddings;
    type VecStore: VectorStore<Self::Embeddings, M>
    where
        M: Serialize + DeserializeOwned;

    type Error;

    async fn ingest(&self) -> Result<(), Self::Error>;
}
