use gpui::{div, px, rgba, Context, IntoElement, ParentElement, Render, Styled, Window};
use gpui::TextAlign::Center;
use crate::style::Style;

pub struct StatusBar {
    pub style: Style,
}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .h(px(self.style.statusbar.height))
            .w_full()
            .text_xs()
            .bg(rgba(self.style.statusbar.bg_colour))
            .child("Status bar")
    }
}