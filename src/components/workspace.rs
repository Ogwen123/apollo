use crate::components::control_bar::ControlBar;
use crate::components::tab_bar::TabBar;
use crate::state::{State, StateProvider};
use gpui::prelude::FluentBuilder;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div};

pub struct Workspace {}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.state().clone().open_projects.projects;

        div()
            .flex()
            .flex_col()
            .h_full()
            .w_full()
            .when(projects.len() > 0, |_self| {
                _self.child(cx.new(|_| TabBar {}))
            })
            .when(projects.len() > 0, |_self| {
                _self.child(cx.new(|_| ControlBar {}))
            })
            .when(projects.len() == 0, |_self| {
                _self.child("Open a project to start")
            })
            .children(
                projects
                    .iter()
                    .map(|x| div().child(format!("{:?}", x.path))),
            )
    }
}
