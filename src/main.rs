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
use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};

mod error;
mod page;
mod routes;

#[tokio::main]
async fn main() -> Result<()> {
    // for logging
    tracing_subscriber::fmt::init();

    // static files
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/index.html"));

    // init our app
    let app = Router::new()
        .route("/", get(routes::index::index))
        .route("/hello", get(hello))
        .nest_service("/assets", serve_dir.clone())
        .fallback_service(serve_dir)
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

async fn hello() -> impl IntoResponse {
    "hello world"
}
