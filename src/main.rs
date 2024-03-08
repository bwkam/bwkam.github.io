use axum::{
    extract::Path,
    http::{
        header::{self, ACCEPT, AUTHORIZATION, CONTENT_TYPE, LAST_MODIFIED},
        HeaderMap, HeaderValue, Method, StatusCode,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use error::Result;
use tower_http::cors::CorsLayer;

mod error;
mod page;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // for logging
    tracing_subscriber::fmt::init();

    // init our app
    let app = Router::new()
        .route("/", get(routes::index::index))
        .route("/hello", get(hello))
        .route("/_assets/*path", get(assets))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET])
                .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE, LAST_MODIFIED]),
        );

    // serve
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn assets(Path(path): Path<String>) -> impl IntoResponse {
    let mut headers = HeaderMap::new();
    let content = tokio::fs::read_to_string(format!("./assets/{}", path)).await;

    match content {
        Ok(content) => {
            if path.ends_with(".css") {
                headers.insert(header::CONTENT_TYPE, "text/css".parse().unwrap());
            } else if path.ends_with(".js") {
                headers.insert(header::CONTENT_TYPE, "text/javascript".parse().unwrap());
            } else if path.ends_with(".svg") {
                headers.insert(header::CONTENT_TYPE, "image/svg+xml".parse().unwrap());
            }

            (StatusCode::OK, headers, content)
        }
        Err(_) => (StatusCode::NOT_FOUND, headers, "".to_string()),
    }
}
async fn hello() -> impl IntoResponse {
    "hello world"
}
