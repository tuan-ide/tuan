use crate::editor_view::paint::cursor::Cursor;
use serde::Deserialize;

#[derive(PartialEq, Eq, Debug, Deserialize, Clone)]
pub enum EditorAction {
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
}

impl super::EditorState {
    pub fn handle_action(&mut self, action: &EditorAction) {
        match action {
            EditorAction::CursorLeft => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_left(1);
                    }
                });
            }
            EditorAction::CursorRight => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_right(1);
                    }
                });
            }
            EditorAction::CursorUp => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_up(1);
                    }
                });
            }
            EditorAction::CursorDown => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_down(1);
                    }
                });
            }
        }
    }

    fn with_cursors_mut<F>(&mut self, f: F) -> ()
    where
        F: FnOnce(&mut Vec<Cursor>) -> (),
    {
        if let Some(focused_path) = &self.focused_document_path {
            if let Some(cursors) = self.document_cursors.get_mut(focused_path) {
                f(cursors)
            }
        }
    }
}
