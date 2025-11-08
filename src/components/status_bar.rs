use crate::state::StateProvider;
use crate::style::StyleProvider;
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use gpui::prelude::FluentBuilder;
use gpui::{Context, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div, px};

pub struct StatusBar {}

impl Render for StatusBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("{}", cx.state().status.running_tests);
        div()
            .flex()
            .h(cx.style().statusbar.height.get())
            .w_full()
            .px(cx.style().padding.def())
            .text_xs()
            .bg(&cx.style().statusbar.bg_colour)
            .child(div().w(px(80.0)).when_else(
                cx.state().status.running_tests,
                |_self| _self.child("Running tests"),
                |_self| _self.child("Idle"),
            ))
            .child(
                Divider::new()
                    .colour(&cx.style().separator_colour)
                    .thickness(1.0)
                    .direction(Direction::Vertical)
                    .margin(4.0)
                    .render(window, cx),
            )
            .child(div().w(px(80.0)).when_else(
                cx.state().get_active_project().is_some()
                    && cx.state().get_active_project().unwrap().tests.is_some(),
                |_self| {
                    _self.child(format!(
                        "{} Tests Run",
                        cx.state()
                            .get_active_project()
                            .unwrap()
                            .tests_linear()
                            .unwrap()
                            .len()
                    ))
                },
                |_self| _self.child("No Tests Run"),
            ))
    }
}
