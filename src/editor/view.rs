mod buffer;

use std::io::Error;

use buffer::Buffer;

use super::terminal::{Size, Terminal};
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Default)]
pub struct View {
    buffer: Buffer,
}

impl View {
    pub fn render(&self) -> Result<(), Error> {
        let Size { height, .. } = Terminal::size()?;
        for current_row in 0..height {
            if let Some(line)= self.buffer.lines.get(current_row)
            {
                Terminal::print(line)?;
                Terminal::print("\r\n")?;
                continue;
            }
            let welcome_line = height as f64 * 0.382;
            if current_row == welcome_line as usize {
                Self::draw_welcome_message()?;
            } else {
                Self::draw_empty_rows()?;
            }
            if current_row.saturating_add(1) < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn draw_welcome_message() -> Result<(), Error> {
        let mut welcome_message = format!("{NAME} editor -- version {VERSION}");
        let width = Terminal::size()?.width;
        let len = welcome_message.len();
        let padding = (width - len) / 2;
        let spaces = " ".repeat(padding - 1);
        welcome_message = format!("~{spaces}{welcome_message}");
        welcome_message.truncate(width);
        Terminal::print(&welcome_message)?;
        Ok(())
    }

    fn draw_empty_rows() -> Result<(), Error> {
        Terminal::print("~")?;
        Ok(())
    }
}
