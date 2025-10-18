use crate::state::{Project, State};
use crate::style::Style;
use crate::widgets::core::button::{Button, TextPosition};
use crate::{margin, rounding};
use gpui::BorrowAppContext;
use gpui::{
    AppContext, Context, IntoElement,
    ParentElement, PathPromptOptions, Render, Styled, Window, div, px,
    rgba,
};
use zed_util::ResultExt;

const BUTTON_HEIGHT: f32 = 30f32;
const BUTTON_HOVER_COLOUR: u32 = 0xffffff22;

pub struct ToolBar {
    pub style: Style,
}

impl Render for ToolBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(self.style.toolbar.height))
            .bg(self.style.toolbar.bg_colour.get())
            .items_center()
            .text_color(self.style.text_colour.get())
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(div()
                        .text_xl()
                        .px(px(10.0))
                        .child("Apollo".to_string()))
                    .child(cx.new(|_| Button::new()
                        .text(String::from("Open Project"))
                        .text_colour(self.style.text_colour.get())
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(100f32)
                        .h(BUTTON_HEIGHT)
                        .m(margin!(self.style.margin, 0.0))
                        .colour(self.style.toolbar.bg_colour.get())
                        .hover_colour(rgba(BUTTON_HOVER_COLOUR))
                        .rounding(rounding!(self.style.rounding))
                        .on_click(|_e, _window, _cx| {
                            // Everything inside is owned/moved
                            let options = PathPromptOptions {
                                files: false,
                                directories: true,
                                multiple: false,
                                prompt: None
                            };

                            let rec = _cx.prompt_for_paths(options);

                            _cx.spawn(
                                async move |__cx| match rec.await.anyhow().and_then(|res| res){
                                    Ok(res) => {
                                        match res {
                                            Some(path) => {
                                                println!("{:?}", path);
                                                __cx.update(|___cx| {
                                                    let res = ___cx.has_global::<State>();
                                                    
                                                    if res {
                                                        let _ = ___cx.update_global::<State, ()>(|global, _| {
                                                            global.open_projects.projects.push(Project::new(path[0].clone()));
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
                        })
                    ))
                    .child(cx.new(|_| Button::new()
                        .text(String::from("About"))
                        .text_colour(self.style.text_colour.get())
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(60f32)
                        .h(BUTTON_HEIGHT)
                        .m(margin!(self.style.margin, 0.0))
                        .colour(self.style.toolbar.bg_colour.get())
                        .hover_colour(rgba(BUTTON_HOVER_COLOUR))
                        .rounding(rounding!(self.style.rounding))
                        .on_click(|_e, _window, _cx| {
                            _cx.read_global::<State, ()>(|global, _| {
                                println!("{}", global)
                            })
                        })
                    ))

            )
            .child(
                div()
                    .w(px(40.0))
                    .child(cx.new(|_| Button::new()
                        .text(String::from("X"))
                        .text_colour(self.style.text_colour.get())
                        .justify_content(TextPosition::Centre)
                        .align_text(TextPosition::Centre)
                        .w(BUTTON_HEIGHT) // make the button a circle
                        .h(BUTTON_HEIGHT)
                        .m(margin!(self.style.margin, 0.0))
                        .colour(self.style.toolbar.bg_colour.get())
                        .hover_colour(rgba(BUTTON_HOVER_COLOUR))
                        .rounding(rounding!(100.0))
                        .on_click(|_e, _window, cx| {
                            cx.quit()
                        })
                    ))
            )
    }
}
