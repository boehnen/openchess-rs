pub trait IntoBoard {
    type Board;

    fn into_board(self: Self) -> Result<Self::Board, String>;
}

pub trait IntoSvg {
    fn into_svg(self: Self) -> String;
}

pub mod chess;
