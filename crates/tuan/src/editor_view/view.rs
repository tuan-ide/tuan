use masonry::{accesskit::Role, core::Widget, kurbo::Rect};
use xilem::{
    core::{View, ViewMarker}, view::{button, flex}, Pod, ViewCtx, WidgetView
};

use crate::editor_view::EditorState;

pub fn editor_view(state: &mut EditorState) -> impl WidgetView<EditorState> + use<> {
    state.open_file("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());

    flex((
        button("Open File", |state: &mut EditorState| {
            state.focus_document("/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie/package.json".into());
        }),
        EditorView {
            state: state.clone(),
        },
    ))
}

#[derive(Clone)]
struct EditorView {
    state: EditorState,
}

impl Widget for EditorView {
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
        let viewport = Rect::new(0.0, 0.0, size.width, size.height);

        let document = self.state.get_focused_document();
        if let Some(doc) = document {
            let lines = doc.get_visible_lines(viewport);
            for (i, line) in lines.enumerate() {
                let text = format!("{:?}", line);

                println!("render line {}: {}", i, text);
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
}

impl ViewMarker for EditorView {}

impl<State, Action> View<State, Action, ViewCtx> for EditorView {
    type Element = Pod<EditorView>;
    type ViewState = ();

    fn build(&self, ctx: &mut ViewCtx, app_state: &mut State) -> (Self::Element, Self::ViewState) {
        (Pod::new(self.clone()), ())
    }

    fn rebuild(
        &self,
        prev: &Self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        mut element: xilem::core::Mut<Self::Element>,
        app_state: &mut State,
    ) {
        if prev.state.focused_document_path != self.state.focused_document_path {
            *element.widget = self.clone();
            element.ctx.request_render();
        }
    }

    fn teardown(
        &self,
        view_state: &mut Self::ViewState,
        ctx: &mut ViewCtx,
        element: xilem::core::Mut<'_, Self::Element>,
        app_state: &mut State,
    ) {
        ctx.teardown_leaf(element);
    }

    fn message(
        &self,
        view_state: &mut Self::ViewState,
        id_path: &[xilem::core::ViewId],
        message: xilem::core::DynMessage,
        app_state: &mut State,
    ) -> xilem::core::MessageResult<Action> {
        debug_assert!(
            !id_path.is_empty(),
            "id path should be non-empty in GameView::message"
        );

        // but we haven't set up children yet, so shouldn't be empty either (should just not get here)
        unreachable!("message should not be sent to GameView without child.");
    }
}
