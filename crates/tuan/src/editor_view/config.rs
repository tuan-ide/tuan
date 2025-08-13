use crate::theme;

#[derive(Clone, Debug)]
pub struct EditorConfig {
    pub font_size: f32,
    pub line_height: f32,
    pub theme: theme::Theme,
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
            theme: theme::Theme::from_vscode_theme(
                theme::vscode_theme::VscodeTheme::from_path("/Users/arthurfontaine/Developer/code/github.com/arthur-fontaine/tuan/crates/tuan/assets/rose-pine-color-theme.json".into()).unwrap()
            ),
        }
    }
}
