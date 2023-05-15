use axum::{
    extract::{FromRef, Path, State},
    routing::{delete, post},
    Json, Router,
};

use crate::Result;
use crate::{
    ctx::Ctx,
    model::{IngestEntry, IngestEntryForCreate, ModelController},
};

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
    ctx: Ctx,
    Json(ingest_entry_fc): Json<IngestEntryForCreate>,
) -> Result<Json<IngestEntry>> {
    println!("->> {:<12} - create_ingest_entry", "HANDLER");

    let entry = mc.create_ingest_entry(ctx, ingest_entry_fc).await?;

    Ok(Json(entry))
}

async fn list_ingest_entries(
    State(mc): State<ModelController>,
    ctx: Ctx,
) -> Result<Json<Vec<IngestEntry>>> {
    println!("->> {:<12} - list_ingest_entries", "HANDLER");

    let entries = mc.list_ingest_entries(ctx).await?;

    Ok(Json(entries))
}

async fn delete_ingest_entry(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<IngestEntry>> {
    println!("->> {:<12} - delete_ingest_entry - {id}", "HANDLER");

    let entry = mc.delete_ingest_entry(ctx, id).await?;

    Ok(Json(entry))
}

// endregion: --- REST handlers
