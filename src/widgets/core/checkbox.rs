use std::rc::Rc;
use crate::style::{Colour, Size};
use crate::widgets::core::icon::{Icon, Icons};
use gpui::prelude::FluentBuilder;
use gpui::{App, AppContext, Context, ElementId, IntoElement, MouseDownEvent, ParentElement, Render, Styled, Window, div, InteractiveElement, MouseButton, RenderOnce};
use crate::{margin, rounding};

pub struct CheckBox {
    /// Whether the checkbox is checked
    checked: bool,
    /// Size in pixels
    size: Size,
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    rounding: (Size, Size, Size, Size),
    /// Checked background colour in hex e.g. 0xffffff
    checked_colour: Colour,
    /// Unchecked background colour in hex e.g. 0xffffff
    unchecked_colour: Colour,
    /// Check symbol colour in hex e.g. 0xffffff
    symbol_colour: Colour,
    /// Border colour
    border_colour: Option<Colour>,
    /// Border width in pixels
    border_width: Size,
    /// Function ran on_mouse_down for left click
    on_toggle: Option<Rc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    margin: (Size, Size, Size, Size),
    /// If the button should be disabled
    disabled: bool,
}

impl Render for CheckBox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_center()
            .items_center()
            .size(self.size.get())
            .when_else(
                self.checked,
                |_self| _self.bg(&self.checked_colour),
                |_self| _self.bg(&self.unchecked_colour),
            )
            .rounded_tl(self.rounding.0.abs())
            .rounded_tr(self.rounding.1.abs())
            .rounded_br(self.rounding.2.abs())
            .rounded_bl(self.rounding.3.abs())
            .mt(self.rounding.0.abs())
            .mr(self.rounding.1.abs())
            .mb(self.rounding.2.abs())
            .ml(self.rounding.3.abs())
            .when_some(self.border_colour.clone(), |_self, colour| {
                _self.border_color(colour)
            })
            .border(self.border_width.abs())
            .when(!self.disabled, |_self| {
                let oc = self.on_toggle.clone().unwrap();
                _self.when(self.on_toggle.is_some(), |__self| {
                    __self.on_mouse_down(MouseButton::Left, move |e, window, cx| {
                        oc(e, window, cx)
                    })
                })
            })
            .when(self.checked, |_self| {
                _self.child(cx.new(|_| {
                    Icon::new()
                        .icon(Icons::Check)
                        .size(self.size.px())
                        .colour(&self.symbol_colour)
                }))
            })
    }
}

impl CheckBox {
    pub fn new() -> Self {
        Self {
            checked: false,
            size: Size::Px(5.0),
            rounding: rounding!(Size::Px(5.0)),
            checked_colour: Colour::Rgb(0x2563eb),
            unchecked_colour: Colour::Rgba(0x00000000),
            symbol_colour: Colour::Rgb(0xffffff22),
            border_colour: None,
            border_width: Size::Px(0.0),
            on_toggle: None,
            margin: margin!(Size::Px(0.0)),
            disabled: false

        }
    }
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }
    /// Function ran on_mouse_down for left click
    pub fn on_toggle(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }

    /// Size in pixels
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
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
    pub fn checked_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.checked_colour = colour.into();
        self
    }
    /// Hover colour in hex e.g. 0xffffff
    pub fn unchecked_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.unchecked_colour = colour.into();
        self
    }
    /// Symbol colour in hex e.g. 0xffffff
    pub fn symbol_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.symbol_colour = colour.into();
        self
    }
    /// Border colour
    pub fn border_colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.border_colour = Some(colour.into());
        self
    }
    /// Border width in pixels
    pub fn border(mut self, w: Size) -> Self {
        self.border_width = w;
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
    /// Sets the margin for the top of the button.
    pub fn mt(mut self, margin: Size) -> Self {
        self.margin = (margin.clone(), self.margin.1, self.margin.2, self.margin.3);
        self
    }
    /// Sets the margin for the right of the button.
    pub fn mr(mut self, margin: Size) -> Self {
        self.margin = (self.margin.0, margin.clone(), self.margin.2, self.margin.3);
        self
    }
    /// Sets the margin for the bottom of the button.
    pub fn mb(mut self, margin: Size) -> Self {
        self.margin = (self.margin.0, self.margin.1, margin.clone(), self.margin.3);
        self
    }
    /// Sets the margin for the left of the button.
    pub fn ml(mut self, margin: Size) -> Self {
        self.margin = (self.margin.0, self.margin.1, self.margin.2, margin.clone());
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
    pub fn disable(mut self) -> Self {
        self.disabled = true;
        self
    }
    /// Enable button
    pub fn enable(mut self) -> Self {
        self.disabled = false;
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
