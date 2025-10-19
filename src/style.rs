use crate::utils::logger::warning;
use gpui::{AbsoluteLength, DefiniteLength, Length, Pixels, Rgba, px, rgb, rgba};

#[derive(Clone)]
pub enum Colour {
    Rgb(u32),
    Rgba(u32),
}

impl Colour {
    pub fn get(&self) -> Rgba {
        match self.clone() {
            Colour::Rgb(res) => rgb(res),
            Colour::Rgba(res) => rgba(res),
        }
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

#[derive(Clone)]
/// Contains all the styling specific to the toolbar at the top of the app
pub struct ToolBarStyle {
    /// Height of the status bar
    pub height: Size,
    /// Background colour of the status bar
    pub bg_colour: Colour,
}

#[derive(Clone)]
/// Contains all the styling specific to the status bar at the bottom of the app
pub struct StatusBarStyle {
    /// Height of the status bar
    pub height: Size,
    /// Height of the status bar
    pub bg_colour: Colour,
}

#[derive(Clone)]
/// Contains all the styling specific to the tab bar just under the toolbar
pub struct TabBarStyle {
    /// Height of the tab bar
    pub height: Size,
    /// Height of the tab bar
    pub bg_colour: Colour,
    /// Colour of a hovered tab
    pub hover_colour: Colour,
    /// Colour of the active tab
    pub active_colour: Colour,
}

#[derive(Clone)]
/// Top level style struct
pub struct Style {
    /// Text colour
    pub text_colour: Colour,
    /// Background colour
    pub bg_colour: Colour,
    /// Primary colour for interactive elements like buttons
    pub primary_colour: Colour,
    /// Primary hover colour for interactive elements like buttons
    pub hover_primary_colour: Colour,
    /// Secondary colour
    pub secondary_colour: Colour,
    /// Secondary hover colour
    pub hover_secondary_colour: Colour,
    /// The colour used when separating ui elements, e.g. the tab bar and tab itself
    pub separator_colour: Colour,
    /// Rounding for interactive element like buttons
    pub rounding: Size,
    /// The default padding applied to most elements
    pub padding: Size,
    /// The default margin applied to most elements
    pub margin: Size,
    /// Styling for the toolbar, which is the bar at the top of the page
    pub toolbar: ToolBarStyle,
    /// Styling for the status bar, which is the bar at the bottom of the page
    pub statusbar: StatusBarStyle,
    /// Styling for the tab bar, which is just under the toolbar
    pub tabbar: TabBarStyle,
}

impl Default for ToolBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(40.0),
            bg_colour: Colour::Rgb(0x2b2d30),
        }
    }
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(20.0),
            bg_colour: Colour::Rgb(0x2b2d30),
        }
    }
}

impl Default for TabBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(40.0),
            bg_colour: Colour::Rgb(0x1e1f22),
            hover_colour: Colour::Rgb(0x4e4f42),
            active_colour: Colour::Rgba(0x2563ebaa),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_colour: Colour::Rgb(0xffffff),
            bg_colour: Colour::Rgb(0x1e1f22),
            primary_colour: Colour::Rgb(0x2563eb),
            hover_primary_colour: Colour::Rgb(0x1b46a6),
            secondary_colour: Colour::Rgb(0x2563eb),
            hover_secondary_colour: Colour::Rgb(0x1b46a6),
            separator_colour: Colour::Rgb(0x535353),
            rounding: Size::Px(4.0),
            padding: Size::Px(4.0),
            margin: Size::Px(4.0),
            toolbar: Default::default(),
            statusbar: Default::default(),
            tabbar: Default::default(),
        }
    }
}
