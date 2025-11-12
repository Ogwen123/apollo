use crate::style::Colour;
use gpui::prelude::FluentBuilder;
use gpui::{Context, IntoElement, Pixels, Render, SharedString, Styled, Window, px, svg};
use std::fmt::Display;

#[derive(Clone)]
pub enum Icons {
    Play,
    Close,
}

impl Into<SharedString> for Icons {
    fn into(self) -> SharedString {
        match self {
            Icons::Play => SharedString::from("play.svg"),
            Icons::Close => SharedString::from("close.svg"),
        }
    }
}

pub struct Icon {
    pub icon: Icons,
    pub colour: Colour,
    pub size: Option<Pixels>,
}

impl Icon {
    pub fn new() -> Self {
        Icon {
            icon: Icons::Close,
            colour: Colour::Rgb(0xffffff),
            size: None,
        }
    }

    pub fn icon(mut self, icon: Icons) -> Self {
        self.icon = icon;
        self
    }

    pub fn colour(mut self, colour: Colour) -> Self {
        self.colour = colour;
        self
    }

    pub fn size<T: Into<Pixels>>(mut self, size: T) -> Self {
        self.size = Some(size.into());
        self
    }
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

impl Render for Icon {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        svg()
            .path(self.icon.clone())
            .when_else(
                self.size.is_some(),
                |_self| _self.size(self.size.unwrap()),
                |_self| _self.size_full(),
            )
            .text_color(&self.colour)
            //.bg(&self.colour)
    }
}
