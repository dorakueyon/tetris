use crate::Movement;
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
    fn areas(&self) -> Vec<Position> {
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
        let x = (width / 2) - 1;
        Position { y: 0, x }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }

    pub fn areas(&self) -> Vec<Position> {
        let mut row_areas = self.tetrimino.areas();
        let &Position { x, y } = self.position();
        row_areas = row_areas
            .iter()
            .map(|position| {
                return Position {
                    x: position.x + x,
                    y: position.y + y,
                };
            })
            .collect();

        row_areas
    }

    pub fn edge_positions(&self, position: Position) -> Vec<Position> {
        let Position { x, y } = position;
        let mut edge_positions = vec![];

        match self.tetrimino {
            Tetrmino::I => {
                let mut down_edges = vec![Position { x: x, y: y + 3 }];
                let mut side_edges = vec![
                    Position { x: x, y: y },
                    Position { x: x, y: y + 1 },
                    Position { x: x, y: y + 2 },
                    Position { x: x, y: y + 3 },
                ];
                edge_positions.append(&mut down_edges);
                edge_positions.append(&mut side_edges);
            }
        }

        edge_positions
    }

    pub fn peek_edge_positions(&self, movement: &Movement) -> Vec<Position> {
        let peek_position = self.peek_position(&movement);
        let edge_positions = self.edge_positions(peek_position);

        edge_positions
    }

    fn peek_position(&self, movement: &Movement) -> Position {
        let (diff_x, diff_y) = match movement {
            Movement::Down => (0, 1),
        };
        let Position { x, y } = self.position();

        Position {
            x: x + diff_x,
            y: y + diff_y,
        }
    }

    pub fn movement(&mut self, movement: &Movement) {
        match movement {
            Movement::Down => self.position.y = self.position.y + 1,
        }
    }
}
