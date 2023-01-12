use std::env::args;
use std::fs::read_to_string;
use std::process::exit;

use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    host: String,
    connections: Vec<String>
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            host: String::from(""),
            connections: Vec::new()
        }
    }

    pub fn load(&mut self) -> () {
        let file_location = match args().nth(1) {
            Some(file_location) => file_location,
            None => {
                eprintln!("[ ERROR ] No file location provided.");
                exit(1);
            }
        };

        let file_content = match read_to_string(&file_location) {
            Ok(file_content) => file_content,
            Err(..) => {
                eprintln!("[ ERROR ] Invalid file location provided.");
                exit(1);
            }
        };

        let settings: Settings = match toml::from_str(&file_content) {
            Ok(settings) => settings,
            Err(..) => {
                eprintln!("[ ERROR ] Invalid file content provided.");
                exit(1);
            }
        };

        self.host = settings.host;
        self.connections = settings.connections;
    }

    pub fn debug(&self) -> () {
        println!("[ DEBUG ] host: {:?}", self.host);
        println!("[ DEBUG ] connections: {:?}", self.connections);
    }
}
