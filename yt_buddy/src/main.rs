use salvo::prelude::*;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("").get(to_search))
        .push(Router::with_path("ingest").get(StaticFile::new("public/ingest.html")))
        .push(Router::with_path("search").get(StaticFile::new("public/search.html")))
        .push(
            Router::with_path("api/v1")
                .push(Router::with_path("ingest").post(ingest_video))
                .push(Router::with_path("query").post(query_collection)),
        );

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn to_search(res: &mut Response) {
    res.render(Redirect::other("search"));
}

#[derive(Debug, Deserialize)]
pub struct IngestVideoRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct IngestVideoResponse {
    pub status: String,
}

#[handler]
async fn ingest_video(req: &mut Request, res: &mut Response) {
    let req_data = req.parse_body::<IngestVideoRequest>().await.unwrap();
    println!("req_data: {:?}", req_data);

    res.render(Json(IngestVideoResponse {
        status: "ok".to_string(),
    }));
}

#[derive(Debug, Deserialize)]
pub struct QueryRequest {
    pub query: String,
    pub collection: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct QueryResponse {
    pub status: String,
    pub result: String,
}

#[handler]
async fn query_collection(req: &mut Request, res: &mut Response) {
    let req_data = req.parse_body::<QueryRequest>().await.unwrap();
    println!("req_data: {:?}", req_data);

    res.render(Json(QueryResponse {
        status: "ok".to_string(),
        result: "lalalalala".to_string(),
    }));
}
