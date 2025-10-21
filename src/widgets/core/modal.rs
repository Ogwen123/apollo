use std::sync::Arc;
use crate::state::State;
use crate::style::{Colour, Size};
use crate::widgets::core::button::Button;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, BorrowAppContext, Context, DefiniteLength, InteractiveElement, IntoElement, Length,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Render, RenderOnce, Styled, Window,
    anchored, div, point, px, rgba,
};

/// Displays a modal which is horizontally centred
pub struct Modal {
    /// The title of the modal
    pub title: String,
    /// The text in the body of the modal
    pub body: String,
    /// The width of the modal
    pub width: Pixels,
    /// The height of the modal
    pub height: Pixels,
    /// The rounding of the corners of the modal
    pub rounding: Size,
    /// The background colour of the button
    pub bg_colour: Colour,
    /// The colour of the accepting button
    pub accept_button_colour: Colour,
    /// The colour of the cancelling button
    pub cancel_button_colour: Colour,
    /// The text shown on the accepting button
    pub accept_text: String,
    /// The text shown on the cancelling button
    pub cancel_text: String,
    /// The distance in pixels the modal should be from the top of the screen, if None then the modal will be vertically centered
    pub top_offset: Option<Pixels>,
    /// The function run when the accepting button is pressed
    pub on_accept: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// The function run when the cancelling button it pressed
    pub on_cancel: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// The function run when the closing button is pressed
    pub on_close: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// Whether clicking the backdrop should close the modal
    pub backdrop_close: bool,
}

impl Render for Modal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        anchored()
            .position(point(
                (window.viewport_size().width / 2.0) - (self.width / 2.0),
                if self.top_offset.is_some() {
                    self.top_offset.unwrap()
                } else {
                    (window.viewport_size().height / 2.0) - (self.height / 2.0)
                },
            ))
            .snap_to_window()
            .child(
                div()
                    .occlude()
                    .bg(rgba(0x00000055))
                    .w(window.viewport_size().width)
                    .h(window.viewport_size().height)
                    .when(self.backdrop_close, |_self| {
                        _self.on_mouse_down(MouseButton::Left, |e, window, cx| {
                            cx.update_global::<State, ()>(|global, cx| {})
                        })
                    })
                    .child(
                        div()
                            .bg(&self.bg_colour)
                            .w(self.width)
                            .h(self.height)
                            .rounded(self.rounding.abs())
                            .child(
                                div()
                                    .h(Length::from(DefiniteLength::Fraction(0.1)))
                                    .child("title"),
                            )
                            .child(
                                div()
                                    .h(Length::from(DefiniteLength::Fraction(0.7)))
                                    .child("body"),
                            )
                            .child(
                                div()
                                    .h(Length::from(DefiniteLength::Fraction(0.2)))
                                    .child(Button::new().text("accept").render(window, cx))
                                    .child(Button::new().text("cancel").render(window, cx)),
                            ),
                    ),
            )
            .into_any_element()
    }
}

impl Modal {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Set the title of the modal
    pub fn title<T: ToString>(mut self, title: T) -> Self {
        self.title = title.to_string();
        self
    }
    /// Set the text in the body of the modal
    pub fn body<T: ToString>(mut self, body: T) -> Self {
        self.body = body.to_string();
        self
    }
    /// Set the width of the modal
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }
    /// Set the height of the modal
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }
    /// Set the rounding of the corners of the modal
    pub fn rounding(mut self, rounding: Size) -> Self {
        self.rounding = rounding;
        self
    }
    /// Set the background colour of the button
    pub fn bg_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.bg_colour = colour.into();
        self
    }
    /// Set the colour of the accepting button
    pub fn accept_button_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.accept_button_colour = colour.into();
        self
    }
    /// Set the colour of the cancelling button
    pub fn cancel_button_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.cancel_button_colour = colour.into();
        self
    }
    /// Set the text shown on the accepting button
    pub fn accept_text<T: ToString>(mut self, text: T) -> Self {
        self.accept_text = text.to_string();
        self
    }
    /// Set the text shown on the cancelling button
    pub fn cancel_text<T: ToString>(mut self, text: T) -> Self {
        self.cancel_text = text.to_string();
        self
    }
    /// Set the distance in pixels the modal should be from the top of the screen, if None then the modal will be vertically centered
    pub fn top_offset(mut self, offset: Option<Pixels>) -> Self {
        self.top_offset = offset;
        self
    }
    /// Set the function run when the accepting button is pressed
    pub fn on_accept(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_accept = Some(Arc::new(handler));
        self
    }
    /// Set the function run when the cancelling button it pressed
    pub fn on_cancel(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_cancel = Some(Arc::new(handler));
        self
    }
    /// Set the function run when the closing button is pressed
    pub fn on_close(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Arc::new(handler));
        self
    }
    /// Toggle whether clicking the backdrop should close the modal, default is true
    pub fn toggle_backdrop_close(mut self) -> Self {
        self.backdrop_close = !self.backdrop_close;
        self
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self {
            title: "Placeholder title".to_string(),
            body: "Placeholder body".to_string(),
            width: px(100.0),
            height: px(100.0),
            rounding: Size::Px(0.0),
            bg_colour: Colour::Rgb(0x000000),
            accept_button_colour: Colour::Rgb(0x00ff00),
            cancel_button_colour: Colour::Rgb(0xff0000),
            accept_text: "Ok".to_string(),
            cancel_text: "Cancel".to_string(),
            top_offset: None,
            on_accept: None,
            on_cancel: None,
            on_close: None,
            backdrop_close: true,
        }
    }
}
