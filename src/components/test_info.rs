use crate::state::StateProvider;
use crate::style::StyleProvider;
use crate::widgets::core::divider::Divider;
use crate::widgets::core::icon::{Icon, Icons};
use crate::widgets::styling::Direction;
use cargo_ptest::parse::{ParsedTest, Status};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div, px, rgb,
};

pub struct TestInfo {}

impl RenderOnce for TestInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_selected_test =
            cx.state().has_active_project() && cx.state().get_selected_test().is_some();

        div()
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .bg(&cx.style().secondary_bg_colour)
            .when(has_selected_test, |_self| {
                let test = cx.state().get_selected_test().unwrap();

                _self
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(105.0))
                            .h_full()
                            .justify_center()
                            .child(cx.new(|_cx| {
                                Icon::new()
                                    .icon(match test.status {
                                        Status::Passed => Icons::Check,
                                        Status::Ignored => Icons::AlertCircle,
                                        Status::Failed => Icons::Ban,
                                    })
                                    .colour(match test.status {
                                        Status::Failed => &_cx.style().failed_colour,
                                        Status::Ignored => &_cx.style().ignore_colour,
                                        Status::Passed => &_cx.style().passed_colour,
                                    })
                                    .size(px(100.0))
                            })),
                    )
                    .child(
                        Divider::new()
                            .thickness(2.0)
                            .colour(&cx.style().separator_colour)
                            .direction(Direction::Vertical)
                            .margin(8.0)
                            .render(window, cx),
                    )
                    .child(div().h_full().w_full().child(test.module_path))
            })
            .when(!has_selected_test, |_self| {
                _self
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
            })
    }
}
