use std::error::Error;
use std::io::stdout;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

pub struct Tetris {}

impl Tetris {
    pub fn default() -> Self {
        Tetris {}
    }

    fn die(e: std::io::Error) {
        println!("{}", termion::clear::All);
        panic!(e)
    }

    fn prosess_keypress(&self) -> Result<(), std::io::Error> {
        let key = self.read_key()?;
        dbg!(key);
        Ok(())
    }

    fn read_key(&self) -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = std::io::stdin().lock().keys().next() {
                return key;
            }
        }
    }
    pub fn run(&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        if let Err(err) = self.edi
        if let Err(err) = self.prosess_keypress() {
            Tetris::die(err);
        }
    }
}
