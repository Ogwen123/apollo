use crate::utils::logger::warning;
use gpui::{AbsoluteLength, DefiniteLength, Fill, Hsla, Length, px, rgb, rgba};
use std::ops::Deref;

#[derive(Clone)]
pub enum Colour {
    Rgb(u32),
    Rgba(u32),
}

impl Into<Colour> for &Colour {
    fn into(self) -> Colour {
        self.clone()
    }
}

impl Into<Hsla> for Colour {
    fn into(self) -> Hsla {
        match self.clone() {
            Colour::Rgb(res) => rgb(res).into(),
            Colour::Rgba(res) => rgba(res).into(),
        }
    }
}

impl Into<Hsla> for &Colour {
    fn into(self) -> Hsla {
        match self.clone() {
            Colour::Rgb(res) => rgb(res).into(),
            Colour::Rgba(res) => rgba(res).into(),
        }
    }
}

impl Into<Fill> for Colour {
    fn into(self) -> Fill {
        let hsla: Hsla = self.into();

        hsla.into()
    }
}

impl Into<Fill> for &Colour {
    fn into(self) -> Fill {
        let hsla: Hsla = self.clone().into();

        hsla.into()
    }
}

#[derive(Clone, Copy)]
pub enum Size {
    /// Length in pixels
    Px(f32),
    /// Length in percent, must be between 0 and 1.
    Percent(f32),
    /// Uses w_auto(), h_auto(), etc
    Auto,
}

impl Size {
    pub fn get(&self) -> Length {
        match self.clone() {
            Size::Px(res) => Length::from(px(res)),
            Size::Percent(res) => {
                if res < 0.0 || res > 1.0 {
                    warning!(
                        "The value of Size::Percent must be between 0 and 1, defaulting to 100%."
                    );
                    return Length::from(DefiniteLength::Fraction(1.0));
                }
                Length::from(DefiniteLength::Fraction(res))
            }
            Size::Auto => Length::Auto,
        }
    }

    pub fn abs(&self) -> AbsoluteLength {
        match self.clone() {
            Size::Px(res) => AbsoluteLength::Pixels(px(res)),
            Size::Percent(_) => {
                warning!(
                    "The value of Size::Percent cannot be converted into an AbsoluteLength, defaulting to 0px."
                );
                AbsoluteLength::Pixels(px(0.0))
            }
            Size::Auto => {
                warning!("Cannot convert Size::Auto into an AbsoluteLength, defaulting to 0px.");
                AbsoluteLength::Pixels(px(0.0))
            }
        }
    }

    pub fn def(&self) -> DefiniteLength {
        match self.clone() {
            Size::Px(_) => DefiniteLength::Absolute(self.abs()),
            Size::Percent(res) => {
                if res < 0.0 || res > 1.0 {
                    warning!(
                        "The value of Size::Percent must be between 0 and 1, defaulting to 100%."
                    );
                    return DefiniteLength::Fraction(1.0);
                }
                DefiniteLength::Fraction(res)
            }
            Size::Auto => {
                warning!("Cannot convert Size::Auto into a definite length, defaulting to 0px.");
                DefiniteLength::Absolute(AbsoluteLength::Pixels(px(0.0)))
            }
        }
    }
}

#[derive(PartialEq)]
pub enum Direction {
    Horizontal,
    Vertical,
}
