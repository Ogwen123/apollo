use crate::state::{Project, RunArgs, State, StateProvider};
use crate::style::{Colour, Size, StyleProvider};
use crate::utils::logger::warning;
use crate::widgets::core::button::button::{Button, ContentPosition};
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::divider::Divider;
use crate::widgets::core::icon::Icons;
use crate::widgets::core::modal::ModalButtonOptions;
use crate::widgets::styling::Direction;
use crate::{AlertHandler, AsyncAlertHandler, ModalHelper};
use cargo_ptest::config::Config;
use cargo_ptest::parse::ParsedTestGroup;
use cargo_ptest::run::{RunError, run};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, AppContext, AsyncApp, BorrowAppContext, Context, InteractiveElement, IntoElement,
    MouseButton, ParentElement, Render, RenderOnce, Styled, Task, Window, div, px,
};
use std::env::set_current_dir;
use std::path::PathBuf;

fn run_tests(
    dir: PathBuf,
    args: Vec<String>,
    cx: &AsyncApp,
) -> Task<Result<Vec<ParsedTestGroup>, RunError>> {
    cx.background_executor().spawn(async move {
        let _ = set_current_dir(dir);

        match run(
            Some(Config {
                debug: false,
                ..Default::default()
            }),
            Some(args),
        ) {
            Ok(res) => Ok(res),
            Err(err) => {
                warning!("An error occurred when running tests");
                Err(err)
            }
        }
    })
}

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
                        div()
                            .flex()
                            .flex_row()
                            .child(
                                Button::new("run-tests-button")
                                    .text("Run Tests")
                                    .justify_content(ContentPosition::Centre)
                                    .align_text(ContentPosition::Centre)
                                    .rounding((
                                        cx.style().rounding,
                                        0.0.into(),
                                        0.0.into(),
                                        cx.style().rounding,
                                    ))
                                    .h(cx.style().controlbar.button_height)
                                    .ml(cx.style().margin)
                                    .colour(&cx.style().primary_colour)
                                    .hover_colour(&cx.style().hover_primary_colour)
                                    .text_size(Size::Px(15.0))
                                    .text_colour(&cx.style().text_colour)
                                    .tooltip("Run the projects tests")
                                    .on_click(|_e, _window, _cx| {
                                        _cx.update_global::<State, ()>(|global, _cx| {
                                            global.status.running_tests = true;
                                        });
                                        _window.refresh();

                                        let dir =
                                            _cx.read_global::<State, PathBuf>(|global, ___cx| {
                                                global.get_active_project().unwrap().path
                                            });
                                        _cx.spawn(async |__cx| {
                                            let args: Vec<String> = __cx
                                                .read_global::<State, Vec<String>>(|global, _| {
                                                    global.run_args.clone().into()
                                                })
                                                .unwrap_or(RunArgs::default_vec());

                                            match run_tests(dir, args, __cx).await {
                                                Ok(res) => {
                                                    let _ = __cx.update_global::<State, ()>(
                                                        |global, ___cx| {
                                                            global.set_tests(
                                                                global.active_project,
                                                                res,
                                                            );
                                                            global.status.running_tests = false;
                                                        },
                                                    );
                                                }
                                                Err(err) => {
                                                    __cx.alert_error(
                                                        Some("cargo_ptest: RunError"),
                                                        format!(
                                                            "Could not run tests: {}",
                                                            err.error
                                                        ),
                                                        Some(5000.0),
                                                    );
                                                }
                                            };
                                            __cx.refresh()
                                        })
                                        .detach();
                                    })
                                    .render(window, cx),
                            )
                            .child(
                                Divider::new()
                                    .thickness(1.0)
                                    .colour(&cx.style().separator_colour)
                                    .direction(Direction::Vertical)
                                    .render(window, cx),
                            )
                            .child(
                                IconButton::new("run-settings-button")
                                    .icon(Icons::Settings)
                                    .justify_content(ContentPosition::Centre)
                                    .align_text(ContentPosition::Centre)
                                    .rounding((
                                        0.0.into(),
                                        cx.style().rounding,
                                        cx.style().rounding,
                                        0.0.into(),
                                    ))
                                    .h(cx.style().controlbar.button_height)
                                    .w(cx.style().controlbar.button_height)
                                    .mr(cx.style().margin)
                                    .colour(&cx.style().primary_colour)
                                    .hover_colour(&cx.style().hover_primary_colour)
                                    .icon_size(Size::Px(15.0))
                                    .icon_colour(&cx.style().text_colour)
                                    .tooltip("Edit run settings")
                                    .on_click(|_e, _window, _cx| {
                                        _window.open_modal(_cx, |modal, __window, __cx| {
                                            modal
                                                .title("Run Config")
                                                .body(
                                                    div()
                                                        .child("Lib")
                                                        .child("Bin")
                                                        .child("Docs")
                                                        .child("Workspace")
                                                        .child("No Fail Fast"),
                                                )
                                                .h(px(500.0))
                                                .w(px(500.0))
                                                .rounding(__cx.style().rounding)
                                                .bg_colour(&__cx.style().bg_colour)
                                                .p(Size::Px(10.0))
                                                .accept_button_options(None)
                                                .cancel_button_options(Some(
                                                    ModalButtonOptions {
                                                        show: true,
                                                        text: "Close".to_string(),
                                                        colour: __cx.style().bg_colour.clone(),
                                                        hover_colour: Some(Colour::Rgba(
                                                            0xffffff22,
                                                        )),
                                                        border_width: Size::Px(1.0),
                                                        border_colour: Some(
                                                            __cx.style().separator_colour.clone(),
                                                        ),
                                                        padding: Size::Px(50.0),
                                                        rounding: __cx.style().rounding,
                                                        on_click: None,
                                                    }
                                                    .on_click(|e, ___window, ___cx| {
                                                        ___window.close_modal(___cx)
                                                    }),
                                                ))
                                                .on_close(|e, ___window, ___cx| {
                                                    ___window.close_modal(___cx)
                                                })
                                        })
                                    })
                                    .render(window, cx),
                            ),
                    )
                    .child(
                        IconButton::new("clear-tests-button")
                            .icon(Icons::Trash)
                            .justify_content(ContentPosition::Centre)
                            .align_text(ContentPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .w(cx.style().controlbar.button_height)
                            .colour(&cx.style().bg_colour)
                            .hover_colour(Colour::Rgba(0xffffff22))
                            .icon_size(cx.style().controlbar.button_height * 0.75)
                            .icon_colour(&cx.style().text_colour)
                            .tooltip("Clear tests")
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
                        IconButton::new("open-folder-location-button")
                            .icon(Icons::OpenFolder)
                            .justify_content(ContentPosition::Centre)
                            .align_text(ContentPosition::Centre)
                            .rounding_all(cx.style().rounding)
                            .pa(cx.style().padding)
                            .mx(cx.style().margin)
                            .h(cx.style().controlbar.button_height)
                            .w(cx.style().controlbar.button_height)
                            .colour(Colour::Rgba(0x00000000))
                            .hover_colour(Colour::Rgba(0xffffff22))
                            .icon_size(cx.style().controlbar.button_height * 0.75)
                            .icon_colour(&cx.style().text_colour)
                            .tooltip("Open folder location")
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
