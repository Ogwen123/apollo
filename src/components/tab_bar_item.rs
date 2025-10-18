use crate::state::State;
use crate::style::Style;
use crate::utils::utils::get_starting_path;
use crate::widgets::core::button::{Button, TextPosition};
use crate::{margin, rounding};
use gpui::{
    AppContext, AsyncApp, BorrowAppContext, Context, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, Window, div, px, rgb, rgba,
};

pub struct TabBarItem {
    pub style: Style,
    pub name: String,
    pub project_id: u32,
    pub active: bool,
}

impl Render for TabBarItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg = if self.active {
            self.style.tabbar.active_colour.get()
        } else {
            self.style.bg_colour.get()
        };
        div()
            .flex()
            .flex_row()
            .justify_between()
            .items_center()
            .min_w(px(100f32))
            .max_w(px(200f32))
            .h(px(self.style.tabbar.height))
            .p(px(4.0))
            .child(div().child("Text"))
            .bg(bg)
            .hover(|style| {
                if self.active {
                    style.bg(bg)
                } else {
                    style.bg(self.style.tabbar.hover_colour.get())
                }
            })
            .child(cx.new(|_| {
                Button::new()
                    .text(String::from("x"))
                    .text_colour(self.style.text_colour.get())
                    .justify_content(TextPosition::Centre)
                    .align_text(TextPosition::Centre)
                    .w(20f32)
                    .h(20f32)
                    .m(margin!(self.style.margin, 0.0))
                    .colour(self.style.bg_colour.get())
                    .hover_colour(rgba(0x00000000))
                    .rounding(rounding!(self.style.rounding))
                    .on_click(|e, window, _cx| {})
            }))
    }
}
