use axum::handler::Handler;
use axum::routing::get;

#[tokio::main]
async fn main() {
    // Build our application by creating our router
    let app = axum::Router::new()
        .fallback(fallback.into_service())
        .route("/", get(hello));

    // Run our application as a hyper server on http://localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

/// axum handler for any request that fails to match the router routes.
/// This implementaion returns HTTP status code Not Found(404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

/// Tokio signal handler that will wait for a user to press CTRL+C
/// We use this in our hyper `Server` method `with_graceful_shutdown`.`
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}