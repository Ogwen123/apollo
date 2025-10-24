use crate::state::{Project, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::widgets::core::button::{Button, TextPosition};
use gpui::{Context, IntoElement, ParentElement, Render, RenderOnce, Styled, Window, div, px};

pub struct ControlBar {}

impl Render for ControlBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let path = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .path_string();

        let split_path = path
            .split(&['/', '\\'][..])
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .text_sm()
                    .text_color(&cx.style().sub_text_colour)
                    .ml(cx.style().margin.get())
                    .children({
                        split_path.iter().enumerate().map(|(index, x)| {
                            div().child(
                                x.to_string()
                                    + if index == split_path.len() - 1 {
                                        ""
                                    } else {
                                        " > "
                                    },
                            )
                        })
                    }),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .border_b_1()
                    .h(cx.style().controlbar.height.get())
                    .w_full()
                    .bg(&cx.style().controlbar.bg_colour)
                    .border_color(&cx.style().separator_colour)
                    .child(
                        Button::new()
                            .text("Run Tests")
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .pa(cx.style().padding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .colour(&cx.style().primary_colour)
                            .hover_colour(&cx.style().hover_primary_colour)
                            .text_size(Size::Px(15.0))
                            .text_colour(&cx.style().text_colour)
                            .on_click(|e, window, cx| println!("running tests"))
                            .render(window, cx),
                    )
                    .child(
                        Button::new()
                            .text("Clear Output")
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .pa(cx.style().padding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .colour(&cx.style().bg_colour)
                            .hover_colour(Colour::Rgba(0xffffff22))
                            .text_size(Size::Px(15.0))
                            .text_colour(&cx.style().text_colour)
                            .on_click(|e, window, cx| println!("clearing output"))
                            .render(window, cx),
                    ),
            )
    }
}
