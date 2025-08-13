use std::{path::PathBuf, sync::Arc};

use masonry::kurbo::Rect;
use tuan_core::{syntax::Syntax, xi_rope::{spans::SpansInfo, tree::Node}};
use tuan_rpc::style::Style;

use super::{buffer, line};
use crate::editor_view;

#[derive(Debug, Clone)]
pub struct Document {
    path: PathBuf,
    buffer: buffer::Buffer,
    config: Arc<editor_view::EditorConfig>,
    styles: Option<Node<SpansInfo<Style>>>,
}

impl Document {
    pub fn new(
        path: PathBuf,
        content: String,
        read_only: bool,
        config: Arc<editor_view::EditorConfig>,
    ) -> Self {
        Self {
            path,
            buffer: buffer::Buffer::new(content, read_only),
            config,
            styles: None,
        }
    }

    pub fn get_visible_lines(&self, viewport: Rect) -> impl Iterator<Item = line::Line> {
        let line_height = self.config.real_line_height();

        let min_line = (viewport.y0 / line_height as f64).floor() as usize;
        let max_line = (viewport.y1 / line_height as f64).ceil() as usize;

        let lines = self.buffer.get_lines_in_range(min_line..max_line);

        line::Line::from_iter(lines, min_line)
    }

    pub fn get_styles(&self) -> Option<Node<SpansInfo<Style>>> {
        self.styles.clone()
    }

    pub fn update_styles_with_syntax(&mut self) {
        let mut syntax = Syntax::init(&self.path);
        syntax.parse(1, self.buffer.text.clone(), None);
        self.styles = syntax.styles;
    }
}
