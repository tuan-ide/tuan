use std::sync::Arc;

use masonry::kurbo::Rect;
use xilem::{Affine, Color};

use crate::theme::{self, theme::Theme};

use super::{config::EditorConfig, line};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlinkState {
    On,
    Off,
}

#[derive(Clone, Debug)]
pub struct Cursor {
    pub line: usize,
    pub column: usize,
    pub blink_state: BlinkState,
    editor_config: Arc<EditorConfig>,
}

impl Cursor {
    pub fn new(line: usize, column: usize, editor_config: Arc<EditorConfig>) -> Self {
        Self {
            line,
            column,
            blink_state: BlinkState::On,
            editor_config,
        }
    }

    fn set_blink_state(&mut self, state: BlinkState) {
        self.blink_state = state;
    }

    fn next_blink_state(&mut self) {
        let new_state = match self.blink_state {
            BlinkState::On => BlinkState::Off,
            BlinkState::Off => BlinkState::On,
        };
        self.set_blink_state(new_state);
    }

    pub fn tick(&mut self) {
        self.next_blink_state();
    }
}

impl Cursor {
    pub(super) fn paint(
        &self,
        scene: &mut masonry::vello::Scene,
        scroll_delta: (f64, f64),
        lines: &Vec<line::Line>,
    ) -> Option<()> {
        let line_height = self.editor_config.real_line_height();

        let line = lines.iter().find(|l| l.line.line_number == self.line)?;

        let x_range = line.get_x_range_for_index(self.column)?;

        let x = x_range.0 as f64 + scroll_delta.0;
        let y = ((self.line as f32) * line_height) as f64 + scroll_delta.1;
        let width = (x_range.1 - x_range.0) as f64;
        let height = self.editor_config.real_line_height() as f64;

        let cursor_rect = match self.blink_state {
            BlinkState::On => Rect::new(x, y, x + width, y + height),
            BlinkState::Off => Rect::new(0.0, 0.0, 0.0, 0.0),
        };

        let cursor_color = match &self.editor_config.theme {
            theme::Theme::Vscode(vscode_theme) => vscode_theme
                .get_style(vec!["editorCursor.foreground"])
                .and_then(|s| s.color),
        }
        .unwrap_or(Color::BLACK);

        scene.fill(
            masonry::peniko::Fill::EvenOdd,
            Affine::IDENTITY,
            cursor_color,
            None,
            &cursor_rect,
        );

        Some(())
    }
}
