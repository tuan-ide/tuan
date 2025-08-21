use winit::keyboard::KeyCode;

use crate::document::Document;

#[derive(Debug)]
pub enum EditorAction {
    KeyPress(KeyCode),
    Scroll {
        delta: (f64, f64),
        document: Document,
    },
    AddCursor {
        document: Document,
        position: (usize, usize),
    },
    ClearCursors {
        document: Document,
    },
}
