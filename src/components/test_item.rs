use crate::style::StyleProvider;
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use cargo_ptest::parse::{ParsedTest, Status};
use gpui::{App, Context, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div, px};

pub struct TestItem {
    pub test_data: ParsedTest,
}

impl Render for TestItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .child(
                        div()
                            .w(px(60.0))
                            .text_color(match self.test_data.status {
                                Status::Failed => &cx.style().failed_colour,
                                Status::Ignored => &cx.style().ignore_colour,
                                Status::Passed => &cx.style().passed_colour,
                            })
                            .child(self.test_data.status.to_string()),
                    )
                    .child(self.test_data.module_path.clone()),
            )
            .child(
                Divider::new()
                    .direction(Direction::Horizontal)
                    .thickness(1.0)
                    .colour(&cx.style().separator_colour)
                    .render(window, cx),
            )
    }
}
