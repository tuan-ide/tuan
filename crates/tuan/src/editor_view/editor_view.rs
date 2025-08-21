use super::paint::line::Line;
use crate::{document::Document, editor_view::EditorState};
use masonry::{
    accesskit::Role,
    core::{ScrollDelta, Widget},
    kurbo::Rect,
};
use std::time::Duration;
use winit::{dpi::LogicalPosition, keyboard::KeyCode};
use xilem::{
    Pod, ViewCtx, WidgetView,
    core::{MessageResult, View, ViewMarker, fork},
    tokio,
    view::{button, flex, task},
};

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    state.open_file("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());

    fork(
        // TODO: remove the flex box and the Open File button, those are just for testing
        flex((
            button("Open File", |state: &mut EditorState| {
                state.focus_document("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());
            }),
            EditorView,
        )),
        task(
            async move |proxy| {
                let mut interval = tokio::time::interval(Duration::from_millis(500));
                loop {
                    interval.tick().await;
                    let Ok(()) = proxy.message(()) else {
                        break;
                    };
                }
            },
            |data: &mut EditorState, ()| {
                data.tick_cursors();
            },
        ),
    )
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
                tracing::debug!("EditorState is not initialized");
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
        let document = self
            .with_state(|state| state.get_focused_document())
            .flatten();

        if document.is_none() {
            tracing::debug!("No focused document to paint");
            return;
        }
        let mut document = document.unwrap();

        let size = ctx.size();

        let scroll_delta = self
            .with_state(|state| state.get_document_scroll(&document.path))
            .flatten()
            .unwrap_or((0.0, 0.0));
        let viewport = Rect::new(
            -scroll_delta.0,
            -scroll_delta.1,
            size.width - scroll_delta.0,
            size.height - scroll_delta.1,
        );

        document.update_styles_with_syntax();
        self.y_to_line_mapping.clear();

        let lines = document.get_visible_lines(viewport);
        let cursors = self
            .with_state(|state| state.get_document_cursors(&document.path))
            .flatten()
            .unwrap_or_else(Vec::new);

        let config = self.with_state(|state| state.config.clone()).unwrap();
        let lines = lines
            .into_iter()
            .map(|line| Line::new(&config, &line, &document, ctx, cursors.clone()))
            .collect::<Vec<_>>();

        for cursor in cursors {
            cursor.paint(scene, scroll_delta, &lines);
        }

        for line in &lines {
            let (y_min, y_max) = line.paint(scene, scroll_delta);
            self.y_to_line_mapping.push((y_min, y_max, line.clone()));
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
                    if let Some(focused_document) = self
                        .with_state(|state| state.get_focused_document())
                        .flatten()
                    {
                        ctx.submit_action(EditorAction::Scroll {
                            delta: (delta.x, delta.y),
                            document: focused_document,
                        });
                    }
                }
            }
            masonry::core::PointerEvent::Down {
                pointer,
                button,
                state,
            } => {
                let focused_document = self
                    .with_state(|state| state.get_focused_document())
                    .flatten()
                    .expect("Focused document should not be None");

                let scroll_delta = self
                    .with_state(|state| state.get_document_scroll(&focused_document.path))
                    .flatten()
                    .unwrap_or((0.0, 0.0));

                let position: LogicalPosition<f64> =
                    state.position.to_logical(ctx.get_scale_factor());

                let x = position.x - ctx.paint_rect().x0 - scroll_delta.0;
                let y = position.y - ctx.paint_rect().y0 - scroll_delta.1;

                let (line_number, char_index) = self
                    .y_to_line_mapping
                    .iter()
                    .find(|(y_min, y_max, _)| *y_min <= y && y <= *y_max)
                    .map(|(_, _, line)| {
                        (
                            line.line.line_number,
                            line.get_clicked_character_index(x as f32).unwrap_or(0),
                        )
                    })
                    .unwrap_or((0, 0));

                ctx.submit_action(EditorAction::ClearCursors {
                    document: focused_document.clone(),
                });
                ctx.submit_action(EditorAction::AddCursor {
                    document: focused_document,
                    position: (line_number, char_index),
                });
            }
            _ => {}
        }
    }

    fn get_debug_text(&self) -> Option<String> {
        "EditorPortal".to_string().into()
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
        (
            ctx.with_action_widget(|_| (Pod::new(EditorPortal::new(app_state)))),
            (),
        )
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
        if let Ok(editor_action) = message.downcast::<EditorAction>() {
            match editor_action.as_ref() {
                EditorAction::KeyPress(key_code) => MessageResult::Nop,
                EditorAction::Scroll { delta, document } => {
                    app_state.scroll_document(&document.path, (delta.0, delta.1));
                    MessageResult::RequestRebuild
                }
                EditorAction::AddCursor { document, position } => {
                    app_state.add_cursor(document.path.clone(), position);
                    MessageResult::RequestRebuild
                }
                EditorAction::ClearCursors { document } => {
                    app_state.clear_cursors(document.path.clone());
                    MessageResult::RequestRebuild
                }
            }
        } else {
            MessageResult::Nop
        }
    }
}

#[derive(Debug)]
enum EditorAction {
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
