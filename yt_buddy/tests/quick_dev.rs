use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    // Hello routes
    hc.do_get("/hello2/Spiros").await?.print().await?;

    // Static routes
    // hc.do_get("/ingest.html").await?.print().await?;

    // API login
    let req_login = hc.do_post(
        "/api/login",
        json!({
          "username": "demo",
          "pwd": "demo",
        }),
    );
    req_login.await?.print().await?;

    // Just to test that login cookie persists
    hc.do_get("/hello2/Spiros").await?.print().await?;

    // Ingest API
    let req_create_ingest_entry = hc.do_post(
        "/api/ingest",
        json!({
          "video_id": "https://www.youtube.com/watch?v=9bZkp7q19f0",
        }),
    );
    req_create_ingest_entry.await?.print().await?;

    hc.do_get("/api/ingest").await?.print().await?;

    hc.do_delete("/api/ingest/0").await?.print().await?;

    hc.do_get("/api/ingest").await?.print().await?;

    Ok(())
}
