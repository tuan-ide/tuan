use std::ops::Range;

use tuan_core::{buffer::rope_text::RopeText, xi_rope::Rope};

use super::line::Line;

#[derive(Debug, Clone)]
pub struct Buffer {
    pub(super) text: Rope,
}

impl Buffer {
    pub fn new(content: String, read_only: bool) -> Self {
        Self {
            text: Rope::from(content),
        }
    }

    pub fn get_lines_in_range(&self, range: Range<usize>) -> impl Iterator<Item = Line> {
        self.iter_lines()
            .skip(range.start)
            .take(range.end - range.start)
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = Line> {
        self.text.lines(..).enumerate().map(|(i, line)| Line {
            content: line.to_string(),
            line_number: i,
            start: self.text.offset_of_line(i),
            end: self.text.line_end_offset(i, true),
        })
    }
}
