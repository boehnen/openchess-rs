use axum::{extract::Query, routing::get, Router};
use serde::Deserialize;

use crate::error::ApiError;

#[derive(Deserialize)]
struct Fen(String);

impl Fen {
    fn into_board(self: Self) -> Result<Board, ApiError> {
        todo!()
    }
}

enum Piece {
    Knight,
}

struct Board {
    pieces: [[Piece; 8]; 8],
}

impl Board {
    fn render(self: Self, theme: Theme, rotation: u32, labels: bool) -> String {
        todo!()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Theme {
    Classic,
    Modern,
}

#[derive(Deserialize)]
struct Parameters {
    fen: Fen,
    theme: Option<Theme>,
    rotation: Option<u32>,
    labels: Option<bool>,
}

async fn board(params: Query<Parameters>) -> Result<String, ApiError> {
    let params = params.0;
    let theme = params.theme.unwrap_or(Theme::Classic);
    let rotation = params.rotation.unwrap_or(0);
    let labels = params.labels.unwrap_or(false);

    let board = params.fen.into_board()?;

    Ok(board.render(theme, rotation, labels))
}

pub fn create_router() -> Router {
    Router::new().route("/board", get(board))
}
