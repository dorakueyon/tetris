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
#[derive(Debug, PartialEq)]
pub struct PeekPosition {
    pub x: i16,
    pub y: i16,
}

pub enum Movement {
    Down,
    Left,
    Right,
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

#[derive(Debug)]
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

    pub fn size(&self) -> &Size {
        &self.size
    }

    fn refresh_matrix(&mut self) {
        if let Some(current_tetrimino) = &self.current_tetrimino {
            let tetrimino_areas = current_tetrimino.areas();
            for position in tetrimino_areas {
                self.matrix.draw(position)
            }
        }
    }

    pub fn insert_tetrimino(&mut self, tetrimino: Tetrmino) {
        self.current_tetrimino = Some(CurrentTetrimino::default(tetrimino, self.size.width));
        self.refresh_matrix()
    }

    pub fn current_tetrimino_move(&mut self, movement: Movement) {
        if let Some(current_tetrimino) = self.current_tetrimino.as_mut() {
            dbg!(&current_tetrimino);
            match &movement {
                Movement::Down | Movement::Left | Movement::Right => {
                    let mut movable = true;
                    let peek_edge_positions = current_tetrimino.peek_edge_positions(&movement);
                    for position in peek_edge_positions {
                        if !self.matrix.is_buf_movable(&position, &self.size) {
                            movable = false
                        }
                    }
                    if movable {
                        let areas = current_tetrimino.areas();
                        self.matrix.change_bufs_to_off(areas);

                        current_tetrimino.movement(&movement);

                        let areas = current_tetrimino.areas();
                        self.matrix.change_bufs_to_current_tetrimino_dot(areas);
                    }
                    return;
                }
            }
        };
    }

    fn is_current_tetrimino_movement_completable(&self) -> bool {
        true
    }

    fn complete_current_tetrimino_movement(&mut self) -> Result<(), String> {
        if !self.is_current_tetrimino_movement_completable() {
            return Err(String::from(
                "current tetrimino movement cannot be completed",
            ));
        };
        match &self.current_tetrimino {
            None => return Err(String::from("current tetrimino does not existed")),
            Some(current_tetrimiono) => {
                let areas = current_tetrimiono.areas();
                self.matrix.change_bufs_to_on(areas);
            }
        };
        self.current_tetrimino = None;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert_I() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        let current_tetrimino = match &field.current_tetrimino {
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
        test_matrix_render(expected, &field);
    }

    #[test]
    fn test_tetrimino_move_down() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        field.current_tetrimino_move(Movement::Down);

        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position { x: 4, y: 1 }
        );

        let expected = vec![
            "          ",
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
        ];
        test_matrix_render(expected, &field);
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

    #[test]
    fn test_tetrimino_move_left_and_right() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        field.current_tetrimino_move(Movement::Left);

        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position { x: 3, y: 0 }
        );

        let tetrimino_width = 1;
        let mut limit = field.size.width - tetrimino_width - 1;
        for _ in 0..limit {
            field.current_tetrimino_move(Movement::Left);
        }
        field.current_tetrimino_move(Movement::Left);
        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position { x: 0, y: 0 }
        );

        limit = field.size.width;
        for _ in 0..limit {
            field.current_tetrimino_move(Movement::Right);
        }
        field.current_tetrimino_move(Movement::Right);
        assert_eq!(
            field.current_tetrimino.as_ref().unwrap().position(),
            &Position { x: limit - 1, y: 0 }
        );
    }

    #[test]
    fn test_tetrimino_complete_movement() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        field.current_tetrimino_move(Movement::Down);

        let tetrimino_height = 4;
        let limit = field.size.height - tetrimino_height - 1;
        for _ in 0..limit {
            println!("===========");
            field.current_tetrimino_move(Movement::Down);
        }
        field.complete_current_tetrimino_movement().unwrap();

        assert!(field.current_tetrimino.is_none());

        let expected = vec![
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
            "    ■     ",
            "    ■     ",
            "    ■     ",
            "    ■     ",
        ];
        test_matrix_render(expected, &field)
    }

    fn test_matrix_render(expected: Vec<&str>, field: &Field) {
        assert_eq!(expected.len(), field.size.height);
        for i in 0..field.size.height {
            if let Some(row) = field.matrix.render(i as usize) {
                assert_eq!(
                    format!("{}:{}", i, row),
                    format!("{}:{}", i, expected[i as usize])
                )
            }
        }
    }
}
