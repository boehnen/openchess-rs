use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use lib_openchess::{chess::Fen, IntoBoard, IntoSvg};
use serde::Deserialize;
use thiserror::Error;

#[derive(Error, Debug)]
enum ApiError {
    #[error("internal logic error: {0}")]
    LogicError(#[from] anyhow::Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
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
    let app = Router::new()
        .route("/healthcheck", get(healthcheck))
        .route("/chess", get(chess));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
