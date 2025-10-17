use gpui::{App, Context, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, Render, Styled, Window, div, px, rgb, rgba, Hsla, Rgba};
use crate::{margin, padding, rounding};

pub enum TextPosition {
    Start,
    Centre,
    End,
}

pub struct Button {
    /// Text to be displayed on the button
    pub text: String,
    /// Horizontal position for the text
    pub justify_content: TextPosition,
    /// Vertical position for the text
    pub align_text: TextPosition,
    /// Width in pixels
    pub width: f32,
    /// Height in pixels
    pub height: f32,
    /// Text size in pixels
    pub text_size: f32,
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    pub rounding: (f32, f32, f32, f32),
    /// Background colour in hex e.g. 0xffffff
    pub colour: Rgba,
    /// Hover colour in hex e.g. 0xffffff
    pub hover_colour: Option<Rgba>,
    /// Text colour in hex e.g. 0xffffff
    pub text_colour: Rgba,
    /// Border colour
    pub border_colour: Option<Rgba>,
    /// Border width in pixels
    pub border_width: f32,
    /// Function ran on_mouse_down for left click
    pub on_click: fn(&MouseDownEvent, &mut Window, &mut App),
    /// Padding in pixels, ordered as (top, right, left, bottom), you can use the padding!() macro to convert a single value or x and y value to this form.
    pub padding: (f32, f32, f32, f32),
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    pub margin: (f32, f32, f32, f32),
}

impl Render for Button {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let d = div()
            .flex()
            .h(px(self.height))
            .w(px(self.width))
            .text_size(px(self.text_size))
            // Per side padding
            .pt(px(self.padding.0))
            .pr(px(self.padding.1))
            .pb(px(self.padding.2))
            .pl(px(self.padding.3))
            // Per side margin
            .mt(px(self.margin.0))
            .mr(px(self.margin.1))
            .mb(px(self.margin.2))
            .ml(px(self.margin.3))
            // Per corner rounding
            .rounded_tl(px(self.rounding.0))
            .rounded_tr(px(self.rounding.1))
            .rounded_br(px(self.rounding.2))
            .rounded_bl(px(self.rounding.3))
            .border(px(self.border_width))
            .border_color(self.border_colour.unwrap_or(self.colour))
            .text_color(self.text_colour)
            .bg(self.colour)
            .hover(|style| style.bg(self.hover_colour.unwrap_or(self.colour)))
            .on_mouse_down(MouseButton::Left, self.on_click)
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

impl Default for Button {
    fn default() -> Self {
        Self {
            text: String::new(),
            justify_content: TextPosition::Start,
            align_text: TextPosition::Start,
            width: 100.0,
            height: 50.0,
            text_size: 12.0,
            rounding: rounding!(0.0),
            colour: rgb(0xf5f5f5),
            hover_colour: None,
            text_colour: rgb(0x000000),
            border_colour: None,
            border_width: 0.0,
            on_click: |_, _, _| {},
            padding: padding!(0.0),
            margin: margin!(0.0),
        }
    }
}
