use crate::state::State;
use gpui::EventEmitter;

pub struct OpenProjectEvent {
    project_id: u32,
}
impl EventEmitter<OpenProjectEvent> for State {}
