use gpui::{div, px, rgb, App, Context, InteractiveElement, IntoElement, MouseButton, MouseDownEvent, ParentElement, Render, Styled, Window};

pub struct Button {
    /// Text to be displayed on the button
    pub text: String,
    /// Width in pixels
    pub width: f32,
    /// Height in pixels
    pub height: f32,
    /// Text size in pixels
    pub text_size: f32,
    /// Corner rounding in pixels
    pub rounding: f32,
    /// Background colour in hex e.g. 0xffffff
    pub colour: u32,
    /// Hover colour in hex e.g. 0xffffff
    pub hover_colour: Option<u32>,
    /// Text colour in hex e.g. 0xffffff
    pub text_colour: u32,
    /// Border colour
    pub border_colour: Option<u32>,
    /// Border width in pixels
    pub border_width: f32,
    /// Function ran on_mouse_down for left click
    pub on_click: fn(&MouseDownEvent, &mut Window, &mut App),
    /// Padding in pixels
    pub padding: f32,
    /// Margin in pixels
    pub margin: f32
}

impl Render for Button {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .h(px(self.height))
            .w(px(self.width))
            .text_size(px(self.text_size))
            .border(px(self.border_width))
            .p(px(self.padding))
            .m(px(self.margin))
            .rounded(px(self.rounding))
            .border_color(rgb(self.border_colour.unwrap_or(self.colour)))
            .text_color(rgb(self.text_colour))
            .bg(rgb(self.colour))
            .hover(|style| style.bg(rgb(self.hover_colour.unwrap_or(self.colour))))
            .on_mouse_down(MouseButton::Left, self.on_click)
            .child(self.text.clone())
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            text: String::new(),
            width: 100.0,
            height: 50.0,
            text_size: 12.0,
            rounding: 0.0,
            colour: 0xf5f5f5,
            hover_colour: None,
            text_colour: 0x0000000,
            border_colour: None,
            border_width: 0.0,
            on_click: |_, _, _| {},
            padding: 0.0,
            margin: 0.0
        }
    }
}