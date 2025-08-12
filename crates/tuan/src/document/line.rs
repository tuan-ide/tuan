#[derive(Debug, Clone)]
pub struct Line {
    pub content: String,
    pub line_number: usize,
}

impl Line {
    pub fn from_iter<I>(lines: I, start_line: usize) -> impl Iterator<Item = Line>
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        lines.into_iter().enumerate().map(move |(i, line)| {
            Line {
                line_number: start_line + i,
                content: line.as_ref().to_string(), // Convert to String here
            }
        })
    }
}
