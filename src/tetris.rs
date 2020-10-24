use crate::Field;
use crate::Movement;
use crate::Terminal;
use crate::Tetrmino;

use termion::event::Key;
use termion::input::TermRead;

const VERTICAL_LINE: char = '|';
const HORIZONTAL_LINE: char = '-';

pub struct Tetris {
    should_break: bool,
    terminal: Terminal,
    field: Field,
    needed_tetrimino: bool,
    is_game_over: bool,
}

impl Tetris {
    pub fn default() -> Self {
        Tetris {
            should_break: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            field: Field::default(),
            needed_tetrimino: true,
            is_game_over: false,
        }
    }

    fn die(e: std::io::Error) {
        println!("{}", termion::clear::All);
        panic!(e)
    }

    fn prosess_keypress(&mut self) -> Result<(), std::io::Error> {
        let key = self.read_key()?;
        dbg!(key);
        match key {
            Key::Char('q') => self.should_break = true,
            Key::Down => self.field.current_tetrimino_move(Movement::Down),
            Key::Left => self.field.current_tetrimino_move(Movement::Left),
            Key::Right => self.field.current_tetrimino_move(Movement::Right),
            _ => {}
        }
        Ok(())
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    fn draw_rows(&self) {
        print!("\r\n");
        print!("\r\n");
        for i in 0..self.field.size().height {
            let field_row = self.field.matrix.render(i).unwrap();
            let row = format!("{}{}{}", VERTICAL_LINE, field_row, VERTICAL_LINE);

            print!("{}", row.trim_end());
            print!("\r\n");
        }
        let mut bottom_row = String::new();
        for _ in 0..self.field.size().width + 2 {
            bottom_row.push(HORIZONTAL_LINE)
        }
        print!("{}", bottom_row);
        print!("\r\n");
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        self.draw_rows();
        Terminal::flush()
    }

    fn is_current_tetrimino_movement_completed(&self) -> bool {
        false
    }

    fn current_tetrimino_movement_complete(&mut self) {}

    pub fn run(&mut self) {
        loop {
            dbg!(&self.field.current_tetrimino);
            if self.needed_tetrimino {
                self.field.insert_tetrimino(Tetrmino::I);
                self.needed_tetrimino = false;
            }

            if let Err(err) = self.refresh_screen() {
                Tetris::die(err)
            }

            if self.should_break {
                break;
            }

            if let Err(err) = self.prosess_keypress() {
                Tetris::die(err);
            }

            if self.is_current_tetrimino_movement_completed() {
                self.current_tetrimino_movement_complete();
                if self.is_game_over {
                    break;
                }
                self.needed_tetrimino = true;
            }
        }
    }
}
