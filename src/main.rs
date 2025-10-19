mod components;
mod events;
mod state;
mod style;
mod utils;
mod widgets;

use crate::components::status_bar::StatusBar;
use crate::components::toolbar::ToolBar;
use crate::components::workspace::Workspace;
use crate::state::{OpenProjects, State};
use crate::style::Style;
use gpui::{
    App, Application, Bounds, Context, Entity, EventEmitter, SharedString, Window, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, rgba, size,
};

struct Base {
    style: Style,
}

impl Render for Base {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(self.style.bg_colour.get())
            .items_center()
            .text_color(self.style.text_colour.get())
            .child(_cx.new(|_| ToolBar {
                style: self.style.clone(),
            }))
            .child(_cx.new(|_| Workspace {
                style: self.style.clone(),
            }))
            .child(_cx.new(|_| StatusBar {
                style: self.style.clone(),
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

        cx.open_window(window_options, |_, cx| {
            let state = State {
                open_projects: OpenProjects::new(),
            };

            cx.set_global(state);

            cx.new(|_cx| Base {
                style: Default::default(),
            })
        })
        .unwrap();
    });
}
