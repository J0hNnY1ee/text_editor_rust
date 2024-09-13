use std::{fs::read_to_string, io::Error}; // 引入标准库中的 read_to_string 函数和 Error 类型。

use super::line::Line; // 从当前模块的父模块中引入 Line 结构体。

#[derive(Default)] // 标记 Buffer 结构体可以使用 Default trait 来生成默认实例。
pub struct Buffer {
    pub lines: Vec<Line>, // Buffer 包含一个 Line 类型的向量，表示文本的每一行。
}

impl Buffer {
    // 为 Buffer 实现方法。
    pub fn load(file_name: &str) -> Result<Self, Error> {
        // 定义一个加载文件内容并创建 Buffer 实例的方法。
        let contents = read_to_string(file_name)?; // 读取文件内容，如果出错则返回错误。
        let mut lines = Vec::new(); // 创建一个用于存储 Line 实例的向量。
        for value in contents.lines() { // 遍历文件的每一行。
            lines.push(Line::from(value)) // 将每行文本转换为 Line 实例并添加到向量中。
        }
        Ok(Self { lines }) // 如果成功，返回包含所有 Line 实例的 Buffer 实例。
    }

    pub fn is_empty(&self) -> bool {
        // 定义一个方法，用于检查 Buffer 是否为空。
        self.lines.is_empty() // 如果 lines 向量为空，则返回 true。
    }

    pub fn height(&self) -> usize {
        // 定义一个方法，用于获取 Buffer 的“高度”，即行数。
        self.lines.len() // 返回 lines 向量的长度，即行数。
    }
}