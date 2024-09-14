use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

// 引入 crossterm 库中的事件处理相关的类型。

use super::terminal::Size;

// 从父模块中的 terminal 模块引入 Size 类型。

pub enum Direction {
    PageUp,
    PageDown,
    Home,
    End,
    Up,
    Down,
    Left,
    Right,
}

// 定义一个枚举 Direction，表示编辑器中的导航方向。

pub enum EditorCommand {
    Move(Direction),
    Resize(Size),
    Quit,
    Insert(char),
}

// 定义一个枚举 EditorCommand，表示编辑器可以执行的命令。
// 它可以是一个移动操作（Move），一个调整大小操作（Resize），或者退出（Quit）。

impl TryFrom<Event> for EditorCommand {
    type Error = String;

    // 为 EditorCommand 实现 TryFrom trait，允许从 Event 类型转换。
    // 如果转换失败，将返回一个 String 类型的错误。

    fn try_from(event: Event) -> Result<Self, Self::Error> {
        match event {
            Event::Key(KeyEvent {
                code, modifiers, ..
            }) => match (code, modifiers) {
                // 匹配键盘事件，根据按键代码和修饰键确定具体操作。
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => Ok(Self::Quit),
                // 如果是 Ctrl+Q，返回退出命令。
                (KeyCode::Char(character), KeyModifiers::NONE | KeyModifiers::SHIFT) => {
                    Ok(Self::Insert(character))
                }
                (KeyCode::Up, _) => Ok(Self::Move(Direction::Up)),
                // 如果是向上箭头键，返回向上移动命令。
                (KeyCode::Down, _) => Ok(Self::Move(Direction::Down)),
                // 如果是向下箭头键，返回向下移动命令。
                (KeyCode::Left, _) => Ok(Self::Move(Direction::Left)),
                // 如果是向左箭头键，返回向左移动命令。
                (KeyCode::Right, _) => Ok(Self::Move(Direction::Right)),
                // 如果是向右箭头键，返回向右移动命令。
                (KeyCode::PageDown, _) => Ok(Self::Move(Direction::PageDown)),
                // 如果是 PageDown 键，返回 PageDown 移动命令。
                (KeyCode::PageUp, _) => Ok(Self::Move(Direction::PageUp)),
                // 如果是 PageUp 键，返回 PageUp 移动命令。
                (KeyCode::Home, _) => Ok(Self::Move(Direction::Home)),
                // 如果是 Home 键，返回 Home 移动命令。
                (KeyCode::End, _) => Ok(Self::Move(Direction::End)),
                // 如果是 End 键，返回 End 移动命令。
                _ => Err(format!("Key Code not supported: {code:?}")),
                // 如果按键不被支持，返回错误。
            },
            Event::Resize(width_u16, height_u16) => Ok(Self::Resize(Size {
                height: height_u16 as usize,
                width: width_u16 as usize,
            })),
            _ => Err(format!("Event not supported: {event:?}")),
            // 如果事件不被支持，返回错误。
        }
    }
}
