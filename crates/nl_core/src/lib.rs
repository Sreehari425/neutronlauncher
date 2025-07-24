pub mod init;

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

/// A macro to log messages with a colored [LAUNCHER] prefix.

#[macro_export]
macro_rules! launcher_log {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [LAUNCHER] in yellow and bold, then your message
        println!("{} {}", "[LAUNCHER]".yellow().bold(), format!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        use $crate::colored::Colorize;
        // Print [ERROR] in red and bold, then your message
        println!("{} {}", "[ERROR]".red().bold(), format!($($arg)*));
    }};
}
