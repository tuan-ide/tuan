use winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem, core::lens};

use crate::editor_view::{EditorState, editor_view};

mod editor_view;
mod globals;

pub struct AppState {
    editor_state: EditorState,
}

impl AppState {
    fn new() -> Self {
        Self {
            editor_state: EditorState::new(),
        }
    }
}

fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    lens(editor_view, |s: &mut AppState| &mut s.editor_state)
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(AppState::new(), app_logic, WindowOptions::new("Tuan"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
