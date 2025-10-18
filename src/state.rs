use gpui::{Entity, Global};
use rfd::FileHandle;

#[derive(Clone)]
pub struct Project {
    pub id: u32,
    pub handle: FileHandle,
}

#[derive(Clone)]
pub struct OpenProjects {
    pub projects: Vec<Project>,
}

impl OpenProjects {
    pub(crate) fn new() -> OpenProjects {
        OpenProjects {
            projects: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct State {
    pub open_projects: OpenProjects,
}
impl Global for State {}

pub struct StateModel {
    pub(crate) inner: Entity<State>,
}

impl Global for StateModel {}
