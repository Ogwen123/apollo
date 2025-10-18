use crate::components::tab_bar::TabBar;
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};
use crate::state::State;

pub struct Workspace {
    pub style: Style,
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.global::<State>().clone().open_projects.projects;

        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .child(cx.new(|_| TabBar {
                style: self.style.clone(),
            }))
            .child("Workspace")
            .children(projects.iter().map(|x| div().child(format!("{:?}", x.path))))
    }
}
