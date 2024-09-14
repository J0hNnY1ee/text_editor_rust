
use super::{
    terminal::{Size, Terminal},
    DocumentStates,
};

pub struct StatusBus {
    current_status: DocumentStates,
    needs_redraw: bool,
    margin_bottom: usize,
    width: usize,
    position_y: usize,
}

impl StatusBus {
    pub fn new(margin_bottom: usize) -> Self {
        let size = Terminal::size().unwrap_or_default();
        Self {
            current_status: DocumentStates::default(),
            needs_redraw: true,
            margin_bottom,
            width: size.width,
            position_y: size.height.saturating_sub(margin_bottom).saturating_sub(1),
        }
    }

    pub fn resize(&mut self, size: Size) {
        self.width = size.width;
        self.position_y = size
            .height
            .saturating_sub(self.margin_bottom)
            .saturating_sub(1);
        self.needs_redraw = true;
    }
    pub fn update_status(&mut self, new_status: DocumentStates) {
        if new_status != self.current_status {
            self.current_status = new_status;
            self.needs_redraw = true;
        }
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let mut status = format!("{:?}", self.current_status);
        status.truncate(self.width);
        let result = Terminal::print_row(self.position_y, &status);
        debug_assert!(result.is_ok(), "Failed to render status bar");
        self.needs_redraw = false;
    }
}
