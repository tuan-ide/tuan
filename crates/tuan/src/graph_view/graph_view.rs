use crate::graph_view::graph_descriptor::create_graph_descriptor;
use crate::graph_view::{GraphState, camera};
use crate::theme;
use crate::theme::theme::Theme as _;
use masonry::core::ScrollDelta;
use masonry::kurbo::{Circle, Line, Point, Rect};
use masonry::{accesskit::Role, core::Widget};
use winit::dpi::LogicalPosition;
use xilem::{Affine, Color, Vec2};
use xilem::{
    Pod, ViewCtx, WidgetView,
    core::{MessageResult, View, ViewMarker},
};

pub fn graph_view(state: &mut GraphState) -> impl WidgetView<GraphState> + use<> {
    GraphView
}

struct GraphPortal {
    state: GraphState,
}

impl GraphPortal {
    fn new(state: GraphState) -> Self {
        Self { state }
    }
}

impl Widget for GraphPortal {
    type Action = GraphAction;

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

        let background_rect = Rect::new(0.0, 0.0, size.width, size.height);
        let background_color = match &self.state.editor_config.theme {
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

        self.state
            .init_camera((size.width as f64, size.height as f64));

        let camera = self.state.camera.borrow();
        let camera = camera.as_ref().unwrap();

        let graph = self.state.graph.get_graph();

        let graph_descriptor = create_graph_descriptor(graph, &camera);

        let edge_color = match &self.state.editor_config.theme {
            theme::Theme::Vscode(vscode_theme) => vscode_theme
                .get_style(vec!["editor.foreground"])
                .and_then(|s| s.color),
        }
        .unwrap_or(Color::BLACK);

        for edge in graph_descriptor.edges {
            scene.stroke(
                &masonry::kurbo::Stroke::new(edge.width),
                Affine::IDENTITY,
                edge_color,
                None,
                &Line::new(
                    Point {
                        x: edge.source_position.x as f64,
                        y: edge.source_position.y as f64,
                    },
                    Point {
                        x: edge.target_position.x as f64,
                        y: edge.target_position.y as f64,
                    },
                ),
            );
        }

        for node in graph_descriptor.nodes {
            scene.fill(
                masonry::peniko::Fill::EvenOdd,
                Affine::IDENTITY,
                edge_color,
                None,
                &Circle::new(
                    Point {
                        x: node.position.x as f64,
                        y: node.position.y as f64,
                    },
                    node.radius,
                ),
            );
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
            masonry::core::PointerEvent::Scroll(movement) => match movement.delta {
                ScrollDelta::PixelDelta(physical_position) => {
                    let position: LogicalPosition<f64> =
                        physical_position.to_logical(ctx.get_scale_factor());
                    ctx.submit_action::<GraphAction>(GraphAction::Translate(Vec2::new(
                        position.x,
                        position.y,
                    )));
                }
                _ => {}
            },
            masonry::core::PointerEvent::Gesture(gesture) => match gesture.gesture {
                masonry::core::pointer::PointerGesture::Pinch(pinch_delta) => {
                    let logical_position: LogicalPosition<f64> =
                        gesture.state.position.to_logical(ctx.get_scale_factor());
                    println!(
                        "origin: {:?}, delta: {:?}",
                        logical_position, pinch_delta
                    );
                    ctx.submit_action::<GraphAction>(GraphAction::Zoom(
                        pinch_delta as f64,
                        Vec2::new(logical_position.x, logical_position.y),
                    ));
                }
                _ => {}
            },
            _ => {
                // println!("GraphPortal received pointer event: {:?}", event);
            }
        }
    }

    fn get_debug_text(&self) -> Option<String> {
        "GraphPortal".to_string().into()
    }
}

struct GraphView;
impl ViewMarker for GraphView {}
impl View<GraphState, (), ViewCtx> for GraphView {
    type Element = Pod<GraphPortal>;
    type ViewState = ();

    fn build(
        &self,
        ctx: &mut ViewCtx,
        app_state: &mut GraphState,
    ) -> (Self::Element, Self::ViewState) {
        (
            ctx.with_action_widget(|_| (Pod::new(GraphPortal::new(app_state.clone())))),
            (),
        )
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem::core::Mut<Self::Element>,
        app_state: &mut GraphState,
    ) {
        *element.widget = GraphPortal::new(app_state.clone());
        element.ctx.request_render();
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: xilem::core::Mut<'_, Self::Element>,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        message: &mut xilem::core::MessageContext,
        element: xilem::core::Mut<'_, Self::Element>,
        app_state: &mut GraphState,
    ) -> xilem::core::MessageResult<()> {
        if let Some(graph_action) = message.take_message::<GraphAction>() {
            match graph_action.as_ref() {
                GraphAction::Zoom(factor, origin) => {
                    let mut camera = app_state.camera.borrow_mut();
                    if let Some(camera) = &mut *camera {
                        camera.zoom(*factor, Some(*origin));
                        MessageResult::RequestRebuild
                    } else {
                        MessageResult::Nop
                    }
                }
                GraphAction::Translate(xy) => {
                    let mut camera = app_state.camera.borrow_mut();
                    if let Some(camera) = &mut *camera {
                        camera.translate(*xy);
                        MessageResult::RequestRebuild
                    } else {
                        MessageResult::Nop
                    }
                }
            }
        } else {
            MessageResult::Nop
        }
    }
}

#[derive(Debug)]
enum GraphAction {
    Zoom(f64, Vec2),
    Translate(Vec2),
}
