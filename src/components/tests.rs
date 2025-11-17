use crate::components::test_info::TestInfo;
use crate::components::test_list::TestList;
use crate::components::test_list_item::TestListItem;
use crate::state::{Project, StateProvider};
use crate::style::{Size, StyleProvider};
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, Element, InteractiveElement, IntoElement, ParentElement, Render,
    RenderOnce, SharedString, StatefulInteractiveElement, Styled, TextOverflow,
    UniformListScrollHandle, Window, div, percentage, px, rgb,
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
                _self.child(
                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        .h_full()
                        .when_else(
                            horizontal_positioning,
                            |_self| _self.flex_row(),
                            |_self| _self.flex_col(),
                        )
                        .child(
                            div()
                                .id("test-list-parent")
                                .flex()
                                .when_else(
                                    horizontal_positioning,
                                    |_self| _self.w_1_3().h_full(),
                                    |_self| _self.w_full().h_2_3(),
                                )
                                .overflow_scroll()
                                .child(cx.new(|_| TestList {})),
                            //.on_scroll_wheel(|e, _, _| println!("{:?}", e.delta)),
                        )
                        .child(
                            Divider::new()
                                .thickness(2.0)
                                .colour(&cx.style().separator_colour)
                                .direction(Direction::Horizontal)
                                .render(window, cx),
                        )
                        .child(
                            div()
                                .id("test-info-parent")
                                .overflow_scroll()
                                .when_else(
                                    horizontal_positioning,
                                    |_self| _self.w_2_3().h_full(),
                                    |_self| _self.w_full().h_1_3(),
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
                // this is needed over a when_else so both closures don't borrow cx
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
