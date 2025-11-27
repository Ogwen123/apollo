use crate::state::StateProvider;
use crate::style::StyleProvider;
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use cargo_ptest::parse::{GeneralTestType, ParsedTest, Status};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, InteractiveElement, IntoElement, ParentElement, Render, RenderOnce,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};

pub struct TestInfo {}

impl RenderOnce for TestInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_selected_test =
            cx.state().has_active_project() && cx.state().get_selected_test().is_some();

        if has_selected_test {
            let test = cx.state().get_selected_test().unwrap();
            div()
                .flex()
                .flex_col()
                .w_full()
                .h_full()
                .bg(&cx.style().secondary_bg_colour)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .w_full()
                        .h(px(30.0))
                        .justify_center()
                        .items_center()
                        .text_color(match test.status {
                            Status::Failed => &cx.style().failed_colour,
                            Status::Ignored => &cx.style().ignore_colour,
                            Status::Passed => &cx.style().passed_colour,
                        })
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .w({
                                    let letter_width = 30.0;

                                    px(match test.status {
                                        Status::Passed => letter_width * 6.0,
                                        Status::Ignored => letter_width * 7.0,
                                        Status::Failed => letter_width * 6.0,
                                    })
                                })
                                .justify_between()
                                .children(match test.status {
                                    Status::Passed => "PASSED".split(""),
                                    Status::Ignored => "IGNORED".split(""),
                                    Status::Failed => "FAILED".split(""),
                                }),
                        ),
                )
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Horizontal)
                        .margin(0.0)
                        .render(window, cx),
                )
                .child(
                    div()
                        .id("test-info-main")
                        .overflow_y_scroll()
                        .h_full()
                        .w_full()
                        .flex()
                        .flex_col()
                        .px(px(4.0))
                        .child(
                            div()
                                .grid()
                                .grid_cols(2)
                                .grid_rows(3)
                                .h(px(87.0))
                                //.gap(cx.style().test_info.grid_padding.def())
                                .border_color(&cx.style().separator_colour)
                                .child(
                                    div()
                                        .border_r(px(1.0))
                                        .border_b(px(1.0))
                                        .border_color(&cx.style().separator_colour)
                                        .child("Test Type"),
                                )
                                .child(
                                    div()
                                        .pl(cx.style().test_info.grid_padding.def())
                                        .border_b(px(1.0))
                                        .border_color(&cx.style().separator_colour)
                                        .child(match test.test_type {
                                            GeneralTestType::Normal => "Normal",
                                            GeneralTestType::Doc => "Doc Test",
                                        }),
                                )
                                .child(
                                    div()
                                        .border_r(px(1.0))
                                        .border_color(&cx.style().separator_colour)
                                        .child("Module Path"),
                                )
                                .child(
                                    div()
                                        .pl(cx.style().test_info.grid_padding.def())
                                        .child(test.module_path),
                                )
                                .child(
                                    div()
                                        .border_r(px(1.0))
                                        .border_t(px(1.0))
                                        .border_b(px(1.0))
                                        .border_color(&cx.style().separator_colour)
                                        .child("File Path"),
                                )
                                .child(
                                    div()
                                        .pl(cx.style().test_info.grid_padding.def())
                                        .border_t(px(1.0))
                                        .border_b(px(1.0))
                                        .border_color(&cx.style().separator_colour)
                                        .child(
                                            test.file_path.unwrap_or_else(|| "Unknown".to_string()),
                                        ),
                                ),
                        )
                        .when(test.status == Status::Failed, |_self| {
                            _self.child(
                                div()
                                    .bg(&cx.style().test_info.error_background)
                                    .text_color(&cx.style().test_info.error_foreground)
                                    .w_4_5()
                                    .rounded(cx.style().rounding.abs())
                                    .p(cx.style().test_info.block_padding.def())
                                    .when_else(
                                        test.error_reason.is_some(),
                                        |__self| __self.child(test.error_reason.unwrap()),
                                        |__self| __self.child("Unknown error reason"),
                                    ),
                            )
                        })
                        .when(
                            test.status == Status::Ignored && test.ignore_reason.is_some(),
                            |_self| {
                                _self.child(
                                    div()
                                        .bg(&cx.style().test_info.ignore_background)
                                        .text_color(&cx.style().test_info.ignore_foreground)
                                        .w_4_5()
                                        .rounded(cx.style().rounding.abs())
                                        .p(cx.style().padding.def())
                                        .child(test.ignore_reason.unwrap()),
                                )
                            },
                        ),
                )
        } else {
            div()
                .flex()
                .flex_row()
                .w_full()
                .h_full()
                .bg(&cx.style().secondary_bg_colour)
                .child("show pie chart of results here")
                .child(
                    Divider::new()
                        .thickness(1.0)
                        .colour(&cx.style().separator_colour)
                        .direction(Direction::Vertical)
                        .margin(5.0)
                        .render(window, cx),
                )
                .child("other data here")
        }
    }
}
