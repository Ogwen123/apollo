use crate::state::{State, StateProvider};
use crate::style::StyleProvider;
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use cargo_ptest::parse::{ParsedTest, Status};
use gpui::{
    App, BorrowAppContext, Context, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, RenderOnce, Styled, Window, div, px, rgba,
};

pub struct TestListItem {
    pub index: usize,
    pub test_data: ParsedTest,
}

impl Render for TestListItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let index = self.index.clone();
        div()
            .flex()
            .flex_col()
            .w_full()
            .h(px(30.0))
            .justify_center()
            .hover(|style| style.bg(rgba(0xffffff22)))
            .on_mouse_down(MouseButton::Left, move |e, _window, _cx| {
                _cx.update_global::<State, ()>(move |global, _| global.select_test(index))
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h_full()
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
