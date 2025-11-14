use crate::display_vec;
use crate::utils::logger::warning;
use cargo_ptest::parse::{ParsedTest, ParsedTestGroup};
use gpui::{App, Global};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::path::PathBuf;
// PROJECT

#[derive(Clone, Serialize, Deserialize)]
/// Stores data about open projects
pub struct Project {
    pub id: u32,
    pub path: PathBuf,
    #[serde(skip_serializing, skip_deserializing)]
    pub tests: Option<Vec<ParsedTestGroup>>,
    #[serde(skip_serializing, skip_deserializing)]
    /// The index of the selected tests from the tests_linear() function
    pub selected_test: Option<usize>,
}

impl Project {
    pub fn new(id: u32, path: PathBuf) -> Self {
        Self {
            id,
            path,
            tests: None,
            selected_test: None,
        }
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
        self.path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_else(|err| {
                warning!("Could not get string from project path.");
                println!("{:?}", err);
                String::from("ERROR")
            })
    }

    pub fn tests_linear(&self) -> Option<Vec<ParsedTest>> {
        let mut tests = Vec::new();

        if self.tests.is_none() {
            return None;
        }

        for group in self.tests.clone().unwrap() {
            for test in group.tests {
                tests.push(test)
            }
        }

        Some(tests)
    }
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: 0, // A 0 id for a project or for State::active_project means inactive
            path: PathBuf::new(),
            tests: None,
            selected_test: None,
        }
    }
}

impl Debug for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "    Project {} at {:?}", self.id, self.path)
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Project{{    id: {},\n    path: {:?},\n    tests: {:?}\n}}",
            self.id,
            self.path,
            display_vec!(self.tests.clone().unwrap_or(Vec::new()))
        )
    }
}

// STATUS

#[derive(Clone, Serialize)]
/// Stores information for what to display on the status bar
pub struct Status {
    pub running_tests: bool,
}

impl Default for Status {
    fn default() -> Self {
        Self {
            running_tests: false,
        }
    }
}

// ALERT

#[derive(Clone)]
pub enum AlertSeverity {
    SUCCESS,
    INFO,
    WARNING,
    ERROR,
}

#[derive(Clone)]
pub enum AlertType {
    Timed(f64),
    UserMustClose,
}

#[derive(Clone)]
pub struct Alert {
    pub string: String,
    pub severity: AlertSeverity,
    pub _type: AlertType,
}

// RUN ARGS

#[derive(Clone)]
pub struct RunArgs {
    pub lib: bool,
    pub bin: bool,
    pub docs: bool,
    pub workspace: bool,
    pub no_fail_fast: bool,
}

impl Default for RunArgs {
    fn default() -> Self {
        Self {
            lib: false,
            bin: false,
            docs: false,
            workspace: true,
            no_fail_fast: true,
        }
    }
}

impl RunArgs {
    pub fn default_vec() -> Vec<String> {
        vec!["--no-fail-fast".to_string(), "--workspace".to_string()]
    }
}

impl Into<Vec<String>> for RunArgs {
    fn into(self) -> Vec<String> {
        let mut args = Vec::new();

        if self.lib {
            args.push(String::from("--lib"))
        }

        if self.bin {
            args.push(String::from("--bin"))
        }

        if self.docs {
            args.push(String::from("--docs"))
        }

        if self.workspace {
            args.push(String::from("--workspace"))
        }

        if self.no_fail_fast {
            args.push(String::from("--no-fail-fast"))
        }

        args
    }
}

// STATE

#[derive(Clone)]
/// Stores the global state for the app
pub struct State {
    /// All of the currently open projects
    pub open_projects: Vec<Project>,
    pub active_project: u32,
    pub status: Status,
    /// Client-side decorations for wayland
    pub csd: bool,
    /// Stores data for the alert banner
    pub alert: Option<Alert>,
    /// Args passed into cargo_ptest::Run::run()
    pub run_args: RunArgs,
}

impl State {
    pub fn has_path(&self, path: &PathBuf) -> bool {
        !self
            .open_projects
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

        let id = self.open_projects.iter().map(|x| x.id).max().unwrap_or(0) + 1;

        let project = Project::new(id, path.clone());

        self.open_projects.push(project);
        self.active_project = id
    }
    pub fn add_project(&mut self, project: Project) {
        self.open_projects.push(project.clone());
        self.active_project = project.id
    }
    pub fn remove_project(&mut self, id: u32) {
        self.open_projects = self
            .open_projects
            .iter()
            .filter_map(|x| if x.id != id { Some(x.clone()) } else { None })
            .collect::<Vec<Project>>();
        if id == self.active_project {
            match self.open_projects.first() {
                Some(proj) => self.set_active_project(proj.id),
                None => self.set_active_project(0),
            }
        }
    }
    pub fn set_active_project(&mut self, id: u32) {
        if self
            .open_projects
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
    pub fn has_active_project(&self) -> bool {
        self.get_active_project().is_some()
    }
    pub fn set_tests(&mut self, id: u32, tests: Vec<ParsedTestGroup>) {
        self.open_projects = self
            .open_projects
            .clone()
            .into_iter()
            .map(|x| {
                if x.id == id {
                    Project {
                        tests: Some(tests.clone()),
                        ..x
                    }
                } else {
                    x
                }
            })
            .collect::<Vec<Project>>();
    }
    pub fn clear_tests(&mut self, id: u32) {
        self.open_projects = self
            .open_projects
            .clone()
            .into_iter()
            .map(|x| {
                if x.id == id {
                    Project {
                        tests: None,
                        selected_test: None,
                        ..x
                    }
                } else {
                    x
                }
            })
            .collect::<Vec<Project>>();
    }
    pub fn select_test(&mut self, index: usize) {
        self.open_projects = self
            .open_projects
            .clone()
            .into_iter()
            .map(|x| {
                if x.id == self.active_project {
                    Project {
                        selected_test: Some(index),
                        ..x
                    }
                } else {
                    x
                }
            })
            .collect::<Vec<Project>>();
    }
    pub fn get_selected_test(&self) -> Option<ParsedTest> {
        if !self.has_active_project() {
            return None;
        }
        if self.get_active_project()?.tests_linear().is_none() {
            return None;
        }
        if self.get_active_project()?.selected_test.is_none() {
            return None;
        }
        match self
            .get_active_project()?
            .tests_linear()?
            .get(self.get_active_project()?.selected_test?)
        {
            Some(res) => Some(res.clone()),
            None => None,
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
        write!(f, "State {{\n  open_projects: {:?}\n}}", self.open_projects)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            open_projects: Vec::new(),
            active_project: 0,
            status: Default::default(),
            csd: false,
            alert: None,
            run_args: Default::default(),
        }
    }
}
