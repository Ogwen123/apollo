use gpui::{Rgba, rgba};

pub fn make_rgba(colour: u32) -> Rgba {
    if colour <= 0xffffff {
        rgba(colour * 256 + 0xff)
    } else {
        rgba(colour)
    }
}
