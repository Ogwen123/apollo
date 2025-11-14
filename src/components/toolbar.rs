use crate::state::{Project, State, StateProvider};
use crate::style::{Style, StyleProvider};
use crate::widgets::styling::{Colour, Size};

use crate::widgets::core::button::button::{Button, ContentPosition};
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::icon::Icons;
use crate::widgets::core::modal::{Modal, ModalButtonOptions};
use crate::{AsyncAlertHandler, ModalHelper};
use gpui::prelude::FluentBuilder;
use gpui::{
    AppContext, AsyncApp, Context, Div, DragMoveEvent, InteractiveElement, IntoElement,
    MouseButton, MouseDownEvent, ParentElement, PathPromptOptions, Render, Styled, Window, div, px,
    rgba,
};
use gpui::{BorrowAppContext, RenderOnce};
use std::env;
use zed_util::ResultExt;

const BUTTON_HOVER_COLOUR: u32 = 0xffffff22;

pub struct ToolBar {}
// cx.style().toolbar.bg_colour.get()
impl Render for ToolBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .h(cx.style().toolbar.height.get())
            .bg(&cx.style().toolbar.bg_colour)
            .items_center()
            .text_color(&cx.style().text_colour)
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .h_full()
                    .bg(&cx.style().toolbar.bg_colour)
                    .child(div()
                        .text_xl()
                        .px(px(10.0))
                        .child("Apollo".to_string()))
                    .child(Button::new("open-projects-button")
                        .text(String::from("Open Project"))
                        .text_colour(&cx.style().text_colour)
                        .justify_content(ContentPosition::Centre)
                        .align_text(ContentPosition::Centre)
                        .w(Size::Px(100.0))
                        .h(cx.style().toolbar.button_height)
                        .mx(cx.style().margin)
                        .colour(&cx.style().toolbar.bg_colour)
                        .hover_colour(Colour::Rgba(BUTTON_HOVER_COLOUR))
                        .rounding_all(cx.style().rounding)
                        .on_click(|_e, _window, _cx| {
                            // Everything inside is owned/moved
                            let options = PathPromptOptions {
                                files: false,
                                directories: true,
                                multiple: true,
                                prompt: None
                            };

                            let rec = _cx.prompt_for_paths(options);

                            _cx.spawn(
                                async move |__cx| match rec.await.anyhow().and_then(|res| res) {
                                    Ok(res) => {
                                        match res {
                                            Some(path) => {
                                                __cx.update(|___cx| {
                                                    let res = ___cx.has_global::<State>();

                                                    if res {
                                                        let _ = ___cx.update_global::<State, ()>(|global, _| {
                                                            path.iter().for_each(|x|global.add_project_by_path(x.clone()));
                                                        });
                                                    } else {
                                                        println!("No global state set")
                                                    }

                                                })
                                                    .ok();
                                            },
                                            None => {
                                                __cx.alert_error("Could not open this path", Some(5000.0));
                                                println!("No path was found")
                                            }
                                        }

                                    }
                                    Err(err) => {
                                        // TODO: add proper error handling once implemented
                                        println!("The following error occurred when opening new folder");
                                        println!("{}", err)
                                    }
                                },
                            )
                                .detach();
                        }).render(window, cx)
                    )
                    .child(Button::new("about-button")
                        .text(String::from("About"))
                        .text_colour(&cx.style().text_colour)
                        .justify_content(ContentPosition::Centre)
                        .align_text(ContentPosition::Centre)
                        .w(Size::Px(60f32))
                        .h(cx.style().toolbar.button_height)
                        .mx(cx.style().margin)
                        .colour(&cx.style().toolbar.bg_colour)
                        .hover_colour(Colour::Rgba(BUTTON_HOVER_COLOUR))
                        .rounding_all(cx.style().rounding)
                        .on_click(|_e, _window, _cx| {
                            _window.open_modal(_cx, |modal, __window, __cx| {
                                modal
                                    .title("About")
                                    //.body(vec![format!("Version: {}", env!("CARGO_PKG_VERSION")), "Author: Owen Jones".to_string()])
                                    .body(div().children(vec![format!("Version: {}", env!("CARGO_PKG_VERSION")), "Author: Owen Jones".to_string()]))
                                    .rounding(__cx.style().rounding)
                                    .bg_colour(&__cx.style().bg_colour)
                                    .p(Size::Px(10.0))
                                    .accept_button_options(None)
                                    .cancel_button_options(Some(ModalButtonOptions {
                                        show: true,
                                        text: "Close".to_string(),
                                        colour: __cx.style().bg_colour.clone(),
                                        hover_colour: Some(Colour::Rgba(0xffffff22)),
                                        border_width: Size::Px(1.0),
                                        border_colour: Some(__cx.style().separator_colour.clone()),
                                        padding: Size::Px(50.0),
                                        rounding: __cx.style().rounding,
                                        on_click: None
                                    }.on_click(|e, ___window, ___cx| {
                                        ___window.close_modal(___cx)
                                    })))
                                    .on_close(|e, ___window, ___cx| {
                                        ___window.close_modal(___cx)
                                    })
                            }
                            )
                        }).render(window, cx)
                    )

            )
            .when(cx.state().csd , |_self|
                _self.child(
                    div()
                        .w(px(40.0))
                        .child( IconButton::new("quit-button")
                            .icon(Icons::Close)
                            .icon_colour(Colour::Rgb(0xffffff))
                            .justify_content(ContentPosition::Centre)
                            .align_text(ContentPosition::Centre)
                            .w(cx.style().toolbar.button_height) // make the button a circle
                            .h(cx.style().toolbar.button_height)
                            .icon_size(cx.style().toolbar.button_height * 0.75)
                            .mx(cx.style().margin.clone())
                            .colour(&cx.style().toolbar.bg_colour)
                            .hover_colour(Colour::Rgba(BUTTON_HOVER_COLOUR))
                            .rounding_all(Size::Px(100.0))
                            .on_click(|_e, _window, cx| {
                                cx.quit()
                            }).render(window, cx)
                        )
                )
            )
    }
}
