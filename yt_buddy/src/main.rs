use salvo::prelude::*;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(Router::with_path("").get(to_search))
        .push(Router::with_path("ingest").get(StaticFile::new("public/ingest.html")))
        .push(Router::with_path("search").get(StaticFile::new("public/search.html")));

    let acceptor = TcpListener::new("127.0.0.1:3000").bind().await;
    Server::new(acceptor).serve(router).await;
}

#[handler]
async fn to_search(res: &mut Response) {
    res.render(Redirect::other("search"));
}
