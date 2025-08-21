use std::{fmt::Debug, path::PathBuf, sync::Arc};

use masonry::kurbo::Rect;
use tuan_core::{buffer::rope_text::RopeText, syntax::Syntax};

use super::line;
use crate::{
    editor_view,
    theme::{self, theme::Theme as _},
};

#[derive(Debug, Clone)]
pub struct RangeStyle {
    pub start: usize,
    pub end: usize,
    pub style: theme::Style,
}

#[derive(Clone)]
pub struct Document {
    pub(crate) path: PathBuf,
    buffer: tuan_core::buffer::Buffer,
    config: Arc<editor_view::EditorConfig>,
    styles: Vec<RangeStyle>,
}

impl Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("path", &self.path)
            .field("buffer", &self.buffer.text().len())
            .field("config", &"&self.config")
            .field("styles", &self.styles.len())
            .finish()
    }
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
            buffer: tuan_core::buffer::Buffer::new(content),
            config,
            styles: Vec::new(),
        }
    }

    pub fn get_visible_lines(&self, viewport: Rect) -> impl Iterator<Item = line::Line> {
        let line_height = self.config.real_line_height();

        let min_line = (viewport.y0 / line_height as f64).floor() as usize;
        let max_line = (viewport.y1 / line_height as f64).ceil() as usize;

        (min_line..max_line).map(move |line_number| line::Line {
            content: self.buffer.line_content(line_number).to_string(),
            line_number,
            start: self.buffer.offset_of_line(line_number),
            end: self.buffer.line_end_offset(line_number, true),
        })
    }

    pub fn get_styles_in_range(
        &self,
        start: usize,
        end: usize,
    ) -> impl Iterator<Item = &RangeStyle> {
        self.styles
            .iter()
            .filter(move |style| style.start < end && style.end > start)
    }

    pub fn get_line_length(&self, line: usize) -> usize {
        self.buffer.line_len(line)
    }

    pub fn count_lines(&self) -> usize {
        self.buffer.num_lines()
    }

    pub fn update_styles_with_syntax(&mut self) {
        let mut syntax = Syntax::init(&self.path);
        syntax.parse(1, self.buffer.text().clone(), None);
        self.styles = if let Some(syntax_styles) = syntax.styles {
            syntax_styles
                .iter()
                .map(|(interval, style)| {
                    style
                        .fg_color
                        .as_ref()
                        .and_then(|fg_color| match &self.config.theme {
                            theme::Theme::Vscode(vscode_theme) => {
                                vscode_theme.get_style(vec![fg_color])
                            }
                        })
                        .map(|style| RangeStyle {
                            start: interval.start,
                            end: interval.end,
                            style,
                        })
                })
                .flatten()
                .collect()
        } else {
            Vec::new()
        };
    }
}
