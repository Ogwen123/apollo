use std::env::home_dir;
use std::path::PathBuf;
use serde::Serialize;
use crate::state::{Project, State};

fn config_folder() -> Result<PathBuf, String> {
    match home_dir() {
        Some(res) => Ok(res.join(".config").join("apollo")),
        None => {
            println!("Could not get home directory");
            Err(String::from("Could not get home dir."))
        }
    }
}

#[derive(Serialize)]
struct SaveOpenProjects {
    open_projects: Vec<Project>,
    active_project: u32
}

pub fn save_state(state: State) {
    let save_obj = SaveOpenProjects {
        open_projects: state.open_projects,
        active_project: state.active_project
    };
    
    let str = serde_json::to_string(&save_obj);

    // write to file
    match config_folder() {
        Ok(mut res) => {
            res.set_file_name("open_projects.json");
            //open file
            //save to file
            // close
            
        },
        Err(_) => {}
    }
}

pub fn load_state() -> State {
    State::default()
}