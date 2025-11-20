use crate::utils::logger::warning;
pub(crate) use crate::widgets::styling::{Colour, Size};
use gpui::{
    AbsoluteLength, App, Background, DefiniteLength, Fill, Global, Hsla, Length, Rgba, px, rgb,
    rgba,
};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[derive(Clone)]
/// Contains all the styling specific to the toolbar at the top of the app
pub struct ToolBarStyle {
    /// Height of the status bar
    pub height: Size,
    /// Height of the buttons in the toolbar, should be less than the height
    pub button_height: Size,
}

#[derive(Clone)]
/// Contains all the styling specific to the status bar at the bottom of the app
pub struct StatusBarStyle {
    /// Height of the status bar
    pub height: Size,
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
/// Contains all the styling specific to the control bar just under the tab bar
pub struct ControlBarStyle {
    /// Height of the control bar
    pub height: Size,
    /// Height of the buttons in the control bar
    pub button_height: Size,
    /// Background colour of the control bar
    pub bg_colour: Colour,
}

#[derive(Clone)]
pub struct AlertStyle {
    pub success: Colour,
    pub info: Colour,
    pub warning: Colour,
    pub error: Colour,
}

#[derive(Clone)]
/// Top level style struct
pub struct Style {
    /// Text colour
    pub text_colour: Colour,
    /// Text colour that is slightly more muted than the main text colour
    pub sub_text_colour: Colour,
    /// Background colour
    pub bg_colour: Colour,
    /// Secondary background colour
    pub secondary_bg_colour: Colour,
    /// Primary colour for interactive elements like buttons
    pub primary_colour: Colour,
    /// Primary hover colour for interactive elements like buttons
    pub hover_primary_colour: Colour,
    /// Secondary colour
    pub secondary_colour: Colour,
    /// Secondary hover colour
    pub hover_secondary_colour: Colour,
    /// Passed colour
    pub passed_colour: Colour,
    /// Ignore colour
    pub ignore_colour: Colour,
    /// Failed colour
    pub failed_colour: Colour,
    /// Doc-tests colour
    pub doctest_colour: Colour,
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
    /// Styling for the control bar, which is just under the tab bar
    pub controlbar: ControlBarStyle,
    /// Styling for the alert popup
    pub alert: AlertStyle,
}

impl Default for ToolBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(40.0),
            button_height: Size::Px(30.0),
        }
    }
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(20.0),
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

impl Default for ControlBarStyle {
    fn default() -> Self {
        Self {
            height: Size::Px(40.0),
            button_height: Size::Px(30.0),
            bg_colour: Colour::Rgb(0x1e1f22),
        }
    }
}

impl Default for AlertStyle {
    fn default() -> Self {
        Self {
            success: Colour::Rgb(0x6bcb61),
            info: Colour::Rgb(0x3675f2),
            warning: Colour::Rgb(0xe0ce70),
            error: Colour::Rgb(0xde5544),
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_colour: Colour::Rgb(0xffffff),
            sub_text_colour: Colour::Rgba(0xffffffbb),
            bg_colour: Colour::Rgb(0x1e1f22),
            secondary_bg_colour: Colour::Rgb(0x2b2d30),
            primary_colour: Colour::Rgb(0x2563eb),
            hover_primary_colour: Colour::Rgb(0x1b46a6),
            secondary_colour: Colour::Rgb(0x2563eb),
            hover_secondary_colour: Colour::Rgb(0x1b46a6),
            passed_colour: Colour::Rgb(0x6bcb61),
            ignore_colour: Colour::Rgb(0xe0ce70),
            failed_colour: Colour::Rgb(0xde5544),
            doctest_colour: Colour::Rgb(0x3675f2),
            separator_colour: Colour::Rgb(0x535353),
            rounding: Size::Px(4.0),
            padding: Size::Px(4.0),
            margin: Size::Px(4.0),
            toolbar: Default::default(),
            statusbar: Default::default(),
            tabbar: Default::default(),
            controlbar: Default::default(),
            alert: Default::default(),
        }
    }
}

// Extend App with the style provider

pub trait StyleProvider {
    fn style(&self) -> &Arc<Style>;
}

impl Style {
    pub fn get_global(cx: &App) -> &Arc<Style> {
        &cx.global::<GlobalStyle>().0
    }
}

#[derive(Clone)]
pub struct GlobalStyle(pub Arc<Style>);

impl Deref for GlobalStyle {
    type Target = Arc<Style>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GlobalStyle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Global for GlobalStyle {}

impl StyleProvider for App {
    fn style(&self) -> &Arc<Style> {
        &self.global::<GlobalStyle>().0
    }
}
