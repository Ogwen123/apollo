use gpui::{anchored, div, point, px, App, Context, IntoElement, ParentElement, Render, Window};
use gpui::prelude::FluentBuilder;
use crate::state::State;

pub struct AboutModal {
}

impl Render for AboutModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("{}", cx.global::<State>().modals.about);
        if cx.global::<State>().modals.about {
            println!("huh");
            anchored()
                .position(point(window.viewport_size().width / 2.0, window.viewport_size().width / 2.0))
                .snap_to_window()
                .child("this is a modal")
        } else {
            anchored()
        }
    }
}