use gpui::{div, px, rgb, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::style::{Style};

pub struct ToolBar {
    pub style: Style
}

impl Render for ToolBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(self.style.toolbar.height))
            .bg(rgb(self.style.toolbar.bg_colour))
            .items_center()
            .text_color(rgb(self.style.text_colour))
            .child(format!("Hello, test!"))
    }
}