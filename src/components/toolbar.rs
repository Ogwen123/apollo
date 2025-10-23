use crate::state::{Project, State};
use crate::style::{Style, StyleProvider};
use crate::widgets::styling::{Colour, Size};

use crate::ModalHelper;
use crate::widgets::core::button::{Button, TextPosition};
use crate::widgets::core::modal::{Modal, ModalButtonOptions};
use gpui::{
    AppContext, Context, IntoElement, MouseDownEvent, ParentElement, PathPromptOptions, Render,
    Styled, Window, div, px, rgba,
};
use gpui::{BorrowAppContext, RenderOnce};
use zed_util::ResultExt;

const BUTTON_HEIGHT: Size = Size::Px(30f32);
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
                    .child(Button::new()
                        .text(String::from("Open Project"))
                        .text_colour(&cx.style().text_colour)
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(Size::Px(100.0))
                        .h(BUTTON_HEIGHT)
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
                                async move |__cx| match rec.await.anyhow().and_then(|res| res){
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
                                                        // TODO: add proper error handling once implemented
                                                        println!("No global state set")
                                                    }

                                                })
                                                    .ok();
                                            },
                                            None => {
                                                // TODO: add proper error handling once implemented
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
                    .child(Button::new()
                        .text(String::from("About"))
                        .text_colour(&cx.style().text_colour)
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(Size::Px(60f32))
                        .h(BUTTON_HEIGHT)
                        .mx(cx.style().margin)
                        .colour(&cx.style().toolbar.bg_colour)
                        .hover_colour(Colour::Rgba(BUTTON_HOVER_COLOUR))
                        .rounding_all(cx.style().rounding)
                        .on_click(|_e, _window, _cx| {
                            _window.open_modal(_cx, Modal::new()
                                .title("About")
                                .body(vec![format!("Version: {}", env!("CARGO_PKG_VERSION")), "Author: Owen Jones".to_string()])
                                .rounding(_cx.style().rounding)
                                .bg_colour(&_cx.style().bg_colour)
                                .padding(Size::Px(10.0))
                                .accept_button_options(None)
                                .cancel_button_options(Some(ModalButtonOptions {
                                    show: true,
                                    text: "Close".to_string(),
                                    colour: _cx.style().bg_colour.clone(),
                                    hover_colour: Some(Colour::Rgba(0xffffff22)),
                                    border_width: Size::Px(1.0),
                                    border_colour: Some(_cx.style().separator_colour.clone()),
                                    padding: Size::Px(50.0),
                                    rounding: _cx.style().rounding,
                                    on_click: None
                                }.on_click(|e, __window, __cx| {
                                    __window.close_modal(__cx)
                                })))
                            )
                        }).render(window, cx)
                    )

            )
            .child(
                div()
                    .w(px(40.0))
                    .child( Button::new()
                        .text(String::from("X"))
                        .text_colour(&cx.style().text_colour)
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(BUTTON_HEIGHT) // make the button a circle
                        .h(BUTTON_HEIGHT)
                        .mx(cx.style().margin.clone())
                        .colour(&cx.style().toolbar.bg_colour)
                        .hover_colour(Colour::Rgba(BUTTON_HOVER_COLOUR))
                        .rounding_all(Size::Px(100.0))
                        .on_click(|_e, _window, cx| {
                            cx.quit()
                        }).render(window, cx)
                    )
            )
    }
}
