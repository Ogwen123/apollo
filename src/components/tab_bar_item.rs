use crate::state::State;
use crate::style::{Size, Style, StyleProvider};
use crate::widgets::core::button::{Button, TextPosition};
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::{Colour, Direction};
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, BorrowAppContext, Context, InteractiveElement, IntoElement, MouseButton,
    ParentElement, Render, RenderOnce, Styled, Window, div, px, rgb, rgba,
};

#[derive(Clone)]
pub struct TabBarItem {
    pub name: String,
    pub project_id: u32,
    pub active: bool,
}

impl Render for TabBarItem {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let id = self.project_id;

        div()
            .flex()
            .flex_row()
            .text_sm()
            .justify_between()
            .items_center()
            .min_w(px(100f32))
            .max_w(px(300f32))
            .h(cx.style().tabbar.height.get())
            .pl(px(4.0))
            .child(self.name.clone())
            .when(self.active, |_self| {
                _self
                    .border_b_4()
                    .border_color(&cx.style().secondary_colour)
            })
            .when_else(
                self.active,
                |_self| _self.bg(&cx.style().tabbar.active_colour),
                |_self| _self.bg(&cx.style().bg_colour),
            )
            .when_else(
                self.active,
                |_self| _self.hover(|style| style.bg(&cx.style().tabbar.active_colour)),
                |_self| _self.hover(|style| style.bg(&cx.style().tabbar.hover_colour)),
            )
            .on_mouse_down(MouseButton::Left, move |e, window, _cx| {
                _cx.update_global::<State, ()>(|global, _| {
                    global.set_active_project(id);
                    window.refresh()
                })
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .h_full()
                    .items_center()
                    .child(
                        Button::new()
                            .text(String::from("x"))
                            .text_colour(&cx.style().text_colour)
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .w(Size::Px(20.0))
                            .h(Size::Px(20.0))
                            .mx(cx.style().margin)
                            .colour(Colour::Rgba(0x00000000))
                            .hover_colour(Colour::Rgba(0xffffff22))
                            .rounding_all(Size::Px(100.0))
                            .on_click(move |e, window, _cx| {
                                _cx.update_global::<State, ()>(|global, _| {
                                    global.remove_project(id);
                                    println!("removed");
                                    window.refresh()
                                })
                            })
                            .render(window, cx),
                    )
                    .child(
                        Divider::new()
                            .colour(&cx.style().separator_colour)
                            .thickness(1.0)
                            .direction(Direction::Vertical)
                            .render(window, cx),
                    ),
            )
    }
}
