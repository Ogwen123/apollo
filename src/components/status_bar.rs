use crate::style::Style;
use gpui::TextAlign::Center;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgba};

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
            .bg(self.style.statusbar.bg_colour.get())
            .child("Status bar")
    }
}
