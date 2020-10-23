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

pub struct Field {
    height: usize,
    width: usize,
    matrix: Matrix,
    current_tetrimino: Option<CurrentTetrimino>,
}

impl Field {
    pub fn default() -> Self {
        Self {
            height: HEIGHT,
            width: WIDTH,
            matrix: Matrix::initialize(HEIGHT, WIDTH),
            current_tetrimino: None,
        }
    }

    fn refresh_matrix(&mut self) {
        if let Some(current_tetrimino) = &self.current_tetrimino {
            let tetrimino_areas = current_tetrimino.tetrimino.area();
            let Position { x, y } = current_tetrimino.position();
            for position in tetrimino_areas {
                self.matrix.draw(position.x + x, position.y + y)
            }
        }
    }

    fn insert_tetrimino(&mut self, tetrimino: Tetrmino) {
        self.current_tetrimino = Some(CurrentTetrimino::default(tetrimino, self.width));
        self.refresh_matrix()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_insert_I() {
        let mut field = Field::default();
        field.insert_tetrimino(Tetrmino::I);
        match &field.current_tetrimino {
            None => panic!(),
            Some(current_tetrimino) => {
                assert_eq!(current_tetrimino.tetrimino, Tetrmino::I);
                assert_eq!(current_tetrimino.position(), &Position { y: 0, x: 5 });
            }
        }

        let expected = vec![
            "     ■    ",
            "     ■    ",
            "     ■    ",
            "     ■    ",
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
        for i in 0..field.height {
            if let Some(row) = field.matrix.render(i as usize) {
                assert_eq!(row, expected[i as usize])
            }
        }
    }
}
