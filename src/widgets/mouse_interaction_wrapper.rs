use cosmic::iced::{
    advanced::{
        layout, mouse, overlay, renderer,
        widget::{tree, Operation, OperationOutputWrapper, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event::{self, Event},
    Length, Rectangle,
};

use std::marker::PhantomData;

pub fn mouse_interaction_wrapper<'a, Msg, T: Into<cosmic::Element<'a, Msg>>>(
    mouse_interaction: mouse::Interaction,
    content: T,
) -> MouseInteractionWrapper<'a, Msg> {
    MouseInteractionWrapper {
        content: content.into(),
        mouse_interaction,
        _msg: PhantomData,
    }
}

pub struct MouseInteractionWrapper<'a, Msg> {
    content: cosmic::Element<'a, Msg>,
    mouse_interaction: mouse::Interaction,
    _msg: PhantomData<Msg>,
}

impl<'a, Msg> Widget<Msg, cosmic::Renderer> for MouseInteractionWrapper<'a, Msg> {
    delegate::delegate! {
        to self.content.as_widget() {
            fn tag(&self) -> tree::Tag;
            fn state(&self) -> tree::State;
            fn children(&self) -> Vec<Tree>;
            fn width(&self) -> Length;
            fn height(&self) -> Length;
            fn layout(
                    &self,
                    tree: &mut Tree,
                    renderer: &cosmic::Renderer,
                    limits: &layout::Limits,
                ) -> layout::Node;
            fn operate(
                    &self,
                    tree: &mut Tree,
                    layout: Layout<'_>,
                    renderer: &cosmic::Renderer,
                    operation: &mut dyn Operation<OperationOutputWrapper<Msg>>,
                );
            fn draw(
                &self,
                state: &Tree,
                renderer: &mut cosmic::Renderer,
                theme: &cosmic::Theme,
                style: &renderer::Style,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                viewport: &Rectangle,
            );
        }

        to self.content.as_widget_mut() {
            fn diff(&mut self, tree: &mut Tree);
            fn on_event(
                &mut self,
                tree: &mut Tree,
                event: Event,
                layout: Layout<'_>,
                cursor: mouse::Cursor,
                renderer: &cosmic::Renderer,
                clipboard: &mut dyn Clipboard,
                shell: &mut Shell<'_, Msg>,
                viewport: &Rectangle,
            ) -> event::Status;
            fn overlay<'b>(
                &'b mut self,
                tree: &'b mut Tree,
                layout: Layout<'_>,
                renderer: &cosmic::Renderer,
            ) -> Option<overlay::Element<'b, Msg, cosmic::Renderer>>;
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &cosmic::Renderer,
    ) -> mouse::Interaction {
        self.mouse_interaction
    }
}

impl<'a, Msg: 'static> From<MouseInteractionWrapper<'a, Msg>> for cosmic::Element<'a, Msg> {
    fn from(widget: MouseInteractionWrapper<'a, Msg>) -> Self {
        cosmic::Element::new(widget)
    }
}
