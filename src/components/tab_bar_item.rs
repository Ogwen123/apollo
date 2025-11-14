use crate::state::State;
use crate::style::{Size, Style, StyleProvider};
use crate::widgets::core::button::button::{Button, ContentPosition};
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::divider::Divider;
use crate::widgets::core::icon::Icons;
use crate::widgets::styling::{Colour, Direction};
use gpui::prelude::FluentBuilder;
use gpui::{AppContext, BorrowAppContext, Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render, RenderOnce, Styled, Window, div, px, rgb, rgba, FontWeight};

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
            .font_weight(FontWeight(900.0))
            .child(self.name.clone())
            .border_b_4()
            .when_else(
                self.active,
                |_self| _self.border_color(&cx.style().tabbar.active_colour),
                |_self| _self.border_color(rgba(0xffffff22)),
            )
            .when_else(
                self.active,
                |_self| _self.bg(&cx.style().tabbar.active_colour.opacity(0xbb)),
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
                        IconButton::new("close-button")
                            .icon(Icons::Close)
                            .icon_colour(&cx.style().text_colour)
                            .justify_content(ContentPosition::Centre)
                            .align_text(ContentPosition::Centre)
                            .w(Size::Px(22.0))
                            .h(Size::Px(22.0))
                            .icon_size(Size::Px(14.0))
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
