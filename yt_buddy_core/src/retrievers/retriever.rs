use async_trait::async_trait;
use llm_chain::schema::Document;

#[derive(Debug, thiserror::Error)]
pub enum RetrieverError {
    #[error("Failed to retrieve documents: {0}")]
    VectorStoreError(String),
}

#[async_trait]
pub trait Retriever<M> {
    async fn get_relevant_documents(
        &self,
        query: &str,
        limit: u32,
    ) -> Result<Vec<Document<M>>, RetrieverError>;
}
