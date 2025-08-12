use std::sync::Arc;

use masonry::kurbo::Rect;

use super::{line, buffer};
use crate::{editor_view};

#[derive(Debug, Clone)]
pub struct Document {
    buffer: buffer::Buffer,
    config: Arc<editor_view::EditorConfig>,
}

impl Document {
    pub fn new(content: String, read_only: bool, config: Arc<editor_view::EditorConfig>) -> Self {
        Self {
            buffer: buffer::Buffer::new(content, read_only),
            config,
        }
    }

    pub fn get_visible_lines(&self, viewport: Rect) -> impl Iterator<Item = line::Line> {
        let min_line = (viewport.y0 / self.config.line_height as f64).floor() as usize;
        let max_line = (viewport.y1 / self.config.line_height as f64).ceil() as usize;

        let lines = self.buffer.get_lines_in_range(min_line..max_line);

        line::Line::from_iter(lines, min_line)
    }
}
