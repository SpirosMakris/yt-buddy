use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

use crate::{
    error::{Error, Result},
    web,
};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

    // @TODO: Implement real db/auth
    if payload.username != "demo" || payload.pwd != "demo" {
        return Err(Error::LoginFail);
    }

    // @TODO, @FIXME: Implement real auth-token generation/signature
    // Set cookie
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

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
