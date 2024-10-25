use crate::config::Config;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Debug)]
pub struct T0 {
    config: Config,
    todo_list: Vec<Todo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Todo {
    pub id: u64,
    pub content: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub ddl: Option<u64>,
    pub completed: bool,
    pub created_at: u64,
    pub completed_at: Option<u64>,
}

impl T0 {
    pub fn new(config: Config) -> Self {
        let todo_file_path = config.todo_file.clone();
        Self {
            config,
            todo_list: read_to_string(todo_file_path)
                .or_else(|err| {
                    // check if file exists
                    if err.kind() != std::io::ErrorKind::NotFound {
                        Err(format!("Cannot open todo file {:?}", err))
                    } else {
                        Ok("[]".to_string())
                    }
                })
                .map(|content| serde_json::from_str::<Vec<Todo>>(&content))
                .unwrap()
                .unwrap(),
        }
    }

    pub fn display(&self) {
        self.todo_list.iter().for_each(|todo| {
            println!("{:?}", todo);
        });
    }

    pub fn add(&mut self, mut todo: Todo) {
        todo.id = self.todo_list.len() as u64;
        self.todo_list.push(todo);
        self.sync();
    }

    fn sync(&self) {
        println!("sync??? TODO:");
    }
}
