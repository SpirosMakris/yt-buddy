use std::sync::Arc;

use llm_chain::traits::VectorStore;
use llm_chain_qdrant::Qdrant;
use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};
use yt_buddy::{Ingester, YTIngestMetadata, YoutubeCaptionsIngester};
use yt_buddy_core::{RSBertEmbeddings, VectorStoreRetriever};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dbg!("Retriever 1 example");

    dbg!("Creating Qdrant config");
    let config = QdrantClientConfig::from_url("http://localhost:6334");
    let client = Arc::new(
        QdrantClient::new(Some(config))
            .await
            .expect("Failed to create client"),
    );

    let collection_name = "retriever_01".to_string();

    dbg!("Creating Rs Bert Embeddings..");
    let embeddings = RSBertEmbeddings::new().expect("Failed to create RSBertEmbeddings");

    dbg!("Creating ingester..");
    let video_id = "yriZBFKE9JU";

    dbg!("Getting collections..");
    let collections_list = client.list_collections().await.unwrap();
    dbg!(collections_list);

    let embeddings_size = embeddings.get_embeddings_size();

    dbg!("Creating VectorStore..");
    let qdrant_vs: Arc<Qdrant<RSBertEmbeddings, YTIngestMetadata>> = Arc::new(Qdrant::new(
        client.clone(),
        collection_name.clone(),
        embeddings,
        None,
        None,
    ));

    // let blocking_ingester_task = tokio::task::spawn_blocking(move || {
    //     YoutubeCaptionsIngester::new(
    //         video_id.to_string(),
    //         client.clone(),
    //         qdrant_vs.clone(),
    //         collection_name.to_string(),
    //         embeddings_size,
    //     )
    // });

    // let ingester = blocking_ingester_task
    //     .await
    //     .expect("Failed to run ingester creation async task")
    //     .await
    //     .expect("Failed to create Youtube Ingester");

    let ingester = YoutubeCaptionsIngester::new(
        video_id.to_string(),
        client.clone(),
        qdrant_vs.clone(),
        collection_name.to_string(),
        embeddings_size,
    )
    .await
    .expect("Faield to create ingester");

    dbg!("Creating current example collection if not exists..");
    ingester
        .ensure_collection()
        .await
        .expect("Failed to ensure collection exists");

    dbg!("Ingesting..: {video_id}");

    ingester
        .ingest()
        .await
        .expect("Failed to ingest: {video_id}");

    dbg!("Querying data..");
    let res = qdrant_vs
        .clone()
        .similarity_search("language models".to_string(), 2)
        .await
        .expect("Failed to do similarity search");

    dbg!(res);
}
