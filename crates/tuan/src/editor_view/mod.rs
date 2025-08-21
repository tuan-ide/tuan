mod state;
mod view;
mod config;
pub(super) mod action;
pub(super) mod line;
pub(super) mod cursor;

pub use state::EditorState;
pub use view::editor_view;
pub use config::EditorConfig;
