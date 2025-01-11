use axum::{http::StatusCode, routing::get, Router};
mod chess;

use chess::board::ChessBoard;
use chess::piece::PieceState;

async fn healthcheck() -> StatusCode {
    StatusCode::OK
}

#[tokio::main]
async fn main() {
    let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let board = ChessBoard::from_fen(fen);
    match board {
        Ok(board) => assert_board_state(board),
        Err(msg) => println!("{}", msg)
    }

    let app = Router::new().route("/healthcheck", get(healthcheck));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

fn assert_board_state(board: ChessBoard){   
    println!("Testing chessboard construction from fen");         
    println!("{:?}", board);

    // black top
    assert!(matches!(board.get_piece(0, 0), PieceState::BRook));
    assert!(matches!(board.get_piece(0, 1), PieceState::BKnight));
    assert!(matches!(board.get_piece(0, 2), PieceState::BBishop));
    assert!(matches!(board.get_piece(0, 3), PieceState::BQueen));
    assert!(matches!(board.get_piece(0, 4), PieceState::BKing));
    assert!(matches!(board.get_piece(0, 5), PieceState::BBishop));
    assert!(matches!(board.get_piece(0, 6), PieceState::BKnight));
    assert!(matches!(board.get_piece(0, 7), PieceState::BRook));
    
    // black pawns
    for x in 0..7 {
        assert!(matches!(board.get_piece(1, x), PieceState::BPawn));
    }

    // empty rows
    for y in 2..5 {
        for x in 0..7 {
            assert!(matches!(board.get_piece(y, x), PieceState::Empty));
        }
    }

    // white pawns
    for x in 0..7 {
        assert!(matches!(board.get_piece(6, x), PieceState::WPawn));
    }

    // white bottom
    assert!(matches!(board.get_piece(7, 0), PieceState::WRook));
    assert!(matches!(board.get_piece(7, 1), PieceState::WKnight));
    assert!(matches!(board.get_piece(7, 2), PieceState::WBishop));
    assert!(matches!(board.get_piece(7, 3), PieceState::WQueen));
    assert!(matches!(board.get_piece(7, 4), PieceState::WKing));
    assert!(matches!(board.get_piece(7, 5), PieceState::WBishop));
    assert!(matches!(board.get_piece(7, 6), PieceState::WKnight));
    assert!(matches!(board.get_piece(7, 7), PieceState::WRook));

    println!("Board state verified");
}