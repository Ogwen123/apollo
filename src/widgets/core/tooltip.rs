use gpui::{App, Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb};

pub struct SimpleTooltip {
    text: String,
}

impl Render for SimpleTooltip {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.text.clone())
    }
}

impl SimpleTooltip {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
