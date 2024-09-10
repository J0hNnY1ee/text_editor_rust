use core::fmt::Display;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear},
    Command,
};
use std::io::{stdout, Error, Write};

pub struct Terminal {}
#[derive(Clone, Copy)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}
#[derive(Copy, Clone,Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}
impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(crossterm::terminal::ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(crossterm::terminal::ClearType::All))?;
        Ok(())
    }

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?;
        Ok(())
    }
    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }
    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }
    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?;
        let height = height_u16 as usize;
        let width = width_u16 as usize;
        Ok(Size { height, width })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }
    fn queue_command<T: Command>(commond: T) -> Result<(), Error> {
        queue!(stdout(), commond)?;
        Ok(())
    }
}
