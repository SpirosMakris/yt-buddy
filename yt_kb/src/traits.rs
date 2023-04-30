use async_trait::async_trait;
use llm_chain::{
    schema::EmptyMetadata,
    traits::{Embeddings, VectorStore},
};

#[async_trait]
pub trait Ingester<M = EmptyMetadata> {
    type Embeddings: Embeddings;
    type VecStore: VectorStore<Self::Embeddings, M>;

    type Error;

    async fn ingest(&self) -> Result<(), Self::Error>;
}
