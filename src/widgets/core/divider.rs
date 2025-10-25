use crate::style::{Colour, Size};
use crate::widgets::styling::Direction;
use gpui::prelude::FluentBuilder;
use gpui::{App, IntoElement, Render, RenderOnce, Styled, Window, div};

pub struct Divider {
    /// The colour of the divider
    colour: Colour,
    /// The direction of the divider, horizontal or vertical
    direction: Direction,
    /// The thickness of the divider in pixels
    thickness: Size,
}

impl RenderOnce for Divider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        div().bg(self.colour).when_else(
            self.direction == Direction::Horizontal,
            |_self| _self.w_full().h(self.thickness.get()),
            |_self| _self.h_full().w(self.thickness.get()),
        )
    }
}

impl Divider {
    pub fn new() -> Self {
        Self::default()
    }
    /// Sets the colour of the divider
    pub fn colour<T: Into<Colour>>(mut self, colour: T) -> Self {
        self.colour = colour.into();
        self
    }
    /// Sets whether the divider is horizontal or vertical
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    /// Thickness in pixels
    pub fn thickness(mut self, thickness: f32) -> Self {
        self.thickness = Size::Px(thickness);
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            colour: Colour::Rgb(0x000000),
            direction: Direction::Vertical,
            thickness: Size::Px(0.0),
        }
    }
}
