use crate::buffer;

#[derive(Debug, Clone)]
pub struct Document {
    buffer: buffer::Buffer,
}

impl Document {
    pub fn new(content: String, read_only: bool) -> Self {
        Self {
            buffer: buffer::Buffer::new(content, read_only),
        }
    }
}
