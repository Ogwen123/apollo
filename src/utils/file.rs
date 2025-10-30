use crate::state::{Project, State, Status};
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::fmt::{Debug, Formatter};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::display_vec;
use crate::utils::logger::warning;

fn config_folder() -> Result<PathBuf, String> {

    match home_dir() {
        Some(res) => {
            // if the parent folder doesn't already exist then create it
            let cfg_path = res.join(".config").join("apollo");

            if !cfg_path.exists() {
                match fs::create_dir_all(cfg_path.clone()) {
                    Ok(_) => {},
                    Err(err) => {
                        warning!("Failed when making config directory at {:?}", cfg_path);
                    }
                }
            }

            Ok(cfg_path)
        },
        None => {
            println!("Could not get home directory");
            Err(String::from("Could not get home dir."))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SaveOpenProjects {
    open_projects: Vec<Project>,
    active_project: u32,
}

impl Default for SaveOpenProjects {
    fn default() -> Self {
        Self {
            open_projects: Vec::new(),
            active_project: 0
        }
    }
}

impl Debug for SaveOpenProjects {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SaveOpenProjects {{\n    open_projects: {:?},\n    active_project: {}\n}}", display_vec!(self.open_projects), self.active_project)
    }
}

pub fn save_state(state: State) {
    let save_obj = SaveOpenProjects {
        open_projects: state.open_projects,
        active_project: state.active_project,
    };

    let str = match serde_json::to_string(&save_obj) {
        Ok(res) => res,
        Err(err) => {
            println!("Could not serialise open_project data: {}", err);
            return
        }
    };

    // write to file
    match config_folder() {
        Ok(mut res) => {
            res = res.join("open_projects.json");
            println!("save path: {:?}", res);
            //open file
            let mut file = match OpenOptions::new()
                .write(true)
                .truncate(true)
                .create(true)
                .open(res)
            {
                Ok(res) => res,
                Err(err) => {
                    println!("Could not open open_projects file (save): {}", err);
                    return;
                }
            };
            //save to file
            match file.write_all(str.as_bytes()) {
                Err(err) => {
                    println!("Could not write open projects to file: {}", err);
                    return
                }
                _ => {}
            }

        }
        Err(_) => {
            warning!("Writing state to file failed.");
            return
        },
    }
}

pub fn load_state() -> State {
    match config_folder() {
        Ok(mut res) => {
            res = res.join("open_projects.json");
            //open file
            let mut file = match OpenOptions::new()
                .read(true)
                .open(res)
            {
                Ok(res) => res,
                Err(err) => {
                    println!("Could not open open_projects file (load), using empty State: {}", err);
                    return State::default();
                }
            };
            //read from file
            let mut data: String = String::new();
            let _ = file.read_to_string(&mut data);

            let saved_state: SaveOpenProjects = serde_json::from_str(data.as_str()).unwrap_or(SaveOpenProjects::default());

            State {
                open_projects: saved_state.open_projects,
                active_project: saved_state.active_project,
                status: Status::default()
            }

        }
        Err(_) => State::default(),
    }
}
