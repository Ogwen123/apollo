use crate::{margin, padding, rounding};
use gpui::{
    App, Context, Hsla, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement, Render, Rgba, Styled, Window, div, px, rgb, rgba,
};

pub enum TextPosition {
    Start,
    Centre,
    End,
}

pub struct Button {
    /// Text to be displayed on the button
    text: String,
    /// Horizontal position for the text
    justify_content: TextPosition,
    /// Vertical position for the text
    align_text: TextPosition,
    /// Width in pixels
    width: f32,
    /// Height in pixels
    height: f32,
    /// Text size in pixels
    text_size: f32,
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    rounding: (f32, f32, f32, f32),
    /// Background colour in hex e.g. 0xffffff
    colour: Rgba,
    /// Hover colour in hex e.g. 0xffffff
    hover_colour: Option<Rgba>,
    /// Text colour in hex e.g. 0xffffff
    text_colour: Rgba,
    /// Border colour
    border_colour: Option<Rgba>,
    /// Border width in pixels
    border_width: f32,
    /// Function ran on_mouse_down for left click
    on_click: fn(&MouseDownEvent, &mut Window, &mut App),
    /// Padding in pixels, ordered as (top, right, left, bottom), you can use the padding!() macro to convert a single value or x and y value to this form.
    padding: (f32, f32, f32, f32),
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    margin: (f32, f32, f32, f32),
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

impl Button {
    pub fn new() -> Self {
        Self::default()
    }

    /// Function ran on_mouse_down for left click
    pub fn on_click(mut self, handler: fn(&MouseDownEvent, &mut Window, &mut App)) -> Self {
        self.on_click = handler;
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
    pub fn w(mut self, w: f32) -> Self {
        self.width = w;
        self
    }
    /// Height in pixels
    pub fn h(mut self, h: f32) -> Self {
        self.height = h;
        self
    }
    /// Text size in pixels
    pub fn text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    /// Corner rounding in pixels, ordered as (top left, top right, bottom right, bottom left) e.g. clockwise starting at the top left, you can use the rounding!() macro to convert a single value to this form.
    pub fn rounding(mut self, rounding: (f32, f32, f32, f32)) -> Self {
        self.rounding = rounding;
        self
    }
    /// Background colour in hex e.g. 0xffffff
    pub fn colour(mut self, colour: Rgba) -> Self {
        self.colour = colour;
        self
    }
    /// Hover colour in hex e.g. 0xffffff
    pub fn hover_colour(mut self, colour: Rgba) -> Self {
        self.hover_colour = Some(colour);
        self
    }
    /// Text colour in hex e.g. 0xffffff
    pub fn text_colour(mut self, colour: Rgba) -> Self {
        self.text_colour = colour;
        self
    }
    /// Border colour
    pub fn border_colour(mut self, colour: Rgba) -> Self {
        self.border_colour = Some(colour);
        self
    }
    /// Border width in pixels
    pub fn border_width(mut self, w: f32) -> Self {
        self.border_width = w;
        self
    }

    /// Padding in pixels, ordered as (top, right, left, bottom), you can use the padding!() macro to convert a single value or x and y value to this form.
    pub fn p(mut self, padding: (f32, f32, f32, f32)) -> Self {
        self.padding = padding;
        self
    }
    /// Margin in pixels, ordered as (top, right, left, bottom), you can use the margin!() macro to convert a single value or x and y value to this form.
    pub fn m(mut self, margin: (f32, f32, f32, f32)) -> Self {
        self.margin = margin;
        self
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
