use crate::components::tab_bar::TabBar;
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct Workspace {
    pub style: Style,
}

impl Render for Workspace {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .child(cx.new(|_| TabBar {
                style: self.style.clone(),
            }))
            .child("Workspace")
    }
}
