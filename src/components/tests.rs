use crate::state::{Project, StateProvider};
use crate::style::StyleProvider;
use gpui::prelude::FluentBuilder;
use gpui::{
    Context, IntoElement, ParentElement, Render, SharedString, Styled, TextOverflow, Window, div,
    px,
};

pub struct Tests {}

impl Render for Tests {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tests_option = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .tests;
        div().h_full().flex().w_full().when_else(
            tests_option.is_some(),
            |_self| {
                let tests = tests_option.unwrap();
                _self.child(div().child(format!("there are {} tests", tests.len())))
            },
            |_self| {
                _self.child(
                    div()
                        .flex()
                        .h_full()
                        .w_full()
                        .justify_center()
                        .items_center()
                        .text_size(px(30.0))
                        .text_color(&cx.style().sub_text_colour)
                        .text_overflow(TextOverflow::Truncate(SharedString::new("...")))
                        .child("Click the \"Run Tests\" button for the tests to appear here."),
                )
            },
        )
    }
}
