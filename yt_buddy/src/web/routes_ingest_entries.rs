use axum::{
    extract::{FromRef, Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::model::{IngestEntry, IngestEntryForCreate, ModelController};
use crate::Result;

#[derive(Clone, FromRef)]
struct AppState {
    mc: ModelController,
}

pub fn routes(mc: ModelController) -> Router {
    let app_state = AppState { mc };

    Router::new()
        .route(
            "/ingest",
            post(create_ingest_entry).get(list_ingest_entries),
        )
        .route("/ingest/:id", delete(delete_ingest_entry))
        .with_state(app_state)
}

// region:    --- REST handlers
async fn create_ingest_entry(
    State(mc): State<ModelController>,
    Json(ingest_entry_fc): Json<IngestEntryForCreate>,
) -> Result<Json<IngestEntry>> {
    println!("->> {:<12} - create_ingest_entry", "HANDLER");

    let entry = mc.create_ingest_entry(ingest_entry_fc).await?;

    Ok(Json(entry))
}

async fn list_ingest_entries(State(mc): State<ModelController>) -> Result<Json<Vec<IngestEntry>>> {
    println!("->> {:<12} - list_ingest_entries", "HANDLER");

    let entries = mc.list_ingest_entries().await?;

    Ok(Json(entries))
}

async fn delete_ingest_entry(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<IngestEntry>> {
    println!("->> {:<12} - delete_ingest_entry - {id}", "HANDLER");

    let entry = mc.delete_ingest_entry(id).await?;

    Ok(Json(entry))
}

// endregion: --- REST handlers
