use crate::components::tab_bar_item::TabBarItem;
use crate::style::Style;
use gpui::{
    AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgba,
};

pub struct TabBar {
    pub(crate) style: Style,
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
            .border_color(self.style.separator_colour.get())
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone(),
                name: "test 1".to_string(),
                project_id: 1,
                active: true,
            }))
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone(),
                name: "test 2".to_string(),
                project_id: 2,
                active: false,
            }))
            .child(cx.new(|_| TabBarItem {
                style: self.style.clone(),
                name: "test 3".to_string(),
                project_id: 3,
                active: false,
            }))
    }
}
