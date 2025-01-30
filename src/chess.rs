use anyhow::anyhow;
use serde::Deserialize;

use crate::{Board, IntoBoard};

#[derive(Deserialize)]
pub struct Fen(String);

impl IntoBoard for Fen {
    type Piece = ChessPiece;

    fn into_board(self) -> Result<Board<Self::Piece>, anyhow::Error> {
        let mut board = Board::<ChessPiece>::new();

        // Isolate the piece positions from the rest of FEN notation
        let board_part = self.0.split_whitespace().next().unwrap_or(self.0.as_ref());

        // Split the piece positions by row
        let rows: Vec<&str> = board_part.split('/').collect();

        if rows.len() != 8 {
            return Err(anyhow!("Invalid FEN: must have 8 rows"));
        }

        // Process each char of each row
        for (row_index, row) in rows.iter().enumerate() {
            let mut col_index: usize = 0;

            for c in row.chars() {
                if col_index == 8 {
                    // The row contains information for too many pieces (>8)
                    return Err(anyhow!("Invalid FEN: row: {row}"));
                } else if c.is_ascii_digit() {
                    if let Some(digit) = c.to_digit(10) {
                        if digit < 1 || digit as usize > (8 - col_index) {
                            // The digit in the row claimed 0 or too many empty pieces
                            return Err(anyhow!("Invalid FEN: row: {row}"));
                        }
                        col_index += digit as usize;
                    } else {
                        return Err(anyhow!("Character {c} failed to convert to digit"));
                    }
                } else {
                    let piece = ChessPiece::from_char(c)?;
                    // Shift PieceState value left col columns and bitwise OR with current row state
                    board.set_piece(row_index, col_index, piece);
                    col_index += 1;
                }
            }
        }

        Ok(board)
    }
}

/// Chess pieces
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub enum ChessPiece {
    #[default]
    Empty = 0,
    WPawn = 2,
    BPawn = 3,
    WKnight = 4,
    BKnight = 5,
    WBishop = 6,
    BBishop = 7,
    WRook = 8,
    BRook = 9,
    WQueen = 10,
    BQueen = 11,
    WKing = 12,
    BKing = 13,
}

impl ChessPiece {
    /// Maps FEN piece notation to PieceState Enum
    fn from_char(c: char) -> Result<ChessPiece, anyhow::Error> {
        match c {
            'P' => Ok(ChessPiece::WPawn),
            'p' => Ok(ChessPiece::BPawn),
            'N' => Ok(ChessPiece::WKnight),
            'n' => Ok(ChessPiece::BKnight),
            'B' => Ok(ChessPiece::WBishop),
            'b' => Ok(ChessPiece::BBishop),
            'R' => Ok(ChessPiece::WRook),
            'r' => Ok(ChessPiece::BRook),
            'Q' => Ok(ChessPiece::WQueen),
            'q' => Ok(ChessPiece::BQueen),
            'K' => Ok(ChessPiece::WKing),
            'k' => Ok(ChessPiece::BKing),
            _ => Err(anyhow!("Invalid FEN: piece character: {c}")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::<ChessPiece>::new();
        for y in 0..8 {
            for x in 0..8 {
                assert!(matches!(board.get_piece(x, y), ChessPiece::Empty));
            }
        }
    }

    #[test]
    fn test_into_board() {
        let fen = Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_owned());
        let board = fen.into_board().unwrap();

        // black top
        assert!(matches!(board.get_piece(0, 0), ChessPiece::BRook));
        assert!(matches!(board.get_piece(0, 1), ChessPiece::BKnight));
        assert!(matches!(board.get_piece(0, 2), ChessPiece::BBishop));
        assert!(matches!(board.get_piece(0, 3), ChessPiece::BQueen));
        assert!(matches!(board.get_piece(0, 4), ChessPiece::BKing));
        assert!(matches!(board.get_piece(0, 5), ChessPiece::BBishop));
        assert!(matches!(board.get_piece(0, 6), ChessPiece::BKnight));
        assert!(matches!(board.get_piece(0, 7), ChessPiece::BRook));

        // black pawns
        for x in 0..8 {
            assert!(matches!(board.get_piece(1, x), ChessPiece::BPawn));
        }

        // empty rows
        for y in 2..6 {
            for x in 0..8 {
                assert!(matches!(board.get_piece(y, x), ChessPiece::Empty));
            }
        }

        // white pawns
        for x in 0..8 {
            assert!(matches!(board.get_piece(6, x), ChessPiece::WPawn));
        }

        // white bottom
        assert!(matches!(board.get_piece(7, 0), ChessPiece::WRook));
        assert!(matches!(board.get_piece(7, 1), ChessPiece::WKnight));
        assert!(matches!(board.get_piece(7, 2), ChessPiece::WBishop));
        assert!(matches!(board.get_piece(7, 3), ChessPiece::WQueen));
        assert!(matches!(board.get_piece(7, 4), ChessPiece::WKing));
        assert!(matches!(board.get_piece(7, 5), ChessPiece::WBishop));
        assert!(matches!(board.get_piece(7, 6), ChessPiece::WKnight));
        assert!(matches!(board.get_piece(7, 7), ChessPiece::WRook));
    }

    #[test]
    fn test_into_board_not_enough_rows() {
        let fen = Fen("8/8/8/8/8/8/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "Invalid FEN: must have 8 rows"
        );
    }

    #[test]
    fn test_into_board_too_many_rows() {
        let fen = Fen("8/8/8/8/8/8/8/8/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert_eq!(
            format!("{}", result.unwrap_err()),
            "Invalid FEN: must have 8 rows"
        );
    }

    #[test]
    fn test_into_board_row_too_many_columns() {
        let fen = Fen("8/8/8/8/8/8/ppppppppp/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("Invalid FEN: row: ppppppppp"));
    }

    #[test]
    fn test_into_board_invalid_piece_char() {
        // 'x' is not a valid FEN character
        let fen = Fen("8/8/8/8/8/8/8/rnbqkbnx".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("Invalid FEN: piece character: x"));
    }

    #[test]
    fn test_into_board_digit_too_large_for_remaining_columns() {
        let fen = Fen("8/8/8/8/8/8/8/9".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(format!("{}", result.unwrap_err()).contains("Invalid FEN: row: 9"));
    }
}
