use hex_color::HexColor;
use masonry::{
    TextAlignOptions,
    accesskit::Role,
    core::{BrushIndex, ScrollDelta, Widget},
    kurbo::Rect,
    parley::{FontFamily, FontStack, FontStyle, GenericFamily, StyleProperty},
    peniko::Brush,
};
use xilem::{
    Affine, Color, FontWeight, Pod, TextAlign, ViewCtx, WidgetView,
    core::{View, ViewMarker},
    view::{button, flex},
};

use crate::{document, editor_view::EditorState};

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    state.open_file("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());

    flex((
        button("Open File", |state: &mut EditorState| {
            state.focus_document("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());
        }),
        EditorView,
    ))
}

struct EditorPortal {
    state: *mut EditorState, // TODO: Try with Arc or Rc
}

impl EditorPortal {
    fn paint_line(
        &mut self,
        line: document::line::Line,
        document: &document::Document,
        ctx: &mut masonry::core::PaintCtx<'_>,
        scene: &mut masonry::vello::Scene,
        scroll_delta: (f64, f64),
    ) {
        let text = line.content;
        let font_size = self.with_state(|state| state.config.font_size).unwrap();
        let line_height = self
            .with_state(|state| state.config.real_line_height())
            .unwrap();

        let styles = document
            .get_styles_in_range(line.start, line.end)
            .collect::<Vec<_>>();

        let (fcx, lcx) = ctx.text_contexts();
        let mut text_layout_builder = lcx.ranged_builder(fcx, &text, 1.0, true);

        text_layout_builder.push_default(StyleProperty::FontStack(FontStack::Single(
            FontFamily::Generic(GenericFamily::Monospace),
        )));
        text_layout_builder.push_default(StyleProperty::FontSize(font_size));

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

        let transform = Affine::translate((
            scroll_delta.0,
            (scroll_delta.1) + (line.line_number as f64 * line_height as f64),
        ));

        masonry::core::render_text(
            scene,
            transform,
            &text_layout,
            &brushes,
            true, // hinting
        );
    }

    fn with_state<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut EditorState) -> R,
    {
        if self.state.is_null() {
            return None;
        }

        unsafe {
            let state = &mut *self.state;
            if !state.initialized {
                println!("EditorState is not initialized");
                return None;
            }
            Some(f(state))
        }
    }
}

impl Widget for EditorPortal {
    fn layout(
        &mut self,
        _ctx: &mut masonry::core::LayoutCtx<'_>,
        _props: &mut masonry::core::PropertiesMut<'_>,
        bc: &masonry::core::BoxConstraints,
    ) -> masonry::kurbo::Size {
        bc.max()
    }

    fn paint(
        &mut self,
        ctx: &mut masonry::core::PaintCtx<'_>,
        props: &masonry::core::PropertiesRef<'_>,
        scene: &mut masonry::vello::Scene,
    ) {
        let size = ctx.size();
        let scroll_delta = self
            .with_state(|state| state.get_focused_document_scroll())
            .flatten()
            .unwrap_or((0.0, 0.0));
        let viewport = Rect::new(
            -scroll_delta.0,
            -scroll_delta.1,
            size.width - scroll_delta.0,
            size.height - scroll_delta.1,
        );

        let document = self
            .with_state(|state| state.get_focused_document())
            .flatten();

        if let Some(mut document) = document {
            document.update_styles_with_syntax();
            let lines = document.get_visible_lines(viewport);

            for line in lines {
                self.paint_line(line, &document, ctx, scene, scroll_delta);
            }
        }
    }

    fn accessibility_role(&self) -> masonry::accesskit::Role {
        Role::MultilineTextInput
    }

    fn accessibility(
        &mut self,
        ctx: &mut masonry::core::AccessCtx<'_>,
        props: &masonry::core::PropertiesRef<'_>,
        node: &mut masonry::accesskit::Node,
    ) {
        // TODO
    }

    fn register_children(&mut self, ctx: &mut masonry::core::RegisterCtx<'_>) {
        // TODO
    }

    fn children_ids(&self) -> masonry::core::ChildrenIds {
        // TODO
        masonry::core::ChildrenIds::new()
    }

    fn on_pointer_event(
        &mut self,
        ctx: &mut masonry::core::EventCtx<'_>,
        props: &mut masonry::core::PropertiesMut<'_>,
        event: &masonry::core::PointerEvent,
    ) {
        match event {
            masonry::core::PointerEvent::Scroll {
                pointer,
                delta,
                state,
            } => {
                if let ScrollDelta::PixelDelta(delta) = delta {
                    self.with_state(|state| {
                        state.scroll_focused_document((delta.x, delta.y));
                    });
                    ctx.request_render();
                }
            }
            _ => {}
        }
    }
}

struct EditorView;
impl ViewMarker for EditorView {}
impl View<EditorState, (), ViewCtx> for EditorView {
    type Element = Pod<EditorPortal>;
    type ViewState = ();

    fn build(
        &self,
        ctx: &mut ViewCtx,
        app_state: &mut EditorState,
    ) -> (Self::Element, Self::ViewState) {
        (Pod::new(EditorPortal { state: app_state }), ())
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem::core::Mut<Self::Element>,
        app_state: &mut EditorState,
    ) {
        *element.widget = EditorPortal { state: app_state };
        element.ctx.request_render();
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: xilem::core::Mut<'_, Self::Element>,
        app_state: &mut EditorState,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[xilem::core::ViewId],
        message: xilem::core::DynMessage,
        app_state: &mut EditorState,
    ) -> xilem::core::MessageResult<()> {
        debug_assert!(
            !id_path.is_empty(),
            "id path should be non-empty in GameView::message"
        );

        // but we haven't set up children yet, so shouldn't be empty either (should just not get here)
        unreachable!("message should not be sent to GameView without child.");
    }
}
