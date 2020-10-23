const VERTICAL_LINE: char = '|';
const HORIZONTAL_LINE: char = '-';
const ON_DOT: char = '■';
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

    pub fn draw(&mut self, x: usize, y: usize) {
        self.bufs[y][x] = ON_DOT;
    }

    pub fn render(&self, index: usize) -> Option<String> {
        match self.bufs.get(index) {
            Some(chars) => Some(chars.iter().collect()),
            None => None,
        }
    }
}
