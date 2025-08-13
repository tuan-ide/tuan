#[derive(Debug, Clone)]
pub struct Line {
    pub content: String,
    pub line_number: usize,
    pub start: usize,
    pub end: usize,
}
