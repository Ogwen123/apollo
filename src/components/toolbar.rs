use gpui::{div, rgb, Context, IntoElement, ParentElement, Render, Styled, Window};

pub struct ToolBar {

}

impl Render for ToolBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(format!("Hello, test!"))
    }
}