use crate::Field;
use crate::Terminal;

use termion::event::Key;
use termion::input::TermRead;

pub struct Tetris {
    should_break: bool,
    terminal: Terminal,
    field: Field,
}

impl Tetris {
    pub fn default() -> Self {
        Tetris {
            should_break: false,
            terminal: Terminal::default().expect("Failed to initialize terminal"),
            field: Field::default(),
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
        
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        self.draw_rows();
        Terminal::flush()
    }

    pub fn run(&mut self) {
        loop {
            if let Err(err) = self.refresh_screen() {
                Tetris::die(err)
            }

            if self.should_break {
                break;
            }

            if let Err(err) = self.prosess_keypress() {
                Tetris::die(err);
            }
        }
    }
}
