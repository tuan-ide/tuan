use std::sync::Arc;

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
}
