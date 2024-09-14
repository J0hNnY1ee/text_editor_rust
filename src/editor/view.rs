use std::cmp::min;

use super::{
    editorcommand::{Direction, EditorCommand},
    terminal::{Position, Size, Terminal},
};
mod buffer;
mod line;
use buffer::Buffer;
use line::Line;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// 引入必要的模块和类型，使用宏 env! 来获取当前 Cargo 包的名称和版本。

#[derive(Clone, Copy, Default)]
pub struct Location {
    pub grapheme_index: usize, // 字形索引，用于在行内定位字符。
    pub line_index: usize,     // 行索引，用于在缓冲区中定位行。
}

// 定义 Location 结构体，用于跟踪文本位置。

pub struct View {
    buffer: Buffer,          // 缓冲区，存储文本数据。
    needs_redraw: bool,      // 是否需要重绘视图。
    size: Size,              // 视图的尺寸。
    text_location: Location, // 文本的位置。
    scroll_offset: Position, // 滚动偏移量。
}

// 定义 View 结构体，表示文本编辑器的视图。

impl View {
    pub fn handle_command(&mut self, command: EditorCommand) {
        // 处理编辑器命令。
        match command {
            EditorCommand::Resize(size) => self.resize(size),
            EditorCommand::Move(direction) => self.move_text_location(&direction),
            EditorCommand::Quit => {}
            EditorCommand::Insert(character) => self.insert_char(character),
            EditorCommand::BackSpace => self.delete_backward(),
            EditorCommand::Delete => self.delete(),
            EditorCommand::Enter => self.insert_newline(),
        }
    }

    // 根据命令调整视图状态。

    pub fn load(&mut self, file_name: &str) {
        // 加载文件到缓冲区。
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) {
        // 渲染视图。
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        let vertical_center = height / 3;
        let top = self.scroll_offset.row;
        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row.saturating_add(top)) {
                let left = self.scroll_offset.col;
                let right = self.scroll_offset.col.saturating_add(width);
                Self::render_line(current_row, &line.get_visible_graphemes(left..right));
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(current_row, "~");
            }
            self.needs_redraw = false;
        }
    }

    fn resize(&mut self, to: Size) {
        // 调整视图尺寸。
        self.size = to;
        self.scroll_text_location_into_view();
        self.needs_redraw = true;
    }

    fn scroll_vertically(&mut self, to: usize) {
        // 垂直滚动视图。
        let Size { height, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.row {
            self.scroll_offset.row = to;
            true
        } else if to >= self.scroll_offset.row.saturating_add(height) {
            self.scroll_offset.row = to.saturating_sub(height).saturating_add(1);
            true
        } else {
            false
        };
        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_horizontally(&mut self, to: usize) {
        // 水平滚动视图。
        let Size { width, .. } = self.size;
        let offset_changed = if to < self.scroll_offset.col {
            self.scroll_offset.col = to;
            true
        } else if to >= self.scroll_offset.col.saturating_add(width) {
            self.scroll_offset.col = to.saturating_sub(width).saturating_add(1);
            true
        } else {
            false
        };
        self.needs_redraw = self.needs_redraw || offset_changed;
    }

    fn scroll_text_location_into_view(&mut self) {
        // 确保文本位置在视图内。
        let Position { row, col } = self.text_location_to_position();
        self.scroll_vertically(row);
        self.scroll_horizontally(col);
    }

    pub fn caret_position(&self) -> Position {
        // 获取光标位置。
        self.text_location_to_position()
            .saturating_sub(self.scroll_offset)
    }

    fn text_location_to_position(&self) -> Position {
        // 将文本位置转换为屏幕位置。
        let row = self.text_location.line_index;
        let col = self.buffer.lines.get(row).map_or(0, |line| {
            line.width_until(self.text_location.grapheme_index)
        });
        Position { col, row }
    }

    fn move_text_location(&mut self, direction: &Direction) {
        // 根据方向移动文本位置。
        let Size { height, .. } = self.size;

        match direction {
            Direction::Up => self.move_up(1),
            Direction::Down => self.move_down(1),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
            Direction::PageUp => self.move_up(height.saturating_sub(1)),
            Direction::PageDown => self.move_down(height.saturating_sub(1)),
            Direction::Home => self.move_to_start_of_line(),
            Direction::End => self.move_to_end_of_line(),
        }
        self.scroll_text_location_into_view();
    }

    fn move_up(&mut self, step: usize) {
        // 向上移动文本位置。
        self.text_location.line_index = self.text_location.line_index.saturating_sub(step);
        self.snap_to_valid_grapheme();
    }

    fn move_down(&mut self, step: usize) {
        // 向下移动文本位置。
        self.text_location.line_index = self.text_location.line_index.saturating_add(step);
        self.snap_to_valid_grapheme();
        self.snap_to_valid_line();
    }

    fn move_right(&mut self) {
        // 向右移动文本位置。
        let line_width = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        if self.text_location.grapheme_index < line_width {
            self.text_location.grapheme_index += 1;
        } else {
            self.move_to_start_of_line();
            self.move_down(1);
        }
    }

    fn move_left(&mut self) {
        // 向左移动文本位置。
        if self.text_location.grapheme_index > 0 {
            self.text_location.grapheme_index -= 1;
        } else if self.text_location.line_index > 0 {
            self.move_up(1);
            self.move_to_end_of_line();
        }
    }

    fn move_to_start_of_line(&mut self) {
        // 移动到行首。
        self.text_location.grapheme_index = 0;
    }

    fn move_to_end_of_line(&mut self) {
        // 移动到行尾。
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
    }

    fn snap_to_valid_grapheme(&mut self) {
        // 确保字形索引有效。
        self.text_location.grapheme_index = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, |line| {
                min(line.grapheme_count(), self.text_location.grapheme_index)
            });
    }

    fn snap_to_valid_line(&mut self) {
        // 确保行索引有效。
        self.text_location.line_index = min(self.text_location.line_index, self.buffer.height());
    }

    fn build_welcome_message(width: usize) -> String {
        // 构建欢迎消息。
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }

        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}", " ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }

    fn render_line(at: usize, line_text: &str) {
        // 渲染一行文本。
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }
    fn insert_char(&mut self, character: char) {
        let old_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        self.buffer.insert_char(character, self.text_location);

        let new_len = self
            .buffer
            .lines
            .get(self.text_location.line_index)
            .map_or(0, Line::grapheme_count);
        let grapheme_delta = new_len.saturating_sub(old_len);
        if grapheme_delta > 0 {
            self.move_right();
        }
        self.needs_redraw = true;
    }
    fn delete_backward(&mut self) {
        if self.text_location.line_index != 0 || self.text_location.grapheme_index != 0
        {
            self.move_text_location(&Direction::Left);
            self.delete();
        }

    }
    fn insert_newline(&mut self)
    {
        self.buffer.insert_newline(self.text_location);
        self.move_text_location(&Direction::Right);
        self.needs_redraw = true;
    }
    fn delete(&mut self) {
        self.buffer.delete(self.text_location);
        self.needs_redraw = true;
    }
}

// 实现 View 的默认构造函数。
impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(), // 使用 Buffer 的默认值初始化 buffer。
            needs_redraw: true,        // 默认需要重绘。
            size: Terminal::size().unwrap_or_default(), // 获取终端尺寸，如果失败则使用默认值。
            text_location: Location::default(), // 使用 Location 的默认值初始化 text_location。
            scroll_offset: Position::default(), // 使用 Position 的默认值初始化 scroll_offset。
        }
    }
}
