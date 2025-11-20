use crate::components::test_info::TestInfo;
use crate::components::test_list::TestList;
use crate::components::test_list_item::TestListItem;
use crate::state::{Project, State, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::widgets::core::divider::Divider;
use crate::widgets::core::spinner::Spinner;
use crate::widgets::styling::Direction;
use cargo_ptest::parse::AggregateSummary;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, BorrowAppContext, Context, Element, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, RenderOnce, SharedString, StatefulInteractiveElement, Styled,
    TextOverflow, UniformListScrollHandle, Window, div, percentage, px, rgb,
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
        let position_side_by_side =
            <gpui::Pixels as Into<f32>>::into(window.viewport_size().width.into()) > 1000.0;
        let summary_line;

        if show_test {
            let summary = cx
                .state()
                .get_active_project()
                .unwrap()
                .tests
                .unwrap()
                .aggregate_summary();

            summary_line = div()
                .id("summary")
                .flex()
                .flex_row()
                .h(px(30.0))
                .hover(|style| style.bg(Colour::Rgba(0xffffff22)))
                .w_full()
                .border_b(px(2.0))
                .border_color(&cx.style().separator_colour)
                .on_mouse_down(MouseButton::Left, |_, _, cx| {
                    cx.update_global::<State, ()>(|global, _| global.unselect_test())
                })
                .child(summary.passed.to_string())
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().passed_colour)
                        .child("Passed"),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child(summary.failed.to_string())
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().failed_colour)
                        .child("Failed"),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child(summary.ignored.to_string())
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().ignore_colour)
                        .child("Ignored"),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child(summary.filtered.to_string())
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().ignore_colour)
                        .child("Filtered"),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child(summary.measured.to_string())
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().ignore_colour)
                        .child("Measured"),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child("Finished in")
                .child(
                    div()
                        .ml(px(4.0))
                        .text_color(&cx.style().doctest_colour)
                        .child(summary.time.to_string()),
                )
        } else {
            summary_line = div().id("empty-summary");
        }

        let test_list = div()
            .id("test-list-parent")
            .relative()
            .when_else(
                position_side_by_side,
                |_self| _self.max_w_1_3().h_full(),
                |_self| _self.w_full()./*h_auto().max_*/h(px(600.0)),
            )
            .overflow_y_scroll()
            .child(cx.new(|_| TestList {}));

        let test_info = div()
            .flex()
            .id("test-info-parent")
            .when_else(
                position_side_by_side,
                |_self| _self.h_full().flex_grow(),
                |_self| _self.w_full().flex_grow(),
            )
            .child(TestInfo {}.render(window, cx));

        let tests_display = div()
            .flex()
            .bg(rgb(0x00ff00))
            .w_full()
            .flex_grow()
            .when_else(
                position_side_by_side,
                |_self| _self.flex_row(),
                |_self| _self.flex_col(),
            )
            //.child(summary_line)
            .child(
                test_list, //.on_scroll_wheel(|e, _, _| println!("{:?}", e.delta)),
            )
            .child(
                Divider::new()
                    .thickness(2.0)
                    .colour(&cx.style().separator_colour)
                    .direction(Direction::Horizontal)
                    .render(window, cx),
            )
            .child(test_info);

        div()
            .flex()
            .flex_col()
            .bg(rgb(0xff0000))
            .h_full()
            .w_full()
            .when(show_test, |_self| _self.child(summary_line).child(tests_display))
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
