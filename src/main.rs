use anyhow::{bail, Context, Error};
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
use maud::{html, Markup, DOCTYPE};
use tower_http::cors::CorsLayer;
use tracing::info;

mod error;

#[tokio::main]
async fn main() -> Result<()> {
    // for logging
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .context("error setting global default subscriber")?;

    info!("initializing router...");

    // init our app
    let app = Router::new()
        .route("/", get(index))
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
} // routes
async fn index() -> Markup {
    html! {
        h1 {"Hello World"}
    }
}

// html boilerplate
fn page(title: &str, content: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            (head(title))
            (body(content))
        }
    }
}

fn head(title: &str) -> Markup {
    html! {
        head {
            // metadata
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            title { (title) }
        }
    }
}

fn body(content: Markup) -> Markup {
    html! {
        body {
            (content)
            script src="/_assets/js/htmx.min.js" {}
        }
    }
}
