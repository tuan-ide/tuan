use super::line::Line;
use crate::editor_view::EditorState;
use masonry::{
    accesskit::Role,
    core::{ScrollDelta, Widget},
    kurbo::Rect,
};
use winit::dpi::LogicalPosition;
use xilem::{
    Pod, ViewCtx, WidgetView,
    core::{View, ViewMarker},
    view::{button, flex},
};

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
    y_to_line_mapping: Vec<(f64, f64, Line)>,
}

impl EditorPortal {
    fn new(state: *mut EditorState) -> Self {
        Self {
            state,
            y_to_line_mapping: Vec::new(),
        }
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
            self.y_to_line_mapping.clear();
            let lines = document.get_visible_lines(viewport);

            let config = self.with_state(|state| state.config.clone()).unwrap();
            for line in lines {
                let line = Line::new(&config, &line, &document, ctx);
                let (y_min, y_max) = line.paint_line(scene, scroll_delta);
                self.y_to_line_mapping.push((y_min, y_max, line));
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
            masonry::core::PointerEvent::Down {
                pointer,
                button,
                state,
            } => {
                let scroll_delta = self
                    .with_state(|state| state.get_focused_document_scroll())
                    .flatten()
                    .unwrap_or((0.0, 0.0));

                let position: LogicalPosition<f64> =
                    state.position.to_logical(ctx.get_scale_factor());

                let x = position.x - ctx.paint_rect().x0 - scroll_delta.0;
                let y = position.y - ctx.paint_rect().y0 - scroll_delta.1;

                let line = self
                    .y_to_line_mapping
                    .iter()
                    .find(|(y_min, y_max, _)| *y_min <= y && y <= *y_max)
                    .map(|(_, _, line)| line);

                let char_index = line
                    .map(|line| line.get_clicked_character_index(x as f32))
                    .flatten();

                line.map(|line| println!("Clicked line: {:?}", line.line))
                    .unwrap_or_else(|| {
                        println!("No line found at position: ({}, {})", x, y);
                    });

                println!("Clicked character index: {:?}", char_index);
                println!(
                    "Clicked character: {:?}",
                    line.and_then(|l| {
                        char_index.and_then(|index| l.line.content.chars().nth(index))
                    })
                );
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
        (Pod::new(EditorPortal::new(app_state)), ())
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem::core::Mut<Self::Element>,
        app_state: &mut EditorState,
    ) {
        *element.widget = EditorPortal::new(app_state);
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
