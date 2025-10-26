use crate::components::tab_bar_item::TabBarItem;
use crate::state::{State, StateProvider};
use crate::style::{Style, StyleProvider};
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct TabBar {}

impl Render for TabBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.state().clone().open_projects;

        div()
            .flex()
            .flex_row()
            .w_full()
            .h(cx.style().tabbar.height.get())
            .items_center()
            .border_b_1()
            .border_color(&cx.style().separator_colour)
            .children(projects.iter().enumerate().map(|(index, x)| {
                cx.new(|_cx| TabBarItem {
                    name: x.display_name(),
                    project_id: x.id,
                    active: _cx.global::<State>().active_project == x.id,
                })
            }))
    }
}
