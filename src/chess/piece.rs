#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum PieceState {
    Empty = 0,      // 0000
    WPawn,          // 0001
    WKnight,        // 0010
    WBishop,        // 0011
    WRook,          // 0100
    WQueen,         // 0101
    WKing,          // 0110
    BPawn,          // 0111
    BKnight,        // 1000
    BBishop,        // 1001
    BRook,          // 1010
    BQueen,         // 1011
    BKing,          // 1100 
}
