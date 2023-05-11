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
          "pwd": "demo1",
        }),
    );
    req_login.await?.print().await?;

    Ok(())
}
