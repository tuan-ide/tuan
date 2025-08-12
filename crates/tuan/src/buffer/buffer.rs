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
}
