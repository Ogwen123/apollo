use crate::style::{Colour, Size};
use gpui::{
    Animation, AnimationExt, App, Bounds, Context, Element, ElementId, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, Pixels, Render, Styled, Transformation, Window,
    bounce, div, ease_in_out, percentage, svg,
};
use std::time::Duration;

pub struct Spinner {
    bg_colour: Colour,
    fg_colour: Colour,
    size: Size,
}
// TODO: actually make the spinner
impl Render for Spinner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        svg()
            .size(self.size.get())
            .overflow_hidden()
            .path("svg/spinner.svg")
            .text_color(&self.fg_colour)
            .bg(&self.bg_colour)
            .with_animation(
                "spinner_circle",
                Animation::new(Duration::from_secs(1))
                    .repeat()
                    .with_easing(bounce(ease_in_out)),
                |svg, delta| svg.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}

impl Spinner {
    pub fn new() -> Spinner {
        Self::default()
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            fg_colour: Colour::Rgb(0xffffff),
            bg_colour: Colour::Rgba(0x00000000),
            size: Size::Px(10.0),
        }
    }
}
