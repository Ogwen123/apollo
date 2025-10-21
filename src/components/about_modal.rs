use crate::state::State;
use crate::widgets::core::modal::Modal;
use gpui::prelude::FluentBuilder;
use gpui::{App, Context, IntoElement, ParentElement, Render, Window, anchored, div, point, px, Element, RenderOnce};

pub struct AboutModal {}

impl Render for AboutModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        println!("{}", cx.global::<State>().modals.about);

        Modal::new(cx.global::<State>().modals.about).render(window, cx)
    }
}
