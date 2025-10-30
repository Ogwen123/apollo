mod components;
mod events;
mod state;
mod style;
mod utils;
mod widgets;

use crate::components::status_bar::StatusBar;
use crate::components::toolbar::ToolBar;
use crate::components::workspace::Workspace;
use crate::state::{State, StateProvider};
use crate::style::{GlobalStyle, Style, StyleProvider};
use crate::widgets::core::modal::Modal;
use gpui::{App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px, size, Task};
use std::sync::Arc;
use crate::utils::file::{load_state, save_state};

trait ModalHelper {
    fn open_modal(&mut self, cx: &mut App, modal: Modal);
    fn close_modal(&mut self, cx: &mut App);
}

impl ModalHelper for Window {
    fn open_modal(&mut self, cx: &mut App, modal: Modal) {
        let root = self
            .root::<Base>()
            .flatten()
            .expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.push(modal);
        });
        self.refresh()
    }

    fn close_modal(&mut self, cx: &mut App) {
        let root = self
            .root::<Base>()
            .flatten()
            .expect("Window root should be type Base");
        root.update(cx, |base, cx| {
            base.modals.pop();
        });
        self.refresh()
    }
}

struct Base {
    modals: Vec<Modal>,
}

impl Render for Base {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(&cx.style().bg_colour)
            .items_center()
            .text_color(&cx.style().text_colour)
            .child(cx.new(|_| ToolBar {}))
            .child(cx.new(|_| Workspace {}))
            .child(cx.new(|_| StatusBar {}))
            // Modals
            .children(self.modals.iter().map(|x| {
                cx.new(|_| Modal {
                    title: x.title.clone(),
                    body: x.body.clone(),
                    width: x.width,
                    height: x.height,
                    padding: x.padding,
                    rounding: x.rounding,
                    bg_colour: x.bg_colour.clone(),
                    accept_button: x.accept_button.clone(),
                    cancel_button: x.cancel_button.clone(),
                    top_offset: x.top_offset,
                    on_close: if x.on_close.is_none() {
                        None
                    } else {
                        None /*Some(Box::clone(&x.on_close.unwrap()))*/
                    },
                    backdrop_close: x.backdrop_close,
                })
            }))
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

        // load previous state from file
        let state = load_state();

        cx.set_global(state);
        cx.set_global(GlobalStyle(Arc::new(Style::default())));



        let _ = cx.on_app_quit(|_cx| {
            // save state to a file
            println!("saving state");
            save_state(_cx.state().clone());
            return Task::ready(())
        }).detach();

        cx.open_window(window_options, |_, cx| {


            cx.new(|_cx| Base { modals: Vec::new() })
        })
        .unwrap();
    });
}
