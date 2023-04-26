use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;
use surrealdb::{engine::remote::ws::Ws, opt::auth::Root, Surreal};

use yt_kb_core::{DocumentLoader, YoutubeCaptionsLoader};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum IngestStatus {
    Pending,
    Loading,
    Loaded,
    Embedding,
    Done,
}

#[derive(Debug, Serialize, Deserialize)]
struct IngestYTVideoItem {
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    id: Option<Thing>,
    video_id: String,
    status: IngestStatus,
    text: Option<String>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

// #[derive(Debug, Deserialize)]
// struct IngestYTVideoRecord {
//     id: Thing,
// }

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    println!("Connecting to surrealdb..");

    // Connect to server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;
    // println!("{db:?}");

    // Sign-in as a namespace, db or root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace/db
    db.use_ns("test").use_db("yt_kb_db").await?;

    let video_id = "IqqHqDcXLww";

    let id = format!("{video_id}|{}", uuid::Uuid::new_v4());

    // Create a new ingest queue entry
    let _created_entry: IngestYTVideoItem = db
        .create(("ingest_queue", id.clone()))
        .content(IngestYTVideoItem {
            id: None,
            video_id: video_id.to_string(),
            text: None,
            status: IngestStatus::Pending,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
        .await?;

    // dbg!(created_entry);

    // Select all ingest records
    // let ingest_items: Vec<IngestYTVideoItem> = db.select("ingest_queue").await?;

    // dbg!(ingest_items);

    // Select oldest entry
    let oldest_queue_entry: Option<IngestYTVideoItem> = db
        .query("SELECT * FROM type::table($table) WHERE status='pending' ORDER BY created_at ASC LIMIT 1;")
        .bind(("table", "ingest_queue"))        
        .await?
        .take(0)?;

    dbg!(&oldest_queue_entry);

    if let Some(item) = oldest_queue_entry {
        let loader = YoutubeCaptionsLoader::new(item.video_id.clone());
        let res = loader.load().await.unwrap();

        dbg!(&res);
        dbg!(item);

        // debug_assert!(&res.len() == 1usize);

        let text = res.get(0).map(|x| x.page_content.clone());

        let update_res: IngestYTVideoItem = db
            .update(("ingest_queue", id.clone()))
            .merge(json!({ "status": IngestStatus::Done, "text": text }))
            .await?;

        dbg!(update_res);
    }

    Ok(())
}
