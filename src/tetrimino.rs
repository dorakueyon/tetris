use crate::Position;

#[derive(Debug, PartialEq)]
pub enum Tetrmino {
    I, //O
       //S
       //Z
       //L
       //T
}

impl Tetrmino {
    pub fn area(&self) -> Vec<Position> {
        match &self {
            Self::I => {
                return vec![
                    //■
                    //■
                    //■
                    //■
                    Position { x: 0, y: 0 },
                    Position { x: 0, y: 1 },
                    Position { x: 0, y: 2 },
                    Position { x: 0, y: 3 },
                ];
            }
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct CurrentTetrimino {
    pub tetrimino: Tetrmino,
    position: Position,
}

impl CurrentTetrimino {
    pub fn default(tetrimino: Tetrmino, width: usize) -> Self {
        Self {
            tetrimino,
            position: Self::default_position(width),
        }
    }

    fn default_position(width: usize) -> Position {
        let x = width / 2;
        Position { y: 0, x }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}
