use axum::{http::StatusCode, routing::get, Router};

mod error;
mod v1;

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .nest("/v1", v1::create_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
