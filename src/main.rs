mod components;
mod events;
mod state;
mod style;
mod utils;
mod widgets;

use crate::components::status_bar::StatusBar;
use crate::components::toolbar::ToolBar;
use crate::components::workspace::Workspace;
use crate::state::{Modals, OpenProjects, State};
use crate::style::Style;
use crate::widgets::styling::{Colour, Size};
use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    size,
};
use crate::widgets::core::modal::Modal;

trait ModalHelper {
    fn open_modal(&mut self, cx: &mut App, modal: Modal);
    fn close_modal(&mut self, cx: &mut App, modal: Modal);
}

impl ModalHelper for Window {
    fn open_modal(&mut self, cx: &mut App, modal: Modal) {
        let root = self.root::<Base>().flatten().expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.push(modal);
        });
        self.refresh()
    }

    fn close_modal(&mut self, cx: &mut App, modal: Modal) {
        let root = self.root::<Base>().flatten().expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.pop();
        });
        self.refresh()
    }
}

struct Base {
    style: Style,
    modals: Vec<Modal>
}

impl Render for Base {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(&self.style.bg_colour)
            .items_center()
            .text_color(&self.style.text_colour)
            .child(cx.new(|_| ToolBar {
                style: self.style.clone(),
            }))
            .child(cx.new(|_| Workspace {
                style: self.style.clone(),
            }))
            .child(cx.new(|_| StatusBar {
                style: self.style.clone(),
            }))
            // Modals
            .children(self.modals.iter().map(|x| cx.new(|_| Modal {
                title: x.title.clone(),
                body: x.body.clone(),
                width: x.width,
                height: x.height,
                rounding: x.rounding,
                bg_colour: x.bg_colour.clone(),
                accept_button_colour: x.accept_button_colour.clone(),
                cancel_button_colour: x.cancel_button_colour.clone(),
                accept_text: x.accept_text.clone(),
                cancel_text: x.cancel_text.clone(),
                top_offset: x.top_offset,
                on_accept: x.on_accept.clone(),
                on_cancel: x.on_cancel.clone(),
                on_close: x.on_close.clone(),
                backdrop_close: x.backdrop_close
            })))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            is_resizable: true,
            ..Default::default()
        };

        let modals = Modals { about: false };

        cx.open_window(window_options, |_, cx| {
            let state = State {
                open_projects: OpenProjects::new(),
                active_project: 0,
                modals,
            };

            cx.set_global(state);

            cx.new(|_cx| Base {
                style: Default::default(),
                modals: Vec::new()
            })
        })
        .unwrap();
    });
}
