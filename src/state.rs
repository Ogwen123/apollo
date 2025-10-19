use crate::utils::logger::warning;
use gpui::Global;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
// PROJECT

#[derive(Clone)]
/// Stores data about open projects
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
}

impl Project {
    pub fn new(path: PathBuf) -> Self {
        Self { id: 1, path }
    }

    pub fn display_name(&self) -> String {
        match self.path.clone().into_os_string().into_string() {
            Ok(res) => {
                if std::env::consts::OS == "windows" {
                    res.split("\\").last().unwrap_or("ERROR").to_string()
                } else {
                    res.split("/").last().unwrap_or("ERROR").to_string()
                }
            }
            Err(err) => {
                warning!("Could not get string from project path.");
                String::from("ERROR")
            }
        }
    }
}

impl Debug for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "    Project {} at {:?}", self.id, self.path)
    }
}

// OPEN PROJECTS

#[derive(Clone)]
/// Stores all the open projects
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

impl Display for OpenProjects {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Projects: {:?}", self.projects)
    }
}

// STATE

#[derive(Clone)]
/// Stores the global state for the app
pub struct State {
    /// All of the currently open projects
    pub open_projects: OpenProjects,
}
impl State {
    pub fn add_project(&mut self, project: Project) {
        self.open_projects.projects.push(project)
    }
    pub fn remove_project(&mut self, id: u32) {
        self.open_projects.projects = self
            .open_projects
            .projects
            .iter()
            .filter_map(|x| if x.id != id { Some(x.clone()) } else { None })
            .collect::<Vec<Project>>();
    }
}

impl Global for State {}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "State {{\n  open_projects: {}\n}}", self.open_projects)
    }
}
