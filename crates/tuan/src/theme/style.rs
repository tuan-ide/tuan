#[derive(Debug, Clone)]
pub struct Style {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub strikethrough: bool,
}
