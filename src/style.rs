#[derive(Clone)]
/// Contains all the styling specific to the toolbar at the top of the app
pub struct ToolBarStyle {
    /// Height of the status bar
    pub height: f32,
    /// Background colour of the status bar
    pub bg_colour: u32,
}

#[derive(Clone)]
/// Contains all the styling specific to the status bar at the bottom of the app
pub struct StatusBarStyle {
    /// Height of the status bar
    pub height: f32,
    /// Height of the status bar
    pub bg_colour: u32,
}

#[derive(Clone)]
/// Contains all the styling specific to the tab bar just under the toolbar
pub struct TabBarStyle {
    /// Height of the tab bar
    pub height: f32,
    /// Height of the tab bar
    pub bg_colour: u32,
    /// Colour of a hovered tab
    pub hover_colour: u32,
    /// Colour of the active tab
    pub active_colour: u32
}

#[derive(Clone)]
/// Top level style struct
pub struct Style {
    /// Text colour
    pub text_colour: u32,
    /// Background colour
    pub bg_colour: u32,
    /// Primary colour for interactive elements like buttons
    pub primary_colour: u32,
    /// Primary hover colour for interactive elements like buttons
    pub hover_primary_colour: u32,
    /// Secondary colour
    pub secondary_colour: u32,
    /// Secondary hover colour
    pub hover_secondary_colour: u32,
    /// The colour used when separating ui elements, e.g. the tab bar and tab itself
    pub separator_colour: u32,
    /// Rounding for interactive element like buttons
    pub rounding: f32,
    /// The default padding applied to most elements
    pub padding: f32,
    /// The default margin applied to most elements
    pub margin: f32,
    /// Styling for the toolbar, which is the bar at the top of the page
    pub toolbar: ToolBarStyle,
    /// Styling for the status bar, which is the bar at the bottom of the page
    pub statusbar: StatusBarStyle,
    /// Styling for the tab bar, which is just under the toolbar
    pub tabbar: TabBarStyle
}

impl Default for ToolBarStyle {
    fn default() -> Self {
        Self {
            height: 40.0,
            bg_colour: 0x2b2d30,
        }
    }
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        Self {
            height: 20.0,
            bg_colour: 0x2b2d30,
        }
    }
}

impl Default for TabBarStyle {
    fn default() -> Self {
        Self {
            height: 40.0,
            bg_colour: 0x1e1f22,
            hover_colour: 0x4e4f42,
            active_colour: 0x2563eb
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_colour: 0xffffff,
            bg_colour: 0x1e1f22,
            primary_colour: 0x2563eb,
            hover_primary_colour: 0x1b46a6,
            secondary_colour: 0x2563eb,
            hover_secondary_colour: 0x1b46a6,
            separator_colour: 0x535353,
            rounding: 4.0,
            padding: 4.0,
            margin: 4.0,
            toolbar: Default::default(),
            statusbar: Default::default(),
            tabbar: Default::default()
        }
    }
}
