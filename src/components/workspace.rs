use crate::components::control_bar::ControlBar;
use crate::components::tab_bar::TabBar;
use crate::components::tests::Tests;
use crate::state::{State, StateProvider};
use crate::style::StyleProvider;
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, Context, InteractiveElement, IntoElement, ParentElement, Render,
    StatefulInteractiveElement, Styled, Window, div, px, rgb,
};

pub struct Workspace {}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let projects = cx.state().clone().open_projects;

        div()
            .id("workspace")
            .overflow_scroll()
            .flex()
            .flex_col()
            .size_full()
            .when(projects.len() > 0, |_self| {
                _self
                    .child(cx.new(|_| TabBar {}))
                    .child(cx.new(|_| ControlBar {}))
                    .child(cx.new(|_| Tests {}))
            })
            .when(projects.len() == 0, |_self| {
                _self.child(
                    div()
                        .flex()
                        .h_full()
                        .w_full()
                        .justify_center()
                        .items_center()
                        .text_size(px(50.0))
                        .text_color(&cx.style().sub_text_colour)
                        .child("Open a project to start"),
                )
            })
    }
}
