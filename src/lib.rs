pub trait IntoBoard {
    type Board;

    fn into_board(self: Self) -> Result<Self::Board, anyhow::Error>;
}

pub trait IntoSvg {
    type Options;

    fn into_svg(self: Self, options: Self::Options) -> String;
}

pub mod chess;
