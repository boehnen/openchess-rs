use serde::Deserialize;

use crate::{IntoBoard, IntoSvg};

#[derive(Deserialize)]
pub struct Fen(String);

impl IntoBoard for Fen {
    type Board = Board;

    fn into_board(self: Self) -> Result<Self::Board, String> {
        let mut board = Board::new();

        // Isolate the piece positions from the rest of FEN notation
        let board_part = self.0.split_whitespace().next().unwrap_or(self.0.as_ref());

        // Split the piece positions by row
        let rows: Vec<&str> = board_part.split('/').collect();

        if rows.len() != 8 {
            return Err(format!("Invalid FEN: must have 8 rows"));
        }

        // Process each char of each row
        for (row_index, row) in rows.iter().enumerate() {
            let mut col_index = 0;

            for c in row.chars() {
                if col_index == 8 {
                    // The row contains information for too many pieces (>8)
                    return Err(format!("Invalid FEN: row: {row}"));
                } else if c.is_ascii_digit() {
                    if let Some(digit) = c.to_digit(10) {
                        if digit < 1 || digit > (8 - col_index) {
                            // The digit in the row claimed 0 or too many empty pieces
                            return Err(format!("Invalid FEN: row: {row}"));
                        }
                        col_index += digit;
                    } else {
                        return Err(format!("Character {c} failed to convert to digit"));
                    }
                } else {
                    let piece = Piece::from_char(c)?;
                    // Shift PieceState value left col columns and bitwise OR with current row state
                    board.set_piece(row_index as u32, col_index, piece);
                    col_index += 1;
                }
            }
        }

        Ok(board)
    }
}

/// Chess pieces
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum Piece {
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

impl Piece {
    /// Maps FEN piece notation to PieceState Enum
    fn from_char(c: char) -> Result<Piece, String> {
        match c {
            'P' => Ok(Piece::WPawn),
            'p' => Ok(Piece::BPawn),
            'N' => Ok(Piece::WKnight),
            'n' => Ok(Piece::BKnight),
            'B' => Ok(Piece::WBishop),
            'b' => Ok(Piece::BBishop),
            'R' => Ok(Piece::WRook),
            'r' => Ok(Piece::BRook),
            'Q' => Ok(Piece::WQueen),
            'q' => Ok(Piece::BQueen),
            'K' => Ok(Piece::WKing),
            'k' => Ok(Piece::BKing),
            _ => Err(format!("Invalid FEN: piece character: {c}")),
        }
    }
}

/// Board state structure
#[derive(Clone, Copy, Debug)]
pub struct Board {
    rows: [u32; 8], // 8 rows of 32 bits, each piece is allocated 4 bits to represent 13 possible states
}

impl Board {
    /// Initializes a new ChessBoard with empty pieces
    pub fn new() -> Self {
        Self { rows: [0; 8] }
    }

    /// Returns an amount to shift a PieceState value to affect the desired column
    fn col_shift(col: u32) -> u32 {
        col * 4 // each piece is allocated 4 bits
    }

    /// Returns a mask that singles out the desired column
    /// e.g. col 0: 0x0000000F
    /// e.g. col 7: 0xF0000000
    fn col_mask(col: u32) -> u32 {
        0xF << Self::col_shift(col)
    }

    /// Returns the PieceState at the given position
    pub fn get_piece(&self, row: u32, col: u32) -> Piece {
        // mask the column from the row then shift to lower 4 bits
        let nibble = (self.rows[row as usize] & Self::col_mask(col)) >> Self::col_shift(col);
        match nibble {
            0 => Piece::Empty,
            2 => Piece::WPawn,
            3 => Piece::BPawn,
            4 => Piece::WKnight,
            5 => Piece::BKnight,
            6 => Piece::WBishop,
            7 => Piece::BBishop,
            8 => Piece::WRook,
            9 => Piece::BRook,
            10 => Piece::WQueen,
            11 => Piece::BQueen,
            12 => Piece::WKing,
            13 => Piece::BKing,
            _ => unreachable!("Invalid nibble: {}", nibble),
        }
    }

    pub fn set_piece(self: &mut Self, row: u32, col: u32, piece: Piece) -> () {
        self.rows[row as usize] |= (piece as u32) << Self::col_shift(col);
    }
}

impl IntoSvg for Board {
    type Options = ();

    fn into_svg(self: Self, _: Self::Options) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let board = Board::new();
        for y in 0..8 {
            for x in 0..8 {
                assert!(matches!(board.get_piece(x, y), Piece::Empty));
            }
        }
    }

    #[test]
    fn test_into_board() {
        let fen = Fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_owned());
        let board = fen.into_board().unwrap();

        // black top
        assert!(matches!(board.get_piece(0, 0), Piece::BRook));
        assert!(matches!(board.get_piece(0, 1), Piece::BKnight));
        assert!(matches!(board.get_piece(0, 2), Piece::BBishop));
        assert!(matches!(board.get_piece(0, 3), Piece::BQueen));
        assert!(matches!(board.get_piece(0, 4), Piece::BKing));
        assert!(matches!(board.get_piece(0, 5), Piece::BBishop));
        assert!(matches!(board.get_piece(0, 6), Piece::BKnight));
        assert!(matches!(board.get_piece(0, 7), Piece::BRook));

        // black pawns
        for x in 0..8 {
            assert!(matches!(board.get_piece(1, x), Piece::BPawn));
        }

        // empty rows
        for y in 2..6 {
            for x in 0..8 {
                assert!(matches!(board.get_piece(y, x), Piece::Empty));
            }
        }

        // white pawns
        for x in 0..8 {
            assert!(matches!(board.get_piece(6, x), Piece::WPawn));
        }

        // white bottom
        assert!(matches!(board.get_piece(7, 0), Piece::WRook));
        assert!(matches!(board.get_piece(7, 1), Piece::WKnight));
        assert!(matches!(board.get_piece(7, 2), Piece::WBishop));
        assert!(matches!(board.get_piece(7, 3), Piece::WQueen));
        assert!(matches!(board.get_piece(7, 4), Piece::WKing));
        assert!(matches!(board.get_piece(7, 5), Piece::WBishop));
        assert!(matches!(board.get_piece(7, 6), Piece::WKnight));
        assert!(matches!(board.get_piece(7, 7), Piece::WRook));
    }

    #[test]
    fn test_into_board_not_enough_rows() {
        let fen = Fen("8/8/8/8/8/8/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid FEN: must have 8 rows");
    }

    #[test]
    fn test_into_board_too_many_rows() {
        let fen = Fen("8/8/8/8/8/8/8/8/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid FEN: must have 8 rows");
    }

    #[test]
    fn test_into_board_row_too_many_columns() {
        let fen = Fen("8/8/8/8/8/8/ppppppppp/8".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid FEN: row: ppppppppp"));
    }

    #[test]
    fn test_into_board_invalid_piece_char() {
        // 'x' is not a valid FEN character
        let fen = Fen("8/8/8/8/8/8/8/rnbqkbnx".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .contains("Invalid FEN: piece character: x"));
    }

    #[test]
    fn test_into_board_digit_too_large_for_remaining_columns() {
        let fen = Fen("8/8/8/8/8/8/8/9".to_owned());
        let result = fen.into_board();

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid FEN: row: 9"));
    }
}
