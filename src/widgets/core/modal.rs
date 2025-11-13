use crate::ModalHelper;
use crate::state::State;
use crate::style::{Colour, Size};
use crate::widgets::core::button::button::{Button, ContentPosition};
use crate::widgets::core::button::icon_button::IconButton;
use crate::widgets::core::icon::Icons;
use gpui::prelude::FluentBuilder;
use gpui::{
    App, BorrowAppContext, Context, DefiniteLength, InteractiveElement, IntoElement, Length,
    MouseButton, MouseDownEvent, ParentElement, Pixels, Render, RenderOnce, Styled, Window,
    anchored, div, point, px, rgb, rgba,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct ModalButtonOptions {
    /// Whether to display the button in the modal
    pub show: bool,
    /// The text shown on the button
    pub text: String,
    /// The colour of the button
    pub colour: Colour,
    /// The hover colour of the button
    pub hover_colour: Option<Colour>,
    /// The padding of the button
    pub padding: Size,
    /// The rounding of the button
    pub rounding: Size,
    /// The thickness of the border of the button
    pub border_width: Size,
    /// The colour of the border of the button
    pub border_colour: Option<Colour>,
    /// The function run when the button is clicked
    pub on_click: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
}

impl ModalButtonOptions {
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Arc::new(handler));
        self
    }
}

/// Displays a modal which is horizontally centred
pub struct Modal {
    /// The title of the modal
    pub title: String,
    /// The body as a vec of strings that represent each line in the body
    pub body: Vec<String>,
    /// The width of the modal
    pub width: Pixels,
    /// The height of the modal
    pub height: Pixels,
    /// Sets the padding around in the inside edge of the modal
    pub padding: Size,
    /// The rounding of the corners of the modal
    pub rounding: Size,
    /// The background colour of the button
    pub bg_colour: Colour,
    /// Button options for the accept button
    pub accept_button: ModalButtonOptions,
    /// Button options for the cancel button
    pub cancel_button: ModalButtonOptions,
    /// The distance in pixels the modal should be from the top of the screen, if None then the modal will be vertically centered
    pub top_offset: Option<Pixels>,
    /// The function run when the closing button is pressed
    pub on_close: Option<Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// Whether clicking the backdrop should close the modal
    pub backdrop_close: bool,
}

impl Render for Modal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        anchored()
            .snap_to_window()
            .child(
                div()
                    .occlude()
                    .bg(rgba(0x00000055))
                    .pl((window.viewport_size().width / 2.0) - (self.width / 2.0))
                    .pt(if self.top_offset.is_some() {
                        self.top_offset.unwrap()
                    } else {
                        (window.viewport_size().height / 2.0) - (self.height / 2.0)
                    })
                    .w(window.viewport_size().width)
                    .h(window.viewport_size().height)
                    .when(self.backdrop_close, |_self| {
                        _self.on_mouse_down(MouseButton::Left, |_, window, cx| {
                            window.close_modal(cx)
                        })
                    })
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .occlude()
                            .bg(&self.bg_colour)
                            .w(self.width)
                            .h(self.height)
                            .p(self.padding.def())
                            .rounded(self.rounding.abs())
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .text_2xl()
                                    .mb_2()
                                    .items_center()
                                    .h(Length::from(DefiniteLength::Fraction(0.1)))
                                    .child(self.title.clone())
                                    .child({
                                        let oc = self.on_close.clone();

                                        IconButton::new("close-button")
                                            .icon(Icons::Close)
                                            .h(Size::Px(20.0))
                                            .w(Size::Px(20.0))
                                            .colour(&self.cancel_button.colour)
                                            .pa(self.accept_button.padding)
                                            .icon_colour(Colour::Rgb(0xffffff))
                                            .icon_size(Size::Px(15.0))
                                            .justify_content(ContentPosition::Centre)
                                            .align_text(ContentPosition::Centre)
                                            .hover_colour(Colour::Rgb(0xff0000))
                                            .rounding_all(self.rounding)
                                            .on_click(move |e, _window, _cx| {
                                                let _oc =
                                                    oc.clone().unwrap_or(Arc::new(|_, _, _| {}));
                                                _oc(e, _window, _cx)
                                            })
                                            .render(window, cx)
                                    }),
                            )
                            .child(
                                div()
                                    .h(Length::from(DefiniteLength::Fraction(0.8)))
                                    .children(self.body.iter().map(|x| div().child(x.to_string()))),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .h(Length::from(DefiniteLength::Fraction(0.1)))
                                    .when(self.accept_button.show, |_self| {
                                        let oc = self.accept_button.on_click.clone();

                                        _self.child(
                                            Button::new("accept_button")
                                                .h(Size::Px(20.0))
                                                .colour(&self.accept_button.colour)
                                                .pa(self.accept_button.padding)
                                                .text_colour(Colour::Rgb(0xffffff))
                                                .text(&self.accept_button.text)
                                                .border_width(self.accept_button.border_width)
                                                .w_full()
                                                .justify_content(ContentPosition::Centre)
                                                .border_colour(
                                                    if self.accept_button.border_colour.is_none() {
                                                        Colour::Rgba(0x00000000)
                                                    } else {
                                                        self.accept_button
                                                            .border_colour
                                                            .clone()
                                                            .unwrap()
                                                    },
                                                )
                                                .when_some(
                                                    self.accept_button.hover_colour.clone(),
                                                    |_self, colour| _self.hover_colour(colour),
                                                )
                                                .rounding_all(self.accept_button.rounding)
                                                .on_click(move |e, _window, _cx| {
                                                    let _oc = oc
                                                        .clone()
                                                        .unwrap_or(Arc::new(|_, _, _| {}));
                                                    _oc(e, _window, _cx)
                                                })
                                                .render(window, cx),
                                        )
                                    })
                                    .when(self.cancel_button.show, |_self| {
                                        let oc = self.cancel_button.on_click.clone();
                                        _self.child(
                                            Button::new("cancel_button")
                                                .h(Size::Px(20.0))
                                                .colour(&self.cancel_button.colour)
                                                .pa(self.accept_button.padding)
                                                .text_colour(Colour::Rgb(0xffffff))
                                                .text(&self.cancel_button.text)
                                                .border_width(self.cancel_button.border_width)
                                                .w_full()
                                                .justify_content(ContentPosition::Centre)
                                                .border_colour(
                                                    if self.cancel_button.border_colour.is_none() {
                                                        Colour::Rgba(0x00000000)
                                                    } else {
                                                        self.cancel_button
                                                            .border_colour
                                                            .clone()
                                                            .unwrap()
                                                    },
                                                )
                                                .when_some(
                                                    self.cancel_button.hover_colour.clone(),
                                                    |_self, colour| _self.hover_colour(colour),
                                                )
                                                .rounding_all(self.cancel_button.rounding)
                                                .on_click(move |e, _window, _cx| {
                                                    let _oc = oc
                                                        .clone()
                                                        .unwrap_or(Arc::new(|_, _, _| {}));
                                                    _oc(e, _window, _cx)
                                                })
                                                .render(window, cx),
                                        )
                                    }),
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
    pub fn body<T: ToString>(mut self, body: Vec<T>) -> Self {
        self.body = body.iter().map(|x| x.to_string()).collect::<Vec<String>>();
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
    /// Set the padding on the inside edge of the modal
    pub fn padding(mut self, padding: Size) -> Self {
        self.padding = padding;
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
    /// Set the options of the cancel buttons
    pub fn cancel_button_options(mut self, options: Option<ModalButtonOptions>) -> Self {
        if options.is_none() {
            self.cancel_button = ModalButtonOptions {
                show: false,
                ..self.cancel_button
            }
        } else {
            self.cancel_button = options.unwrap();
        }
        self
    }
    /// Set the options of the accept buttons
    pub fn accept_button_options(mut self, options: Option<ModalButtonOptions>) -> Self {
        if options.is_none() {
            self.accept_button = ModalButtonOptions {
                show: false,
                ..self.accept_button
            }
        } else {
            self.accept_button = options.unwrap();
        }
        self
    }
    /// Set the distance in pixels the modal should be from the top of the screen, if None then the modal will be vertically centered
    pub fn top_offset(mut self, offset: Option<Pixels>) -> Self {
        self.top_offset = offset;
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
            body: vec!["Line 1".to_string(), "Line 2".to_string()],
            width: px(300.0),
            height: px(300.0),
            padding: Size::Px(0.0),
            rounding: Size::Px(0.0),
            bg_colour: Colour::Rgb(0x000000),
            accept_button: ModalButtonOptions {
                show: true,
                text: "Accept".to_string(),
                colour: Colour::Rgb(0xffffff),
                hover_colour: None,
                padding: Size::Px(0.0),
                rounding: Size::Px(0.0),
                border_width: Size::Px(0.0),
                border_colour: None,
                on_click: None,
            },
            cancel_button: ModalButtonOptions {
                show: true,
                text: "Cancel".to_string(),
                colour: Colour::Rgb(0xffffff),
                hover_colour: None,
                padding: Size::Px(0.0),
                rounding: Size::Px(0.0),
                border_width: Size::Px(0.0),
                border_colour: None,
                on_click: None,
            },
            top_offset: None,
            on_close: None,
            backdrop_close: true,
        }
    }
}
