pub trait IntoBoard {
    type Piece;

    fn into_board(self) -> Result<Board<Self::Piece>, anyhow::Error>;
}

pub trait IntoSvg {
    type Options;

    fn into_svg(self, options: Self::Options) -> String;
}

#[derive(Debug)]
pub struct Board<T> {
    pieces: [[T; 8]; 8],
}

impl<T> Board<T>
where
    T: Copy + Default,
{
    pub(crate) fn new() -> Self {
        Self {
            pieces: [[T::default(); 8]; 8],
        }
    }

    #[cfg(test)]
    pub(crate) fn get_piece(&self, row: usize, col: usize) -> T {
        self.pieces[row][col]
    }

    pub(crate) fn set_piece(&mut self, row: usize, col: usize, piece: T) {
        self.pieces[row][col] = piece;
    }
}

impl<T> IntoSvg for Board<T> {
    type Options = ();

    fn into_svg(self, _: Self::Options) -> String {
        todo!()
    }
}

pub mod chess;
