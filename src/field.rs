use crate::CurrentTetrimino;
use crate::Matrix;
use crate::Tetrmino;

const HEIGHT: usize = 20;
const WIDTH: usize = 10;

#[derive(Debug, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub enum Movement {
    Down,
}

#[derive(Debug, PartialEq)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub fn new(height: usize, width: usize) -> Self {
        Self { height, width }
    }
}

pub struct Field {
    size: Size,
    pub matrix: Matrix,
    pub current_tetrimino: Option<CurrentTetrimino>,
}

impl Field {
    pub fn default() -> Self {
        Self {
            size: Size::new(HEIGHT, WIDTH),
            matrix: Matrix::initialize(HEIGHT, WIDTH),
            current_tetrimino: None,
        }
    }

    fn refresh_matrix(&mut self) {
        if let Some(current_tetrimino) = &self.current_tetrimino {
            let tetrimino_areas = current_tetrimino.areas();
            for position in tetrimino_areas {
                self.matrix.draw(position)
            }
        }
    }

    fn insert_tetrimino(&mut self, tetrimino: Tetrmino) {
        self.current_tetrimino = Some(CurrentTetrimino::default(tetrimino, self.size.width));
        self.refresh_matrix()
    }

    fn current_tetrimino_move(&mut self, movement: Movement) {
        //if let Some(ref mut current_tetrimino) = self.current_tetrimino {
        if let Some(current_tetrimino) = self.current_tetrimino.as_mut() {
            match &movement {
                Movement::Down => {
                    let mut movable = true;
                    let peek_edge_positions =
                        current_tetrimino.peek_edge_positions(&Movement::Down);
                    for position in peek_edge_positions {
                        if !self.matrix.is_buf_movable(&position, &self.size) {
                            movable = false
                        }
                    }
                    if movable {
                        current_tetrimino.movement(&movement);
                    }
                    return;
                }
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert_I() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        let mut current_tetrimino = match &field.current_tetrimino {
            None => panic!(),
            Some(c) => c,
        };

        assert_eq!(current_tetrimino.tetrimino, Tetrmino::I);
        assert_eq!(current_tetrimino.position(), &Position { y: 0, x: 4 });

        let expected = vec![
            "    ❏     ",
            "    ❏     ",
            "    ❏     ",
            "    ❏     ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
            "          ",
        ];
        {
            for i in 0..field.size.height {
                if let Some(row) = field.matrix.render(i as usize) {
                    assert_eq!(row, expected[i as usize])
                }
            }
        }
    }
    #[test]
    fn test_tetrimino_move() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        field.current_tetrimino_move(Movement::Down);

        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position { x: 4, y: 1 }
        );

        let tetrimino_height = 4;
        let limit = field.size.height - tetrimino_height - 1;
        for _ in 0..limit {
            field.current_tetrimino_move(Movement::Down);
        }
        field.current_tetrimino_move(Movement::Down);
        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position {
                x: 4,
                y: field.size.height - tetrimino_height
            }
        );
    }
}
