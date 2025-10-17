use gpui::{div, px, rgba, AppContext, Context, IntoElement, ParentElement, Render, Styled, Window};
use crate::components::tab_bar_item::TabBarItem;
use crate::style::Style;

pub struct TabBar {
    pub(crate) style: Style
}

impl Render for TabBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .h(px(self.style.tabbar.height))
            .items_center()
            .border_b_1()
            .border_color(rgba(self.style.separator_colour))
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone()
            }))
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone()
            }))
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone()
            }))
    }
}