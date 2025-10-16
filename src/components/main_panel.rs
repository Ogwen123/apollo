use gpui::{div, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::style::Style;

pub struct MainPanel {
    pub style: Style,
}

impl Render for MainPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .h_full()
            .w_full()
            .child("Main panel")
    }
}