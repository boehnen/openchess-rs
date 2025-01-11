use axum::{http::StatusCode, routing::get, Router};
mod chess;

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    println!("Testing chessboard construction from fen");
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let board = chess::board::ChessBoard::from_fen(fen);
    match board {
        Ok(board) => println!("{:?}", board),
        Err(msg) => println!("{}", msg)
    }

    let app = Router::new().route("/healthcheck", get(healthcheck));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
