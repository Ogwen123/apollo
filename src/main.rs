mod components;
mod core;
mod style;

use crate::components::toolbar::ToolBar;
use crate::core::utils::make_rgba;
use crate::style::Style;
use gpui::{
    App, Application, Bounds, Context, SharedString, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
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
            .bg(make_rgba(self.style.bg_colour))
            .items_center()
            .text_color(make_rgba(self.style.text_colour))
            .child(_cx.new(|_| ToolBar {
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
            cx.new(|_| Base {
                style: Default::default(),
            })
        })
        .unwrap();
    });
}
