use crate::{document, editor_view::EditorConfig};
use hex_color::HexColor;
use masonry::{
    TextAlignOptions,
    core::BrushIndex,
    parley::{
        FontFamily, FontStack, FontStyle, GenericFamily, PositionedLayoutItem, StyleProperty,
    },
    peniko::Brush,
};
use xilem::{Affine, Color, FontWeight, TextAlign};

pub(super) struct Line {
    editor_config: EditorConfig,
    x_to_character_index_mapping: Vec<(f32, f32, usize)>, // (start, end, index)
    text_layout: masonry::parley::Layout<BrushIndex>,
    brushes: Vec<Brush>,
    pub(super) line: document::line::Line,
}

impl Line {
    pub fn new(
        config: &EditorConfig,
        line: &document::line::Line,
        document: &document::Document,
        paint_ctx: &mut masonry::core::PaintCtx<'_>,
    ) -> Self {
        let (text_layout, brushes, range_index_mapping) =
            Self::get_line_text_layout(&config, line, document, paint_ctx);

        Self {
            editor_config: config.clone(),
            x_to_character_index_mapping: range_index_mapping,
            text_layout,
            brushes,
            line: line.clone(),
        }
    }

    fn get_line_text_layout(
        editor_config: &EditorConfig,
        line: &document::line::Line,
        document: &document::Document,
        ctx: &mut masonry::core::PaintCtx<'_>,
    ) -> (
        masonry::parley::Layout<BrushIndex>,
        Vec<masonry::peniko::Brush>,
        Vec<(f32, f32, usize)>,
    ) {
        let text = line.clone().content;

        let styles = document
            .get_styles_in_range(line.start, line.end)
            .collect::<Vec<_>>();

        let (fcx, lcx) = ctx.text_contexts();
        let mut text_layout_builder = lcx.ranged_builder(fcx, &text, 1.0, true);

        text_layout_builder.push_default(StyleProperty::FontStack(FontStack::Single(
            FontFamily::Generic(GenericFamily::Monospace),
        )));
        text_layout_builder.push_default(StyleProperty::FontSize(editor_config.font_size));

        let mut brushes: Vec<Brush> = vec![];
        for style in styles {
            let range = (style.start - line.start)..(style.end - line.start);
            let style = style.style.clone();

            if style.italic {
                text_layout_builder
                    .push(StyleProperty::FontStyle(FontStyle::Italic), range.clone());
            }
            if style.bold {
                text_layout_builder
                    .push(StyleProperty::FontWeight(FontWeight::BOLD), range.clone());
            }
            if style.underline {
                text_layout_builder.push(StyleProperty::Underline(true), range.clone());
            }
            if style.strikethrough {
                text_layout_builder.push(StyleProperty::Strikethrough(true), range.clone());
            }
            if let Some(color) = style.foreground {
                let color = HexColor::parse(&color).expect("Failed to parse color");
                brushes.push(Color::from_rgba8(color.r, color.g, color.b, color.a).into());

                let brush_index = BrushIndex(brushes.len() - 1);

                text_layout_builder.push(StyleProperty::Brush(brush_index), range.clone());
            }
        }

        let mut text_layout = text_layout_builder.build(&text);
        text_layout.break_all_lines(None);
        text_layout.align(None, TextAlign::Start, TextAlignOptions::default());

        let mut range_index_mapping = Vec::new();
        if let Some(line) = text_layout.lines().next() {
            let mut character_index = 0;
            let mut x = 0.0;

            for item in line.items() {
                if let PositionedLayoutItem::GlyphRun(glyph_run) = item {
                    for glyph in glyph_run.glyphs() {
                        let new_x = x + glyph.advance;

                        range_index_mapping.push((x, new_x, character_index));

                        character_index += 1;
                        x = new_x;
                    }
                }
            }
        }

        (text_layout, brushes, range_index_mapping)
    }

    pub(super) fn paint_line(
        &self,
        scene: &mut masonry::vello::Scene,
        scroll_delta: (f64, f64),
    ) -> (f64, f64) {
        let line = &self.line;
        let text_layout = &self.text_layout;
        let brushes = &self.brushes;
        let line_height = self.editor_config.real_line_height();

        let y_min = line.line_number as f64 * line_height as f64;
        let y_max = y_min + line_height as f64;

        let transform = Affine::translate((scroll_delta.0, scroll_delta.1 + y_min));

        masonry::core::render_text(
            scene,
            transform,
            &text_layout,
            &brushes,
            true, // hinting
        );

        return (y_min, y_max);
    }

    pub fn get_clicked_character_index(&self, x: f32) -> Option<usize> {
        for (start, end, index) in &self.x_to_character_index_mapping {
            if x >= *start && x <= *end {
                return Some(*index);
            }
        }
        None
    }
}
