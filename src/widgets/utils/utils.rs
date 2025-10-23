use gpui::{App, MouseDownEvent, Window};
use std::sync::Arc;

type StorableCallback = Arc<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>;

pub fn add_opacity(colour: u32, opacity: u32) -> u32 {
    colour * 256 + opacity
}
