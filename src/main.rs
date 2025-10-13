mod components;

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, Context, SharedString, Window,
    WindowBounds, WindowOptions,
};
use crate::components::toolbar::ToolBar;

struct Base {
    text: SharedString,
}

impl Render for Base {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_3()
            .bg(rgb(0x505050))
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(_cx.new(|_| ToolBar {}))
    }
}

fn main() {
    let window_options = WindowOptions {
        window_bounds: None,
        is_resizable: true,
        ..Default::default()
    };

    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);

        cx.open_window(
            window_options,
            |_, cx| {
                cx.new(|_| Base {
                    text: "World".into(),
                })
            },
        )
            .unwrap();
    });
}