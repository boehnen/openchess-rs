use super::piece::PieceState;

/// Board state structure
#[derive(Clone, Copy, Debug)]
pub struct ChessBoard {
    rows: [u32; 8], // 8 rows of 32 bits, each piece is allocated 4 bits to represent 13 possible states
                    // 88887777 66665555 44443333 22221111
}

impl ChessBoard {
    /// Initializes a new ChessBoard with empty pieces 
    pub fn new() -> Self {
        Self { rows: [0; 8] }
    }

    /// Initializes a new ChessBoard from FEN notation
    pub fn from_fen(fen: &str) -> Result<Self, String> {
        let mut board = ChessBoard::new();
        
        // Isolate the piece positions from the rest of FEN notation
        let board_part = fen.split_whitespace().next().unwrap_or(fen);
        
        // Split the piece positions by row
        let rows: Vec<&str> = board_part.split('/').collect();
        assert_eq!(rows.len(), 8, "Invalid FEN: must have 8 rows");

        // Process each char of each row
        for (row_index, row) in rows.iter().enumerate() {
            let mut col_index = 0;

            for c in row.chars() {
                if col_index == 8 {
                    // The row contains information for too many pieces (>8)
                    return Err(format!("Invalid FEN: row: {row}"))
                } else if c.is_ascii_digit() {
                    if let Some(digit) = c.to_digit(10) {
                        if digit < 1 || digit > (8 - col_index) {
                            // The digit in the row claimed 0 or too many empty pieces
                            return Err(format!("Invalid FEN: row: {row}"));
                        }
                        col_index += digit;
                    }
                    else{
                        return Err(format!("Character {c} failed to convert to digit"));
                    }
                } else {
                    let piece = Self::piece_from_char(c)?;
                    // Shift PieceState value left col columns and bitwise OR with current row state
                    board.rows[row_index] |= (piece as u32) << Self::col_shift(col_index);
                    col_index += 1;
                }
            }
        }

        Ok(board)
    }
    
    /// Returns an amount to shift a PieceState value to affect the desired column
    fn col_shift(col: u32) -> u32 {
        col * 4 // each piece is allocated 4 bits
    }

    /// Returns a mask that singles out the 
    fn col_mask(col: u32) -> u32 {
        0xF << Self::col_shift(col)
    }

    /// Maps FEN piece notation to PieceState Enum
    fn piece_from_char(c: char) -> Result<PieceState, String> {
        match c {
            'P' => Ok(PieceState::WPawn),
            'N' => Ok(PieceState::WKnight),
            'B' => Ok(PieceState::WBishop),
            'R' => Ok(PieceState::WRook),
            'Q' => Ok(PieceState::WQueen),
            'K' => Ok(PieceState::WKing),
            'p' => Ok(PieceState::BPawn),
            'n' => Ok(PieceState::BKnight),
            'b' => Ok(PieceState::BBishop),
            'r' => Ok(PieceState::BRook),
            'q' => Ok(PieceState::BQueen),
            'k' => Ok(PieceState::BKing),
            _   => Err(format!("Invalid FEN: piece character: {c}")),
        }
    }

    /// Returns the PieceState at the given position
    pub fn get_piece(&self, row: u32, col: u32) -> PieceState {
        // mask the column from the row then shift to lower 4 bits
        let nibble = (self.rows[row as usize] & Self::col_mask(col)) >> Self::col_shift(col); 
        match nibble {
            0  => PieceState::Empty,
            1  => PieceState::WPawn,
            2  => PieceState::WKnight,
            3  => PieceState::WBishop,
            4  => PieceState::WRook,
            5  => PieceState::WQueen,
            6  => PieceState::WKing,
            7  => PieceState::BPawn,
            8  => PieceState::BKnight,
            9  => PieceState::BBishop,
            10 => PieceState::BRook,
            11 => PieceState::BQueen,
            12 => PieceState::BKing,
            _  => unreachable!("Invalid nibble: {}", nibble),
        }
    }
}
