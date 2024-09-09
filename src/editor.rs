mod terminal;
use std::io::{stdout, Error};

use crossterm::{
    event::{
        read,
        Event::{self, Key},
        KeyCode::Char,
        KeyEvent, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();

        result.unwrap()
    }

    fn repl(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye.\r\n");
        } else {
            {
                Self::draw_rows()?;
                Terminal::move_cursor_to(0, 0)?;
            }
        }
        Ok(())
    }
    fn draw_rows() -> Result<(), Error> {
        let height = Terminal::size()?.1;
        for current_row in 0..height {
            print!("~");
            if current_row + 1 < height {
                println!("\r")
            }
        }
        Ok(())
    }
}
