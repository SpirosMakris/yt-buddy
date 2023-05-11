use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

    // @TODO: Implement real db/auth
    if payload.username != "demo" || payload.pwd != "demo" {
        return Err(Error::LoginFail);
    }

    // @TODO: Set cookie

    // Create the success body
    let body = Json(json!({
      "result": {
        "success": true,
      }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
