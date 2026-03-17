use axum::Router;
use axum::response::Html;
use axum::routing::get;

const INDEX_HTML: &str = include_str!("../static/index.html");

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    let app = Router::new().route("/", get(hello_handler));

    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> Html<&'static str> {
    Html(INDEX_HTML)
}
