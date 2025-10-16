use crate::core::button::{Button, TextPosition};
use crate::core::utils::make_rgba;
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb, rgba, percentage};
use crate::padding;

pub struct ToolBar {
    pub style: Style,
}

impl Render for ToolBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(self.style.toolbar.height))
            .bg(make_rgba(self.style.toolbar.bg_colour))
            .items_center()
            .text_color(make_rgba(self.style.text_colour))
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .child(div()
                        .pr(px(self.style.padding))
                        .child("Apollo".to_string()))
                    .child(cx.new(|_| Button {
                        text: String::from("Open Project"),
                        text_colour: self.style.text_colour,
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 80f32,
                        height: 26f32,
                        padding: padding!(self.style.padding, 0.0),
                        colour: self.style.toolbar.bg_colour,
                        hover_colour: Some(0xffffff22),
                        rounding: self.style.rounding,
                        ..Default::default()
                    }))
                    .child(cx.new(|_| Button {
                        text: String::from("About"),
                        text_colour: self.style.text_colour,
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 40f32,
                        height: 26f32,
                        padding: padding!(self.style.padding, 0.0),
                        colour: self.style.toolbar.bg_colour,
                        hover_colour: Some(0xffffff22),
                        rounding: self.style.rounding,
                        ..Default::default()
                    })),

            )
            .child(
                div()
                    .w_1_12()
                    .child(cx.new(|_| Button {
                        text: String::from("X"),
                        text_colour: self.style.text_colour,
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 40f32,
                        height: 26f32,
                        colour: self.style.toolbar.bg_colour,
                        hover_colour: Some(0xffffff22),
                        rounding: self.style.rounding,
                        on_click: |e, window, cx| {
                            cx.quit()
                        },
                        ..Default::default()
                    })),
            )
    }
}
