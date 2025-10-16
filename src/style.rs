#[derive(Clone)]
pub struct ToolBarStyle {
    pub height: f32,
    /// Width in pixels, a width of -1 will use all available space
    pub bg_colour: u32,
}

impl Default for ToolBarStyle {
    fn default() -> Self {
        Self {
            height: 30.0,
            bg_colour: 0x2b2d30,
        }
    }
}

#[derive(Clone)]
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
    /// Rounding for interactive element like buttons
    pub rounding: f32,
    /// The default padding applied to most elements
    pub padding: f32,
    /// Styling for the toolbar, which is the bar at the top of the page
    pub toolbar: ToolBarStyle,
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
            rounding: 4.0,
            padding: 8.0,
            toolbar: Default::default(),
        }
    }
}
