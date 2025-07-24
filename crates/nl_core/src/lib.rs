pub mod init;
pub mod macros;

/// Re-export colored for use in macros
pub extern crate colored;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GameInstance {
    pub game_name: String,
    pub game_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub instances: HashMap<String, GameInstance>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            instances: HashMap::new(),
        }
    }
}
