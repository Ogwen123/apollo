use gpui::{div, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::components::tab_bar::TabBar;
use crate::style::Style;

pub struct MainPanel {
    pub style: Style,
}

impl Render for MainPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .child(cx.new(|_| TabBar {
                style: self.style.clone()
            }))
            .child("Main panel")
    }
}