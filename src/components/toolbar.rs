use std::path::PathBuf;
use crate::widgets::core::button::{Button, TextPosition};
use crate::style::Style;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, div, px, rgb, rgba, percentage, App, AsyncApp, AsyncWindowContext, EventEmitter, Entity};
use crate::{margin, padding, rounding, OpenProjects};
use rfd::AsyncFileDialog;
use crate::state::State;

const BUTTON_HEIGHT: f32 = 30f32;
const BUTTON_HOVER_COLOUR: u32 = 0xffffff22;

pub struct ToolBar {
    pub style: Style
}

impl Render for ToolBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .w_full()
            .h(px(self.style.toolbar.height))
            .bg(rgba(self.style.toolbar.bg_colour))
            .items_center()
            .text_color(rgba(self.style.text_colour))
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
                    .child(cx.new(|_| Button {
                        text: String::from("Open Project"),
                        text_colour: rgba(self.style.text_colour),
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 80f32,
                        height: BUTTON_HEIGHT,
                        margin: margin!(self.style.margin, 0.0),
                        colour: rgba(self.style.toolbar.bg_colour),
                        hover_colour: Some(rgba(BUTTON_HOVER_COLOUR)),
                        rounding: rounding!(self.style.rounding),
                        on_click: |e, window, cx| {
                           cx.spawn(|cx: &mut AsyncApp| async move {
                               if let Some(file) = AsyncFileDialog::new()
                                   .set_directory("~")
                                   .pick_folder()
                                   .await
                               {
                                   // self.state.update(cx, |state, _cx| {
                                   //     state.open_projects.projects.push(file)
                                   // });
                               }
                           })
                               .detach();
                        },
                        ..Default::default()
                    }))
                    .child(cx.new(|_| Button {
                        text: String::from("About"),
                        text_colour: rgba(self.style.text_colour),
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 40f32,
                        height: BUTTON_HEIGHT,
                        margin: margin!(self.style.margin, 0.0),
                        colour: rgba(self.style.toolbar.bg_colour),
                        hover_colour: Some(rgba(BUTTON_HOVER_COLOUR)),
                        rounding: rounding!(self.style.rounding),
                        ..Default::default()
                    }))

            )
            .child(
                div()
                    .w(px(40.0))
                    .child(cx.new(|_| Button {
                        text: String::from("X"),
                        text_colour: rgba(self.style.text_colour),
                        justify_content: TextPosition::Centre,
                        align_text: TextPosition::Centre,
                        width: 40f32,
                        height: BUTTON_HEIGHT,
                        colour: rgba(self.style.toolbar.bg_colour),
                        hover_colour: Some(rgba(BUTTON_HOVER_COLOUR)),
                        rounding: rounding!(self.style.rounding),
                        margin: margin!(self.style.margin, 0.0),
                        on_click: |e, window, cx| {
                            cx.quit()
                        },
                        ..Default::default()
                    })),
            )
    }
}
