use gpui::{div, rgb, Context, InteractiveElement, IntoElement, ParentElement, Render, Styled, Window};

struct Button {
    text: String,
    rounding: u32,
    colour: u32,
    hover_colour: Option<u32>,
    text_colour: u32,
    border_colour: Option<u32>
}

impl Render for Button {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .text_xl()
            .border_2()
            .p_2()
            .rounded_lg()
            .border_color(rgb(self.border_colour.unwrap_or(self.colour)))
            .text_color(rgb(self.text_colour))
            .bg(rgb(self.colour))
            .hover(|style| style.bg(rgb(self.hover_colour.unwrap_or(self.colour))))
            //.on_mouse_down(MouseButton::Left, cx.listener(Self::handle_increment))
            .child(self.text.clone())
    }
}