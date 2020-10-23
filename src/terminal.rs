use std::io::{self, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub struct Terminal {
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        Ok(Terminal {
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All)
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn cursor_position(x: usize, y: usize) {
        print!("{}", termion::cursor::Goto(x as u16 + 1, y as u16 + 1));
    }
}
