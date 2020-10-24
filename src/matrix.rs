use crate::Position;
use crate::Size;

const VERTICAL_LINE: char = '|';
const HORIZONTAL_LINE: char = '-';
const ON_DOT: char = '■';
const CURRENT_TERMINO_DOT: char = '❏';
const OFF_DOT: char = ' ';

pub struct Matrix {
    bufs: Vec<Vec<char>>,
}

impl Matrix {
    pub fn initialize(height: usize, width: usize) -> Self {
        let mut bufs = vec![];
        for _y in 0..height {
            let mut row = vec![];
            for _x in 0..width {
                row.push(OFF_DOT)
            }
            bufs.push(row.clone());
        }
        Self { bufs }
    }

    fn is_inside_field(position: &Position, size: &Size) -> bool {
        let Position { x, y } = position;
        let Size { height, width } = size;
        0 <= *x && *x <= *width - 1 && 0 <= *y && *y <= *height - 1
    }

    pub fn draw(&mut self, position: Position) {
        let Position { x, y } = position;
        self.bufs[y][x] = CURRENT_TERMINO_DOT;
    }

    pub fn is_buf_movable(&self, position: &Position, size: &Size) -> bool {
        !self.is_buf_on(position) && Matrix::is_inside_field(position, size)
    }

    fn is_buf_on(&self, position: &Position) -> bool {
        let Position { x, y } = position;
        if let Some(row) = self.bufs.get(*y) {
            if let Some(buf) = row.get(*x) {
                return *buf == ON_DOT;
            }
        };

        false
    }

    pub fn render(&self, index: usize) -> Option<String> {
        match self.bufs.get(index) {
            Some(chars) => Some(chars.iter().collect()),
            None => None,
        }
    }
}
