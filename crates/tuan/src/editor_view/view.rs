use xilem::{WidgetView, view::button};

use crate::editor_view::EditorState;

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    button("Click me", |state: &mut EditorState| {
        state.open_file("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());
    })
}
