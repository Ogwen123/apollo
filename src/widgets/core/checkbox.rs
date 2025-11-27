use gpui::{div, App, Context, ElementId, IntoElement, MouseDownEvent, Render, Window};
use crate::style::{Colour, Size};

pub struct CheckBox {
    /// Whether the checkbox is checked
    checked: bool,
    /// Size in pixels
    size: Option<Size>,
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    rounding: (Size, Size, Size, Size),
    /// Checked background colour in hex e.g. 0xffffff
    checked_colour: Colour,
    /// Unchecked background colour in hex e.g. 0xffffff
    unchecked_colour: Option<Colour>,
    /// Check symbol colour in hex e.g. 0xffffff
    symbol_colour: Colour,
    /// Border colour
    border_colour: Option<Colour>,
    /// Border width in pixels
    border_width: (Size, Size, Size, Size),
    /// Function ran on_mouse_down for left click
    on_toggle: Option<Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>>,
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    margin: (Size, Size, Size, Size),
    /// If the button should be disabled
    disabled: bool,
}

impl Render for CheckBox {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}