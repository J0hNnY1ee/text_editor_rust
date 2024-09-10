pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: vec!["Hello, World!".to_string()],
        }
    }
}
