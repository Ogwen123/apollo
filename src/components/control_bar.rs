use crate::state::{Project, State, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::utils::logger::warning;
use crate::widgets::core::button::{Button, TextPosition};
use crate::widgets::core::divider::Divider;
use crate::widgets::styling::Direction;
use cargo_ptest::config::Config;
use cargo_ptest::run::run;
use gpui::prelude::FluentBuilder;
use gpui::{
    BorrowAppContext, Context, InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    RenderOnce, Styled, Window, div, px,
};
use std::env::set_current_dir;
use std::path::PathBuf;

pub struct ControlBar {}

impl Render for ControlBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let path = cx
            .state()
            .get_active_project()
            .unwrap_or(Project::default())
            .path_string();

        let split_path = path
            .split(&['/', '\\'][..])
            .map(|x| x.to_string())
            .filter(|x| x.len() > 0)
            .collect::<Vec<String>>();

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .text_sm()
                    .text_color(&cx.style().sub_text_colour)
                    .ml(cx.style().margin.get())
                    .py(px(2.0))
                    .children({
                        split_path.iter().enumerate().map(|(index, x)| {
                            div()
                                .flex()
                                .flex_row()
                                .child(
                                    div()
                                        .hover(|style| {
                                            style.bg(&cx.style().primary_colour).opacity(0.75)
                                        })
                                        .rounded(px(4.0))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            move |_e, _window, _cx| {
                                                let sp = _cx
                                                    .state()
                                                    .get_active_project()
                                                    .unwrap_or(Project::default())
                                                    .path_string()
                                                    .split(&['/', '\\'][..])
                                                    .map(|x| x.to_string())
                                                    .filter(|x| x.len() > 0)
                                                    .collect::<Vec<String>>();

                                                let mut buffer = PathBuf::from("/");

                                                for (loc, i) in sp.iter().enumerate() {
                                                    buffer = buffer.join(i);
                                                    if loc == index {
                                                        break;
                                                    }
                                                }
                                                println!("{:?}", buffer);
                                                _cx.open_with_system(buffer.as_path());
                                            },
                                        )
                                        .child(x.to_string()),
                                )
                                .when(index != split_path.len() - 1, |_self| _self.child(">"))
                        })
                    }),
            )
            .child(
                Divider::new()
                    .direction(Direction::Horizontal)
                    .colour(&cx.style().separator_colour)
                    .thickness(1.0)
                    .render(window, cx),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h(cx.style().controlbar.height.get())
                    .w_full()
                    .bg(&cx.style().controlbar.bg_colour)
                    .child(
                        Button::new()
                            .text("Run Tests")
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .colour(&cx.style().primary_colour)
                            .hover_colour(&cx.style().hover_primary_colour)
                            .text_size(Size::Px(15.0))
                            .text_colour(&cx.style().text_colour)
                            .on_click(|_e, _window, _cx| {
                                _cx.update_global::<State, ()>(|global, _cx| {
                                    global.status.running_tests = true;
                                });
                                _window.refresh();
                                _cx.spawn(async move |__cx| {
                                    let _ = set_current_dir(
                                        __cx.read_global::<State, PathBuf>(|global, ___cx| {
                                            global.get_active_project().unwrap().path
                                        })
                                        .unwrap(),
                                    );

                                    match run(
                                        Some(Config {
                                            debug: false,
                                            ..Default::default()
                                        }),
                                        Some(vec![
                                            "--no-fail-fast".to_string(),
                                            "--workspace".to_string(),
                                        ]),
                                    ) {
                                        Ok(res) => {
                                            let _ =
                                                __cx.update_global::<State, ()>(|global, ___cx| {
                                                    global.set_tests(global.active_project, res);
                                                    global.status.running_tests = false;
                                                });
                                        }
                                        Err(err) => {
                                            warning!("An error occurred when running tests");
                                            println!("{}", err)
                                        }
                                    }
                                })
                                .detach()
                            })
                            .render(window, cx),
                    )
                    .child(
                        Button::new()
                            .text("Clear Output")
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .colour(&cx.style().bg_colour)
                            .hover_colour(Colour::Rgba(0xffffff22))
                            .text_size(Size::Px(15.0))
                            .text_colour(&cx.style().text_colour)
                            .on_click(|_, _window, _cx| {
                                _cx.update_global::<State, ()>(|global, _cx| {
                                    global.clear_tests(global.active_project)
                                });
                                _window.refresh()
                            })
                            .render(window, cx),
                    )
                    .child(
                        Divider::new()
                            .colour(&cx.style().separator_colour)
                            .thickness(1.0)
                            .direction(Direction::Vertical)
                            .render(window, cx),
                    )
                    .child(
                        Button::new()
                            .text("Open File Location")
                            .justify_content(TextPosition::Centre)
                            .align_text(TextPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .pa(cx.style().padding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .w(Size::Px(140.0))
                            .colour(&cx.style().primary_colour)
                            .hover_colour(&cx.style().hover_primary_colour)
                            .text_size(Size::Px(15.0))
                            .text_colour(&cx.style().text_colour)
                            .on_click(|_, _window, _cx| {
                                if _cx.state().has_active_project() {
                                    _cx.open_with_system(
                                        _cx.state().get_active_project().unwrap().path.as_path(),
                                    )
                                }
                            })
                            .render(window, cx),
                    ),
            )
            .child(
                Divider::new()
                    .thickness(2.0)
                    .colour(&cx.style().separator_colour)
                    .direction(Direction::Horizontal)
                    .render(window, cx),
            )
    }
}
