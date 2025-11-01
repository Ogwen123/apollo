use crate::components::test_item::TestItem;
use crate::state::{Project, StateProvider};
use crate::style::StyleProvider;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Element, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    Styled, TextOverflow, Window, div, px,
};

pub struct Tests {}

impl Render for Tests {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tests_option = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .tests;
        let show_test = tests_option.is_some();
        div()
            .h_full()
            .flex()
            .w_full()
            .when(show_test, |_self| {
                let tests = tests_option.unwrap();
                _self.child(
                    div().children(
                        {
                            let mut elements: Vec<TestItem> = Vec::new();

                            for group in tests {
                                for test in group.tests {
                                    elements.push(TestItem { test_data: test })
                                }
                            }

                            elements
                        }
                        .into_iter()
                        .map(|mut x| cx.new(|_| x)),
                    ),
                )
            })
            .when(!show_test, |_self| {
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
            })
    }
}
