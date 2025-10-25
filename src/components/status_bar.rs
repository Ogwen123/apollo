use crate::state::StateProvider;
use crate::style::StyleProvider;
use gpui::prelude::FluentBuilder;
use gpui::{Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct StatusBar {}

impl Render for StatusBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .h(cx.style().statusbar.height.get())
            .w_full()
            .text_xs()
            .bg(&cx.style().statusbar.bg_colour)
            .when(cx.state().status.running_tests, |_self| {
                _self.child("Running tests")
            })
    }
}
