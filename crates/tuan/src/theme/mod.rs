pub(super) mod theme;
mod style;
pub mod vscode_theme;

pub use style::*;

#[derive(Clone, Debug)]
pub enum Theme {
    Vscode(vscode_theme::VscodeTheme),
}

impl Theme {
    pub fn from_vscode_theme(theme: vscode_theme::VscodeTheme) -> Self {
        Self::Vscode(theme)
    }
}
