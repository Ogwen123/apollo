use crate::style::{Colour, Size};
use gpui::{Context, IntoElement, Render, Window, div};

pub struct Spinner {
    bg_colour: Colour,
    fg_colour: Colour,
    size: Size,
}
// TODO: actually make the spinner
impl Render for Spinner {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
    }
}
