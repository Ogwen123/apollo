use crate::components::tab_bar_item::TabBarItem;
use crate::state::State;
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct TabBar {
    pub(crate) style: Style,
}

impl Render for TabBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.global::<State>().clone().open_projects.projects;

        if projects.len() == 0 {
            div()
        } else {
            div()
                .flex()
                .flex_row()
                .w_full()
                .h(self.style.tabbar.height.get())
                .items_center()
                .border_b_1()
                .border_color(&self.style.separator_colour)
                .children(projects.iter().enumerate().map(|(index, x)| {
                    cx.new(|_cx| TabBarItem {
                        style: self.style.clone(),
                        name: x.display_name(),
                        project_id: x.id,
                        active: _cx.global::<State>().active_project == x.id
                    })
                }))
        }
    }
}
