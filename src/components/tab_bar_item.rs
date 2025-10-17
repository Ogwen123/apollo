use gpui::{div, px, rgba, AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window};
use crate::{margin, rounding};
use crate::style::Style;
use crate::widgets::core::button::{Button, TextPosition};

pub struct TabBarItem {
    pub style: Style
}

impl Render for TabBarItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .min_w(px(100f32))
            .max_w(px(200f32))
            .h(px(self.style.tabbar.height))
            .child(div().child("Text"))
            .hover(|style| style.bg(rgba(self.style.tabbar.hover_colour)))
            .child(cx.new(|_| Button {
                text: String::from("x"),
                text_colour: rgba(self.style.text_colour),
                justify_content: TextPosition::Centre,
                align_text: TextPosition::Centre,
                width: 20f32,
                height: 20f32,
                margin: margin!(self.style.margin, 0.0),
                colour: rgba(0x00000000),
                rounding: rounding!(self.style.rounding),
                ..Default::default()
            }))
    }
}