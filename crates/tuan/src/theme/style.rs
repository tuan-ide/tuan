use xilem::Color;

#[derive(Debug, Clone)]
pub struct Style {
    pub foreground: Option<Color>,
    pub background: Option<Color>,
    pub italic: bool,
    pub bold: bool,
    pub underline: bool,
    pub strikethrough: bool,
}
