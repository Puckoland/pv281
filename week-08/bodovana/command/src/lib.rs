use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
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
}
