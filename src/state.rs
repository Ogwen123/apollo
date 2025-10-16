use std::path::PathBuf;
use rfd::FileHandle;

#[derive(Clone)]
pub struct OpenProjects {
    pub projects: Vec<FileHandle>
}

impl OpenProjects {
    pub(crate) fn new() -> OpenProjects { OpenProjects {projects: Vec::new()}}
}

#[derive(Clone)]
pub struct State {
    pub open_projects: OpenProjects
}