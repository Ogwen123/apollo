use crate::components::tab_bar::TabBar;
use crate::state::State;
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct Workspace {}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.global::<State>().clone().open_projects.projects;

        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .child(cx.new(|_| TabBar {}))
            .child("Workspace")
            .children(
                projects
                    .iter()
                    .map(|x| div().child(format!("{:?}", x.path))),
            )
    }
}
