use gpui::{div, px, rgb, rgba, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::core::button::{Button, TextPosition};
use crate::core::utils::make_rgba;
use crate::style::{Style};

pub struct ToolBar {
    pub style: Style
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
            .child("Apollo".to_string())
            .child(cx.new(|_| Button {
                text: String::from("test"),
                text_colour: self.style.text_colour,
                justify_content: TextPosition::Centre,
                align_text: TextPosition::Centre,
                width: 40f32,
                height: 26f32,
                colour: self.style.toolbar.bg_colour,
                hover_colour: Some(0xffffff22),
                rounding: 5.0,
                ..Default::default()
            }))
    }
}