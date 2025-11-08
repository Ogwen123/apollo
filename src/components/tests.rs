use crate::components::test_info::TestInfo;
use crate::components::test_item::TestItem;
use crate::state::{Project, StateProvider};
use crate::style::{Size, StyleProvider};
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Element, IntoElement, ParentElement, Render, RenderOnce, SharedString,
    Styled, TextOverflow, Window, div, percentage, px, rgb,
};

pub struct Tests {}

impl Render for Tests {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tests_option = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .tests_linear();
        let show_test = tests_option.is_some();
        let horizontal_positioning =
            <gpui::Pixels as Into<f32>>::into(window.viewport_size().width.into()) > 1000.0;
        div()
            .h_full()
            .flex()
            .w_full()
            .when(show_test, |_self| {
                let tests = tests_option.unwrap();
                _self.child(
                    div()
                        .w_full()
                        .when_else(
                            horizontal_positioning,
                            |_self| _self.flex_row(),
                            |_self| _self.flex_col(),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .when_else(
                                    horizontal_positioning,
                                    |_self| _self.w_1_2().h_full(),
                                    |_self| _self.w_full().h_1_2(),
                                )
                                .children(
                                    {
                                        let mut elements: Vec<TestItem> = Vec::new();

                                        for test in tests {
                                            elements.push(TestItem { test_data: test })
                                        }

                                        elements
                                    }
                                    .into_iter()
                                    .map(|x| cx.new(|_| x)),
                                ),
                        )
                        .child(
                            div()
                                .when_else(
                                    horizontal_positioning,
                                    |_self| _self.w_1_2().h_full(),
                                    |_self| _self.w_full().h_1_2(),
                                )
                                .when_else(
                                    cx.state().has_active_project()
                                        && cx.state().get_selected_test().is_some(),
                                    |_self| {
                                        _self.child(
                                            TestInfo {
                                                test: cx.state().get_selected_test().unwrap(),
                                            }
                                            .render(window, cx),
                                        )
                                    },
                                    |_self| _self.child("Select a test to view it here"),
                                ),
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
