use std::sync::Arc;

use qdrant_client::prelude::{QdrantClient, QdrantClientConfig};
use yt_buddy::{Ingester, YoutubeCaptionsIngester};
use yt_buddy_core::RSBertEmbeddings;

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

    let collection_name = "retriever_01";

    dbg!("Creating Rs Bert Embeddings..");
    let embeddings = RSBertEmbeddings::new().expect("Failed to create RSBertEmbeddings");

    dbg!("Creating ingester..");
    let video_id = "yriZBFKE9JU";

    dbg!("Getting collections..");
    let collections_list = client.list_collections().await.unwrap();
    dbg!(collections_list);

    let blocking_ingester_task = tokio::task::spawn_blocking(move || {
        YoutubeCaptionsIngester::new(
            video_id.to_string(),
            client.clone(),
            collection_name.to_string(),
            embeddings,
        )
    });

    let ingester = blocking_ingester_task
        .await
        .expect("Failed to run ingester creation async task")
        .await
        .expect("Failed to create Youtube Ingester");

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
}
