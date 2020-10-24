mod field;
mod matrix;
mod terminal;
mod tetrimino;
mod tetris;

pub use field::{Field, Movement, Position, Size};
pub use matrix::Matrix;
pub use terminal::Terminal;
pub use tetrimino::{CurrentTetrimino, Tetrmino};
pub use tetris::Tetris;

fn main() {
    Tetris::default().run();
}
