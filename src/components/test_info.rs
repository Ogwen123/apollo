use crate::style::StyleProvider;
use crate::widgets::core::icon::{Icon, Icons};
use cargo_ptest::parse::{ParsedTest, Status};
use gpui::{
    App, AppContext, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div, px,
};
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;

pub struct TestInfo {
    pub test: ParsedTest,
}

impl RenderOnce for TestInfo {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .h_full()
            .items_end()
            .child(div().w(px(105.0)).child(
                cx.new(|_cx| {
                    Icon::new()
                        .icon(match self.test.status {
                            Status::Passed => Icons::Check,
                            Status::Ignored => Icons::AlertCircle,
                            Status::Failed => Icons::Ban,
                        })
                        .colour(match self.test.status {
                            Status::Failed => &_cx.style().failed_colour,
                            Status::Ignored => &_cx.style().ignore_colour,
                            Status::Passed => &_cx.style().passed_colour,
                        })
                        .size(px(100.0))
                })
            ))
            .child(
                Divider::new()
                    .thickness(2.0)
                    .colour(&cx.style().separator_colour)
                    .direction(Direction::Vertical)
                    .margin(8.0)
                    .render(window, cx)
            )
            .child(div().h_full().w_full().child(self.test.module_path))
    }
}
