mod components;
mod widgets;
mod style;
mod state;

use std::path::PathBuf;
use crate::components::toolbar::ToolBar;
use crate::widgets::utils::make_rgba;
use crate::style::Style;
use gpui::{App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div, prelude::*, px, rgb, size, Entity, EventEmitter};
use crate::components::main_panel::MainPanel;
use crate::components::status_bar::StatusBar;
use crate::state::{OpenProjects, State};

struct Base {
    style: Style,
    state: Entity<State>
}

impl Render for Base {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(make_rgba(self.style.bg_colour))
            .items_center()
            .text_color(make_rgba(self.style.text_colour))
            .child(_cx.new(|_| ToolBar {
                style: self.style.clone(),
                state: self.state.clone()
            }))
            .child(_cx.new(|_| MainPanel {
                style: self.style.clone(),
            }))
            .child(_cx.new(|_| StatusBar {
                style: self.style.clone(),
            }))
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let state = cx.new(|_cx| State {
            open_projects: OpenProjects::new(),
        });

        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        let window_options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            is_resizable: true,
            ..Default::default()
        };

        cx.open_window(window_options, |_, cx| {
            cx.new(|_| Base {
                style: Default::default(),
                state
            })
        })
        .unwrap();
    });
}
