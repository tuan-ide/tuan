use std::ops::Range;

use tuan_core::xi_rope::{Rope, rope::Lines};

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

    pub fn get_lines_in_range(&self, range: Range<usize>) -> Lines {
        self.text.lines(range)
    }
}
