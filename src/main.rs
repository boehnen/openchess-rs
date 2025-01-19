use axum::{
    extract::Query,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use lib_openchess::{chess::Fen, IntoBoard, IntoSvg};
use serde::Deserialize;
use thiserror::Error;
use tracing::{error, info};

mod logger;

#[derive(Error, Debug)]
enum ApiError {
    #[error("internal logic error: {0}")]
    LogicError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        error!("API error occurred: {}", self);

        match self {
            Self::LogicError(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", error),
            ),
        }
        .into_response()
    }
}

#[derive(Deserialize)]
struct ChessParams {
    fen: Fen,
}

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

async fn chess(params: Query<ChessParams>) -> Result<String, ApiError> {
    let params = params.0;

    let board = params.fen.into_board()?;

    Ok(board.into_svg(()))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/chess", get(chess))
        .layer(middleware::from_fn(logger::print_request_response));

    info!("Starting server on 0.0.0.0:8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
