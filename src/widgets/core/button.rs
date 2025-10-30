use crate::widgets::styling::{Colour, Size};
use crate::{margin, padding, rounding};
use gpui::prelude::FluentBuilder;
use gpui::{
    App, Context, Hsla, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement, Render, RenderOnce, Rgba, Styled, Window, div, rgb,
};
use std::sync::Arc;

#[derive(Clone)]
pub enum TextPosition {
    Start,
    Centre,
    End,
}

/// A button capable of displaying text
pub struct Button {
    /// Text to be displayed on the button
    text: String,
    /// Horizontal position for the text
    justify_content: TextPosition,
    /// Vertical position for the text
    align_text: TextPosition,
    /// Width in pixels
    width: Option<Size>,
    /// Height in pixels
    height: Option<Size>,
    /// Text size in pixels
    text_size: Size,
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    rounding: (Size, Size, Size, Size),
    /// Background colour in hex e.g. 0xffffff
    colour: Colour,
    /// Hover colour in hex e.g. 0xffffff
    hover_colour: Option<Colour>,
    /// Text colour in hex e.g. 0xffffff
    text_colour: Colour,
    /// Border colour
    border_colour: Option<Colour>,
    /// Border width in pixels
    border_width: Size,
    /// Function ran on_mouse_down for left click
    on_click: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// Padding in pixels, ordered as (top, right, left, bottom), you can use the padding!() macro to convert a single value or x and y value to this form.
    padding: (Size, Size, Size, Size),
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    margin: (Size, Size, Size, Size),
    /// If the button should be disabled
    disabled: bool,
}

impl RenderOnce for Button {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let disabled = self.disabled;

        let d = div()
            .flex()
            .when_else(
                self.height.is_some(),
                |_self| _self.h(self.height.unwrap().get()),
                |_self| _self.h_full(),
            )
            .when_else(
                self.width.is_some(),
                |_self| _self.w(self.width.unwrap().get()),
                |_self| _self.w_full(),
            )
            .text_size(self.text_size.abs())
            // Per side padding
            .pt(self.padding.0.def())
            .pr(self.padding.1.def())
            .pb(self.padding.2.def())
            .pl(self.padding.3.def())
            // Per side margin
            .mt(self.margin.0.get())
            .mr(self.margin.1.get())
            .mb(self.margin.2.get())
            .ml(self.margin.3.get())
            // Per corner rounding
            .rounded_tl(self.rounding.0.abs())
            .rounded_tr(self.rounding.1.abs())
            .rounded_br(self.rounding.2.abs())
            .rounded_bl(self.rounding.3.abs())
            .border(self.border_width.abs())
            .when_some(self.border_colour, |_self, colour| {
                _self.border_color(colour)
            })
            .text_color(self.text_colour)
            .bg(&self.colour)
            .hover(|style| style.bg(self.hover_colour.unwrap_or(self.colour)))
            .when(!self.disabled, |_self| {
                _self.when_some(self.on_click, |__self, on_click| {
                    __self.on_mouse_down(MouseButton::Left, on_click)
                })
            })
            .child(self.text.clone());

        let justified = match self.justify_content {
            TextPosition::Start => d.justify_start(),
            TextPosition::Centre => d.justify_center(),
            TextPosition::End => d.justify_end(),
        };

        let aligned = match self.align_text {
            TextPosition::Start => justified.items_start(),
            TextPosition::Centre => justified.items_center(),
            TextPosition::End => justified.items_end(),
        };

        aligned
    }
}

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    /// Function ran on_mouse_down for left click
    pub fn on_click(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Text to be displayed on the button
    pub fn text<T: ToString>(mut self, text: T) -> Self {
        self.text = text.to_string();
        self
    }
    /// Horizontal position for the text
    pub fn justify_content(mut self, pos: TextPosition) -> Self {
        self.justify_content = pos;
        self
    }
    /// Vertical position for the text
    pub fn align_text(mut self, pos: TextPosition) -> Self {
        self.align_text = pos;
        self
    }
    /// Width in pixels
    pub fn w(mut self, w: Size) -> Self {
        self.width = Some(w);
        self
    }
    /// Set the width to full
    pub fn w_full(mut self) -> Self {
        self.width = None;
        self
    }
    /// Height in pixels
    pub fn h(mut self, h: Size) -> Self {
        self.height = Some(h);
        self
    }
    /// Set the height to full
    pub fn h_full(mut self) -> Self {
        self.height = None;
        self
    }
    /// Text size in pixels
    pub fn text_size(mut self, size: Size) -> Self {
        self.text_size = size;
        self
    }
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    pub fn rounding(mut self, rounding: (Size, Size, Size, Size)) -> Self {
        self.rounding = rounding;
        self
    }
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    pub fn rounding_all(mut self, rounding: Size) -> Self {
        self.rounding = (
            rounding.clone(),
            rounding.clone(),
            rounding.clone(),
            rounding.clone(),
        );
        self
    }
    /// Background colour in hex e.g. 0xffffff
    pub fn colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.colour = colour.into();
        self
    }
    /// Hover colour in hex e.g. 0xffffff
    pub fn hover_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.hover_colour = Some(colour.into());
        self
    }
    /// Text colour in hex e.g. 0xffffff
    pub fn text_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.text_colour = colour.into();
        self
    }
    /// Border colour
    pub fn border_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.border_colour = Some(colour.into());
        self
    }
    /// Border width in pixels
    pub fn border_width(mut self, w: Size) -> Self {
        self.border_width = w;
        self
    }
    /// Padding in pixels, ordered as (top, right, bottom, left), you can use the padding!() macro to convert a single value or x and y value to this form.
    pub fn p<T: Into<Size>, R: Into<Size>, B: Into<Size>, L: Into<Size>>(
        mut self,
        padding: (T, R, B, L),
    ) -> Self {
        self.padding = (
            padding.0.into(),
            padding.1.into(),
            padding.2.into(),
            padding.3.into(),
        );
        self
    }
    /// Sets the padding for the left and right sides of the button.
    pub fn px(mut self, padding: Size) -> Self {
        self.padding = (
            self.padding.0,
            padding.clone(),
            self.padding.2,
            padding.clone(),
        );
        self
    }
    /// Sets the padding for the top and bottom of the button.
    pub fn py(mut self, padding: Size) -> Self {
        self.padding = (
            padding.clone(),
            self.padding.1,
            padding.clone(),
            self.padding.3,
        );
        self
    }
    /// Padding for all sides, given as a single Size value.
    pub fn pa(mut self, padding: Size) -> Self {
        self.padding = (
            padding.clone(),
            padding.clone(),
            padding.clone(),
            padding.clone(),
        );
        self
    }
    /// Margin in pixels, ordered as (top, right, bottom, left)
    pub fn m(mut self, margin: (Size, Size, Size, Size)) -> Self {
        self.margin = margin;
        self
    }
    /// Sets the margin for the left and right sides of the button.
    pub fn mx(mut self, margin: Size) -> Self {
        self.margin = (self.margin.0, margin.clone(), self.margin.2, margin.clone());
        self
    }
    /// Sets the margin for the top and bottom of the button.
    pub fn my(mut self, margin: Size) -> Self {
        self.margin = (margin.clone(), self.margin.1, margin.clone(), self.margin.3);
        self
    }
    /// Margin for all sides, given as a single Size value.
    pub fn ma(mut self, margin: Size) -> Self {
        self.margin = (
            margin.clone(),
            margin.clone(),
            margin.clone(),
            margin.clone(),
        );
        self
    }
    /// Disable button
    fn disable(mut self) -> Self {
        self.disabled = true;
        self
    }
    /// Enable button
    fn enable(mut self) -> Self {
        self.disabled = false;
        self
    }
    /// Toggle the button
    fn toggle(mut self) -> Self {
        self.disabled = !self.disabled;
        self
    }
    // Mimic gpuis conditional functionality
    /// The callback is run if the predicate is true
    pub fn when(mut self, predicate: bool, callback: impl FnOnce(Self) -> Self) -> Self {
        if predicate { callback(self) } else { self }
    }
    /// The callback is run if the predicate is true
    pub fn when_some<T>(
        mut self,
        option: Option<T>,
        callback: impl FnOnce(Self, T) -> Self,
    ) -> Self {
        if option.is_some() {
            callback(self, option.unwrap())
        } else {
            self
        }
    }
    /// The callback is run if the predicate is true
    pub fn when_else(
        mut self,
        predicate: bool,
        true_callback: impl FnOnce(Self) -> Self,
        false_callback: impl FnOnce(Self) -> Self,
    ) -> Self {
        if predicate {
            true_callback(self)
        } else {
            false_callback(self)
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            text: String::new(),
            justify_content: TextPosition::Start,
            align_text: TextPosition::Start,
            width: Some(Size::Px(100.0)),
            height: Some(Size::Px(50.0)),
            text_size: Size::Px(12.0),
            rounding: rounding!(Size::Px(0.0)),
            colour: Colour::Rgb(0xf5f5f5),
            hover_colour: None,
            text_colour: Colour::Rgb(0x000000),
            border_colour: None,
            border_width: Size::Px(0.0),
            on_click: None,
            padding: padding!(Size::Px(0.0)),
            margin: margin!(Size::Px(0.0)),
            disabled: false,
        }
    }
}
