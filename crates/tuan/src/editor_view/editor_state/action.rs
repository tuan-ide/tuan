use crate::editor_view::paint::cursor::Cursor;
use serde::Deserialize;

#[derive(PartialEq, Eq, Debug, Deserialize, Clone)]
pub enum EditorAction {
    CursorLeft,
    CursorRight,
    CursorUp,
    CursorDown,
    CursorStart,
    CursorEnd,
    CursorTop,
    CursorBottom
}

impl super::EditorState {
    pub fn handle_action(&mut self, action: &EditorAction) {
        match action {
            EditorAction::CursorLeft => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_x_at(cursor.column.saturating_sub(1));
                    }
                });
            }
            EditorAction::CursorRight => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_x_at(cursor.column.saturating_add(1));
                    }
                });
            }
            EditorAction::CursorUp => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_y_at(cursor.line.saturating_sub(1));
                    }
                });
            }
            EditorAction::CursorDown => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_y_at(cursor.line.saturating_add(1));
                    }
                });
            }
            EditorAction::CursorStart => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_x_at(0);
                    }
                });
            }
            EditorAction::CursorEnd => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_x_at(usize::MAX);
                    }
                });
            }
            EditorAction::CursorTop => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_y_at(0);
                    }
                });
            }
            EditorAction::CursorBottom => {
                self.with_cursors_mut(|cursors| {
                    for cursor in cursors {
                        cursor.move_y_at(usize::MAX);
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
