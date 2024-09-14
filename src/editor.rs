use crossterm::event::{read, Event, KeyEvent, KeyEventKind};
use std::{
    env,
    io::Error,
    panic::{set_hook, take_hook},
};
mod editorcommand;
mod terminal;
mod view;
use editorcommand::EditorCommand;
use terminal::Terminal;
use view::View;

// 引入必要的模块和类型，包括跨平台的终端处理库 crossterm 和标准库中的模块。

#[derive(Default)]
pub struct Editor {
    should_quit: bool, // 控制编辑器是否应该退出的布尔值。
    view: View,        // 编辑器的视图组件。
}

// 定义 Editor 结构体，并为其实现 Default trait，以便可以创建默认实例。

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook(); // 取出当前的 panic 钩子。
        set_hook(Box::new(move |panic_info| {
            // 设置一个新的 panic 钩子。
            let _ = Terminal::terminate(); // 尝试终止终端。
            current_hook(panic_info); // 调用之前的 panic 钩子。
        }));
        Terminal::initialize()?; // 初始化终端。
        let mut view = View::default(); // 创建默认的视图实例。
        let args: Vec<String> = env::args().collect(); // 获取命令行参数。
        if let Some(file_name) = args.get(1) {
            // 如果有第二个参数，视为文件名。
            view.load(file_name); // 加载文件。
        }
        Ok(Self {
            should_quit: false, // 初始时不退出。
            view,
        })
    }
    // 实现一个新的方法，用于创建编辑器实例。

    pub fn run(&mut self) {
        loop {
            // 进入主事件循环。
            self.refresh_screen(); // 刷新屏幕。
            if self.should_quit {
                // 如果应该退出，则打破循环。
                break;
            }
            match read() {
                // 读取事件。
                Ok(event) => self.evaluate_event(event), // 处理事件。
                Err(err) => {
                    // 如果读取事件出错，则 panic。
                    panic!("Could not read event: {err:?}");
                }
            }
        }
    }

    // 实现 run 方法，用于运行编辑器的主循环。

    fn evaluate_event(&mut self, event: Event) {
        let should_process = match &event {
            // 确定事件是否应该被处理。
            Event::Key(KeyEvent { kind, .. }) => kind == &KeyEventKind::Press, // 只处理按键按下事件。
            Event::Resize(_, _) => true, // 处理终端尺寸变化事件。
            _ => false,                  // 其他事件不处理。
        };
        if should_process {
            // 如果事件应该被处理。
            if let Ok(command) = EditorCommand::try_from(event) {
                if matches!(command, EditorCommand::Quit) {
                    // 如果命令是退出。
                    self.should_quit = true; // 设置 should_quit 为 true。
                } else {
                    // 否则，处理命令。
                    self.view.handle_command(command);
                }
            }
        }
    }

    // 实现 evaluate_event 方法，用于评估和处理事件。

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_caret(); // 隐藏光标。
        self.view.render(); // 渲染视图。
        let _ = Terminal::move_caret_to(self.view.caret_position()); // 移动光标到当前位置。
        let _ = Terminal::show_caret(); // 显示光标。
        let _ = Terminal::execute(); // 执行终端命令。
    }
    // 实现 refresh_screen 方法，用于刷新屏幕显示。
}

// 为 Editor 实现 Drop trait，确保在 Editor 实例被丢弃时执行清理操作。

impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate(); // 终止终端。
        if self.should_quit {
            // 如果是正常退出。
            let _ = Terminal::print("GoodBye\r\n"); // 打印告别信息。
        }
    }
}
