use serde_derive::{Deserialize, Serialize};
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemModel {
    pub name: Option<String>,
    pub path: String,
}
impl ItemModel {
    pub fn as_string(&self) -> String {
        if let Some(name) = &self.name {
            name.clone() + " " + &self.path
        } else {
            self.path.clone()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    pub bookmarks: Vec<ItemModel>,
}

pub struct Db {
    path: PathBuf,
    pub data: Model,
}
impl Db {
    pub fn new() -> Self {
        let home = home_dir().unwrap();
        let path = home.join(".dirbkm");
        Self {
            path,
            data: Model {
                bookmarks: Vec::new(),
            },
        }
    }
    pub fn read_file(&mut self) {
        if let Ok(mut file) = std::fs::File::open(&self.path) {
            let mut data = String::new();

            if let Ok(_) = file.read_to_string(&mut data) {
                let toml: Model = toml::from_str(&data).unwrap();
                self.data = toml;
            }
        }
    }
    pub fn write_file(&mut self) {
        let toml = toml::to_string(&self.data).unwrap();

        let mut file = std::fs::File::create(&self.path).unwrap();

        file.write(toml.as_bytes()).unwrap();
    }
    pub fn add(&mut self, name: Option<String>, path: String) {
        self.data.bookmarks.push(ItemModel { name, path });
    }
}

fn home_dir() -> Option<PathBuf> {
    let home = std::env::var_os("HOME").and_then(|h| if h.is_empty() { None } else { Some(h) });
    if let Some(home) = home {
        Some(PathBuf::from(home))
    } else {
        None
    }
}

// pub fn get_config() {
//     let config_dir = config_dir();

//     let (toml_str, style_str) = if let Some(config_dir) = config_dir {
//         let path = config_dir.join(".dirbkm");
//         let toml_str =
//             if let Ok(file) = std::fs::read_to_string(&bar_config_dir.join("config.toml")) {
//                 file
//             } else {
//                 default_config.into()
//             };
//     };
// }
