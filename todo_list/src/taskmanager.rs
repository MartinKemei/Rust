use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        TaskManager { tasks: Vec::new() }
    }

    pub fn load_from_file(filename: &str) -> Result<TaskManager, std::io::Error> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let task_manager: TaskManager = serde_json::from_reader(reader)?;
        Ok(task_manager)
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }

    // ... other methods for adding, listing, completing, and deleting tasks
}