use crate::PeekPosition;
use crate::Position;
use crate::Size;

use std::convert::From;

const ON_DOT: char = '■';
const CURRENT_TERMINO_DOT: char = '❏';
const OFF_DOT: char = ' ';

#[derive(Debug)]
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

    fn is_inside_field(position: &PeekPosition, size: &Size) -> bool {
        let PeekPosition { x, y } = position;
        let Size { height, width } = size;
        let height = i16::from(*height as u8);
        let width = i16::from(*width as u8);
        0 <= *x && *x <= width - 1 && 0 <= *y && *y <= height - 1
    }

    pub fn draw(&mut self, position: Position) {
        let Position { x, y } = position;
        self.bufs[y][x] = CURRENT_TERMINO_DOT;
    }

    pub fn is_buf_movable(&self, position: &PeekPosition, size: &Size) -> bool {
        !self.is_buf_on(position) && Matrix::is_inside_field(position, size)
    }

    fn is_buf_on(&self, position: &PeekPosition) -> bool {
        let PeekPosition { x, y } = position;
        if let Some(row) = self.bufs.get(*y as usize) {
            if let Some(buf) = row.get(*x as usize) {
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

    pub fn change_bufs_to_on(&mut self, areas: Vec<Position>) {
        for position in areas {
            let Position { x, y } = position;
            self.bufs[y][x] = ON_DOT;
        }
    }

    pub fn change_bufs_to_off(&mut self, areas: Vec<Position>) {
        for position in areas {
            let Position { x, y } = position;
            self.bufs[y][x] = OFF_DOT;
        }
    }

    pub fn change_bufs_to_current_tetrimino_dot(&mut self, areas: Vec<Position>) {
        for position in areas {
            let Position { x, y } = position;
            self.bufs[y][x] = CURRENT_TERMINO_DOT;
        }
    }
}
