use xilem::{WidgetView, view::flex};

use crate::editor_view::EditorState;

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    flex(())
}
