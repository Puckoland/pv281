use serde::{Deserialize, Serialize};

extern crate savefile;
use savefile::prelude::*;
#[macro_use]
extern crate savefile_derive;

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandType {
    Get,
    Insert,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Command {
    pub cmd: CommandType,
    pub key: usize,
    pub value: Option<Receipt>,
}

impl Command {
    pub fn create_insert_command_from(key: usize, value: Receipt) -> Command {
        Command {
            cmd: CommandType::Insert,
            key,
            value: Some(value),
        }
    }

    pub fn create_get_command_from(key: usize) -> Command {
        Command {
            cmd: CommandType::Get,
            key,
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Savefile)]
pub struct Receipt {
    pub value: i32,
    pub description: Option<String>,
}

impl Receipt {
    pub fn new(value: i32, description: Option<&str>) -> Self {
        let description: Option<String> = match description {
            Some(description) => Some(description.to_string()),
            None => None,
        };
        Self { value, description }
    }

    pub fn save(&self, key: &usize) {
        match save_file(&format!("{}.bin", key), 0, self) {
            Ok(_) => println!("Data saved."),
            Err(err) => println!("{}", err),
        }
    }
    
    pub fn load(key: &usize) -> Option<Receipt> {
        let data = match load_file(&format!("{}.bin", key), 0) {
            Ok(data) => Some(data),
            _ => None,
        };
    
        data
    }
}
