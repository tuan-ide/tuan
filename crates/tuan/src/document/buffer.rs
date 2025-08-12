use std::{borrow::Cow, ops::Range};

use tuan_core::xi_rope::Rope;

#[derive(Debug, Clone)]
pub struct Buffer {
    text: Rope,
}

impl Buffer {
    pub fn new(content: String, read_only: bool) -> Self {
        Self {
            text: Rope::from(content),
        }
    }

    pub fn get_lines_in_range(&self, range: Range<usize>) -> impl Iterator<Item = Cow<'_, str>> {
        self.text.lines(..).skip(range.start).take(range.end - range.start)
    }
}
