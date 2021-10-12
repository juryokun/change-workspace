use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::sync::Mutex;
use structopt::StructOpt;
extern crate dirs;
use std::path::PathBuf;

static CONFIG_DATA: Lazy<Mutex<Config>> = Lazy::new(|| {
    let file_name = get_settings_file();
    let reader = BufReader::new(File::open(file_name).unwrap());
    let config: Config = serde_json::from_reader(reader).unwrap();
    Mutex::new(config)
});

#[cfg(not(test))]
fn get_settings_file() -> PathBuf {
    let mut config_dir = dirs::config_dir().unwrap();
    config_dir.push("change-workspace");
    config_dir.push("config.json");
    config_dir
}
#[cfg(test)]
fn get_settings_file() -> PathBuf {
    PathBuf::from("rsc/config.json")
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    base: String,
    target: String,
}

fn main() {
    let args = Cli::from_args();
    if args.only != "" {
        exec_only(args.only);
        std::process::exit(0);
    }
    if args.add != "" {
        exec_add(args.add);
        std::process::exit(0);
    }
    if args.remove != "" {
        exec_remove(args.remove);
        std::process::exit(0);
    }
    if args.full == true {
        exec_full();
        std::process::exit(0);
    }
}

fn exec_only(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut workspace = WorkSpace::new();

    let base_workspace = load_base_file()?;
    for folder in base_workspace.folders.iter() {
        if folder.name == name {
            workspace.folders.push(folder.clone());
            break;
        }
    }
    if workspace.folders.len() > 0 {
        return write_workspace(workspace);
    }
    Ok(())
}
fn exec_add(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut target_workspace = load_target_file()?;

    let base_workspace = load_base_file()?;
    for folder in base_workspace.folders.iter() {
        if folder.name == name {
            target_workspace.folders.push(folder.clone());
            break;
        }
    }
    if target_workspace.folders.len() > 0 {
        return write_workspace(target_workspace);
    }
    Ok(())
}
fn exec_remove(name: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut workspace = load_target_file()?;

    let mut target_idx: i32 = -1;
    for (i, folder) in workspace.folders.iter().enumerate() {
        if folder.name == name {
            target_idx = i as i32;
            break;
        }
    }
    if target_idx != -1 {
        workspace.folders.remove(target_idx as usize);
    }

    if workspace.folders.len() > 0 {
        return write_workspace(workspace);
    }
    Ok(())
}
fn exec_full() -> Result<(), Box<dyn std::error::Error>> {
    let workspace = load_base_file()?;
    if workspace.folders.len() > 0 {
        return write_workspace(workspace);
    }
    Ok(())
}

fn load_base_file() -> Result<WorkSpace, Box<dyn std::error::Error>> {
    let config = CONFIG_DATA.lock()?;
    load_file(&config.base)
}

fn load_target_file() -> Result<WorkSpace, Box<dyn std::error::Error>> {
    let config = CONFIG_DATA.lock()?;
    load_file(&config.target)
}

fn load_file(path: &String) -> Result<WorkSpace, Box<dyn std::error::Error>> {
    let reader = BufReader::new(File::open(path)?);
    let workspace: WorkSpace = serde_json::from_reader(reader)?;
    Ok(workspace)
}

fn write_workspace(workspace: WorkSpace) -> Result<(), Box<dyn std::error::Error>> {
    let config = CONFIG_DATA.lock()?;
    let mut file = File::create(&config.target).unwrap();
    write!(file, "{}", serde_json::to_string(&workspace)?);
    file.flush()?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorkSpace {
    folders: Vec<FolderSetting>,
    settings: Option<HashMap<String, String>>,
}

impl WorkSpace {
    pub fn new() -> Self {
        Self {
            folders: vec![],
            settings: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FolderSetting {
    path: String,
    name: String,
}

#[derive(Debug, StructOpt)]
struct Cli {
    #[structopt(short = "o", long = "only", default_value = "")]
    only: String,
    #[structopt(short = "a", long = "add", default_value = "")]
    add: String,
    #[structopt(short = "r", long = "remove", default_value = "")]
    remove: String,
    #[structopt(short = "f", long = "full")]
    full: bool,
}

#[test]
fn test_only() {
    exec_only("work".to_string());
}

#[test]
fn test_add() {
    exec_add("target".to_string());
}

#[test]
fn test_remove() {
    exec_remove("work3".to_string());
}

#[test]
fn test_full() {
    exec_full();
}
