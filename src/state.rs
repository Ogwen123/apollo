use crate::utils::logger::warning;
use gpui::{App, Global};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
use std::sync::Arc;
// PROJECT

#[derive(Clone)]
/// Stores data about open projects
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
}

impl Project {
    pub fn new(id: u32, path: PathBuf) -> Self {
        Self { id, path }
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

    pub fn path_string(&self) -> String {
        match self.path.clone().into_os_string().into_string() {
            Ok(res) => res,
            Err(err) => {
                warning!("Could not get string from project path.");
                String::from("ERROR")
            }
        }
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: 0, // A 0 id for a project or for State::active_project means inactive
            path: PathBuf::new(),
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

// MODALS
#[derive(Clone)]
/// Stores the state of all the modals in the app
pub struct Modals {
    pub about: bool,
}

// STATE

#[derive(Clone)]
/// Stores the global state for the app
pub struct State {
    /// All of the currently open projects
    pub open_projects: OpenProjects,
    pub active_project: u32,
    pub modals: Modals,
}
impl State {
    pub fn has_path(&self, path: &PathBuf) -> bool {
        !self
            .open_projects
            .projects
            .iter()
            .filter(|x| &x.path == path)
            .next()
            .is_none()
    }

    pub fn add_project_by_path(&mut self, path: PathBuf) {
        if self.has_path(&path) {
            warning!(
                "Skipped {} because it is already open.",
                path.clone()
                    .into_os_string()
                    .into_string()
                    .unwrap_or("ERROR".to_string())
            );
            return;
        }

        let id = self
            .open_projects
            .projects
            .iter()
            .map(|x| x.id)
            .max()
            .unwrap_or(0)
            + 1;

        let project = Project::new(id, path.clone());

        self.open_projects.projects.push(project);
        self.active_project = id
    }
    pub fn add_project(&mut self, project: Project) {
        self.open_projects.projects.push(project.clone());
        self.active_project = project.id
    }
    pub fn remove_project(&mut self, id: u32) {
        self.open_projects.projects = self
            .open_projects
            .projects
            .iter()
            .filter_map(|x| if x.id != id { Some(x.clone()) } else { None })
            .collect::<Vec<Project>>();
    }
    pub fn set_active_project(&mut self, id: u32) {
        if self
            .open_projects
            .projects
            .iter()
            .filter_map(|x| if x.id == id { Some(id) } else { None })
            .collect::<Vec<u32>>()
            .len()
            < 1
        {
            warning!("Ignored setting active project to non-existent id.");
            return;
        }
        self.active_project = id;
    }
    pub fn get_active_project(&self) -> Option<Project> {
        let search = self
            .open_projects
            .projects
            .iter()
            .filter_map(|x| {
                if x.id == self.active_project {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<Project>>();
        if search.len() == 1 {
            Some(search[0].clone())
        } else {
            None
        }
    }
}

/// A trait for simplifying read-only access to global state
pub trait StateProvider {
    fn state(&self) -> &State;
}

impl StateProvider for App {
    fn state(&self) -> &State {
        self.global::<State>()
    }
}

impl Global for State {}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "State {{\n  open_projects: {}\n}}", self.open_projects)
    }
}
