#[derive(Clone, Debug)]
pub struct EditorConfig {
    pub font_size: u32,
    pub line_height: f32,
}

impl Default for EditorConfig {
    fn default() -> Self {
        Self {
            font_size: 14,
            line_height: 1.5,
        }
    }
}
