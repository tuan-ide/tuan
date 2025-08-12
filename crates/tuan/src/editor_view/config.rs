#[derive(Clone, Debug)]
pub struct EditorConfig {
    pub font_size: f32,
    pub line_height: f32,
}

impl EditorConfig {
    pub fn real_line_height(&self) -> f32 {
        self.font_size * self.line_height
    }
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_size: 14.0,
            line_height: 1.5,
        }
    }
}
