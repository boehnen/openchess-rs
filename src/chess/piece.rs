#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum PieceState {
    Empty = 0,
    WPawn,
    WKnight,
    WBishop,
    WRook,
    WQueen,
    WKing,
    BPawn,
    BKnight,
    BBishop,
    BRook,
    BQueen,
    BKing,
}

// BKing = 1100
// 0000 0000  0000 0000  0000 0000  0000 0000
// 0000 1100  0000 ...

// row || Bking << (4 * col)
