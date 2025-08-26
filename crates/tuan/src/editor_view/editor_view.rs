use super::paint::line::Line;
use crate::theme;
use crate::theme::theme::Theme as _;
use crate::{document::Document, editor_view::EditorState};
use masonry::core::Modifiers;
use masonry::core::keyboard::Key;
use masonry::{
    accesskit::Role,
    core::{ScrollDelta, Widget},
    kurbo::Rect,
};
use std::time::Duration;
use winit::dpi::LogicalPosition;
use xilem::{Affine, Color};
use xilem::{
    Pod, ViewCtx, WidgetView,
    core::{MessageResult, View, ViewMarker, fork},
    tokio,
    view::{button, flex, task},
};

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    state.open_file(
        "/Users/arthur-fontaine/Developer/code/github.com/arthur-fontaine/agrume/package.json"
            .into(),
    );

    fork(
        // TODO: remove the flex box and the Open File button, those are just for testing
        flex((
            button("Open File", |state: &mut EditorState| {
                state.focus_document("/Users/arthur-fontaine/Developer/code/github.com/arthur-fontaine/agrume/package.json".into());
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
    state: EditorState,
    y_to_line_mapping: Vec<(f64, f64, Line)>,
}

impl EditorPortal {
    fn new(state: EditorState) -> Self {
        Self {
            state,
            y_to_line_mapping: Vec::new(),
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
        let document = self.state.get_focused_document();

        if document.is_none() {
            tracing::debug!("No focused document to paint");
            return;
        }
        let mut document = document.unwrap();

        let size = ctx.size();

        let background_rect = Rect::new(0.0, 0.0, size.width, size.height);
        let background_color = match &self.state.config.theme {
            theme::Theme::Vscode(vscode_theme) => vscode_theme
                .get_style(vec!["editor.background"])
                .and_then(|s| s.color),
        }
        .unwrap_or(Color::BLACK);
        scene.fill(
            masonry::peniko::Fill::EvenOdd,
            Affine::IDENTITY,
            background_color,
            None,
            &background_rect,
        );

        let scroll_delta = self
            .state
            .get_document_scroll(&document.path)
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
            .state
            .get_document_cursors(&document.path)
            .unwrap_or_else(Vec::new);

        let config = self.state.config.clone();
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
                    if let Some(focused_document) = self.state.get_focused_document() {
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
                ctx.request_focus();

                let focused_document = self.state.get_focused_document().unwrap();

                let scroll_delta = self
                    .state
                    .get_document_scroll(&focused_document.path)
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

    fn on_text_event(
        &mut self,
        ctx: &mut masonry::core::EventCtx<'_>,
        props: &mut masonry::core::PropertiesMut<'_>,
        event: &masonry::core::TextEvent,
    ) {
        if let masonry::core::TextEvent::Keyboard(key_event) = event {
            if key_event.state.is_down() {
                ctx.submit_action(EditorAction::KeyPress(
                    key_event.key.clone(),
                    key_event.modifiers,
                ));
            }
        }
    }

    fn accepts_focus(&self) -> bool {
        true
    }

    fn accepts_text_input(&self) -> bool {
        true
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
            ctx.with_action_widget(|_| (Pod::new(EditorPortal::new(app_state.clone())))),
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
        *element.widget = EditorPortal::new(app_state.clone());
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
                EditorAction::KeyPress(key, modifiers) => {
                    app_state.press_key(key.clone(), modifiers.clone());
                    MessageResult::RequestRebuild
                }
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
    KeyPress(Key, Modifiers),
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
