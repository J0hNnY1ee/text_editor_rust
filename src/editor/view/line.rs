use std::{char, ops::Range};

// 引入 Rust 标准库中的 char, cmp, Range 和 result 模块。

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

// 引入第三方库，用于处理 Unicode 文本的分割和宽度计算。

#[derive(Clone, Copy)]
// 定义一个可以克隆和复制的枚举类型 GraphemeWidth。
enum GraphemeWidth {
    Half, // 半宽字符
    Full, // 全宽字符
}

impl GraphemeWidth {
    // 实现 GraphemeWidth 的方法，用于饱和加法。
    const fn saturating_add(self, other: usize) -> usize {
        match self {
            Self::Full => other.saturating_add(1), // 全宽字符加1
            Self::Half => other.saturating_add(2), // 半宽字符加2
        }
    }
}

// 定义一个结构体 TextFragment，用于存储文本片段的信息。
struct TextFragment {
    grapheme: String,              // 文本片段
    rendered_width: GraphemeWidth, // 渲染宽度
    replacement: Option<char>,     // 替换字符
}

// 定义一个结构体 Line，用于存储一行文本的片段。
pub struct Line {
    fragments: Vec<TextFragment>, // 文本片段的向量
}

impl Line {
    // 实现 Line 的方法，用于从字符串创建一个 Line 实例。
    pub fn from(line_str: &str) -> Self {
        let fragments = Self::str_to_fragments(line_str);
        Self { fragments }
    }
    fn str_to_fragments(line_str: &str) -> Vec<TextFragment> {
        line_str
            .graphemes(true) // 使用 Unicode 分割文本
            .map(|grapheme| {
                // 映射每个图元到 TextFragment
                let (replacement, rendered_width) = Self::replacement_character(grapheme)
                    .map_or_else(
                        || {
                            let unicode_width = grapheme.width();
                            let rendered_width = match unicode_width {
                                0 | 1 => GraphemeWidth::Half,
                                _ => GraphemeWidth::Full,
                            };
                            (None, rendered_width)
                        },
                        |replacement| (Some(replacement), GraphemeWidth::Half),
                    );
                TextFragment {
                    grapheme: grapheme.to_string(), // 转换为字符串
                    rendered_width,                 // 渲染宽度
                    replacement,                    // 替换字符
                }
            })
            .collect()
    }

    // 实现 Line 的方法，用于获取指定范围内的可见图元。
    pub fn get_visible_graphemes(&self, range: Range<usize>) -> String {
        if range.start >= range.end {
            return String::new(); // 如果范围无效，返回空字符串
        }

        let mut result = String::new(); // 初始化结果字符串
        let mut current_pos = 0; // 当前位置
        for fragment in &self.fragments {
            let fragment_end = fragment.rendered_width.saturating_add(current_pos); // 计算片段结束位置
            if current_pos >= range.end {
                break; // 如果当前位置超出范围，结束循环
            }

            if fragment_end >= range.start {
                if fragment_end > range.end || current_pos < range.start {
                    result.push('…'); // 如果片段超出范围，添加省略号
                } else if let Some(char) = fragment.replacement {
                    result.push(char); // 如果有替换字符，添加替换字符
                } else {
                    result.push_str(&fragment.grapheme); // 否则，添加图元
                }
            }
            current_pos = fragment_end; // 更新当前位置
        }
        result // 返回结果字符串
    }

    // 实现 Line 的方法，用于计算直到指定图元索引的宽度。
    pub fn width_until(&self, grapheme_index: usize) -> usize {
        self.fragments
            .iter() // 迭代片段
            .take(grapheme_index) // 取到指定索引
            .map(|fragment| match fragment.rendered_width {
                GraphemeWidth::Full => 2, // 全宽字符宽度为2
                GraphemeWidth::Half => 1, // 半宽字符宽度为1
            })
            .sum() // 计算总宽度
    }
    pub fn grapheme_count(&self) -> usize {
        self.fragments.len()
    }

    fn replacement_character(for_str: &str) -> Option<char> {
        let width = for_str.width();
        match for_str {
            " " => None,
            "\t" => Some(' '),
            _ if width > 0 && for_str.trim().is_empty() => Some('_'),
            _ if width == 0 => {
                let mut chars = for_str.chars();
                if let Some(ch) = chars.next() {
                    if ch.is_control() && chars.next().is_none() {
                        return Some('▯');
                    }
                }
                Some('·')
            }
            _ => None,
        }
    }
    pub fn insert_char(&mut self,character:char,grapheme_index: usize)
    {
        let mut result = String::new();
        for(index,fragment) in self.fragments.iter().enumerate()
        {
            if index== grapheme_index
            {
                result.push(character);
            }
            result.push_str(&fragment.grapheme);
        }
        if grapheme_index >= self.fragments.len()
        {
            result.push(character);
        }
        self.fragments = Self::str_to_fragments(&result);
    }
}
