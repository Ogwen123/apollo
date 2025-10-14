#[derive(Clone)]
pub struct ToolBarStyle {
    pub height: f32,
    /// Width in pixels, a width of -1 will use all available space
    pub bg_colour: u32
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
    pub text_colour: u32,
    pub bg_colour: u32,
    pub toolbar: ToolBarStyle
}

impl Default for Style {
    fn default() -> Self {
        Self {
            text_colour: 0xffffff,
            bg_colour: 0x1e1f22,
            toolbar: Default::default()
        }
    }
}

