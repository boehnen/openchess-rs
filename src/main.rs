use axum::{extract::Query, http::StatusCode, routing::get, Router};
use lib_openchess::{chess::Fen, IntoBoard, IntoSvg};
use serde::Deserialize;

#[derive(Deserialize)]
struct ChessParams {
    fen: Fen,
}

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

async fn chess(params: Query<ChessParams>) -> Result<String, String> {
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
