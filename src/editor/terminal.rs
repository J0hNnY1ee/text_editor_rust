use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, size, Clear, ClearType, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{queue, Command};
use std::io::{stdout, Error, Write};

// 引入 crossterm 库中的光标控制、样式、终端控制和通用命令相关的功能。

#[derive(Default, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

// 定义一个 Size 结构体，用于存储终端的尺寸。它实现了 Default、Copy 和 Clone trait。

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub col: usize,
    pub row: usize,
}

// 定义一个 Position 结构体，用于表示光标的位置。它实现了 Copy、Clone 和 Default trait。

impl Position {
    pub const fn saturating_sub(self, other: Self) -> Self {
        Self {
            row: self.row.saturating_sub(other.row),
            col: self.col.saturating_sub(other.col),
        }
    }
}

// 为 Position 实现一个方法，用于饱和减法，即减去另一个 Position 的行和列，结果不会小于 0。
// 这个方法是 const fn，可以在编译时执行。

pub struct Terminal;

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::leave_alternate_screen()?; // 离开备用屏幕。
        Self::show_caret()?; // 显示光标。
        Self::execute()?; // 执行队列中的命令。
        disable_raw_mode()?; // 禁用原始模式。
        Ok(())
    }

    // 定义一个方法，用于终止 Terminal 操作，恢复终端到正常状态。

    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?; // 启用原始模式。
        Self::enter_alternate_screen()?; // 进入备用屏幕。
        Self::clear_screen()?; // 清屏。
        Self::execute()?; // 执行队列中的命令。
        Ok(())
    }

    // 定义一个方法，用于初始化 Terminal 操作，设置终端到一个适合程序运行的状态。

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?; // 清屏。
        Ok(())
    }

    // 定义一个方法，用于清空整个屏幕。

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?; // 清除当前行。
        Ok(())
    }

    // 定义一个方法，用于清除当前行的内容。

    pub fn move_caret_to(position: Position) -> Result<(), Error> {
        // 定义一个方法，用于将光标移动到指定位置。
        #[allow(clippy::as_conversions, clippy::cast_possible_truncation)]
        Self::queue_command(MoveTo(position.col as u16, position.row as u16))?; // 将光标移动到 (col, row)。
        Ok(())
    }

    // 将光标移动到指定的列和行。

    pub fn enter_alternate_screen() -> Result<(), Error> {
        Self::queue_command(EnterAlternateScreen)?; // 进入备用屏幕。
        Ok(())
    }

    // 定义一个方法，用于进入备用屏幕，通常用于创建一个独立的屏幕环境。

    pub fn leave_alternate_screen() -> Result<(), Error> {
        Self::queue_command(LeaveAlternateScreen)?; // 离开备用屏幕。
        Ok(())
    }

    // 定义一个方法，用于离开备用屏幕，返回到主屏幕。

    pub fn hide_caret() -> Result<(), Error> {
        Self::queue_command(Hide)?; // 隐藏光标。
        Ok(())
    }

    // 定义一个方法，用于隐藏光标。

    pub fn show_caret() -> Result<(), Error> {
        Self::queue_command(Show)?; // 显示光标。
        Ok(())
    }

    // 定义一个方法，用于显示光标。

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?; // 打印字符串。
        Ok(())
    }

    // 定义一个方法，用于在当前光标位置打印字符串。

    pub fn print_row(row: usize, line_text: &str) -> Result<(), Error> {
        Self::move_caret_to(Position { row, col: 0 })?; // 移动光标到指定行的开始。
        Self::clear_line()?; // 清除当前行。
        Self::print(line_text)?; // 打印文本。
        Ok(())
    }

    // 定义一个方法，用于在指定行打印文本，先清空该行。

    pub fn size() -> Result<Size, Error> {
        let (width_u16, height_u16) = size()?; // 获取屏幕的宽度和高度。

        #[allow(clippy::as_conversions)]
        let height = height_u16 as usize; // 将高度从 u16 转换为 usize。

        #[allow(clippy::as_conversions)]
        let width = width_u16 as usize; // 将宽度从 u16 转换为 usize。
        Ok(Size { height, width }) // 返回尺寸。
    }

    // 定义一个方法，用于获取终端的尺寸。

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?; // 刷新 stdout，确保所有命令都被执行。
        Ok(())
    }

    // 定义一个方法，用于执行队列中的命令。

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?; // 将命令添加到队列中。
        Ok(())
    }

    // 定义一个私有方法，用于将命令添加到执行队列中。
}