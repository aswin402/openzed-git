#![allow(unused)]

use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskEntry {
    pub label: String,
    pub command: String,
    #[serde(rename = "use_new_terminal")]
    pub use_new_terminal: Option<bool>,
    #[serde(rename = "allow_concurrent_runs")]
    pub allow_concurrent_runs: Option<bool>,
    pub reveal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZedTasks {
    #[serde(rename = "tasks")]
    pub tasks: Vec<TaskEntry>,
}

impl ZedTasks {
    pub fn load(path: &Path) -> Result<Self, io::Error> {
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let trimmed = content.trim();

            // Try parsing as direct array first
            if trimmed.starts_with('[') {
                let tasks: Vec<TaskEntry> = serde_json::from_str(trimmed)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                return Ok(Self { tasks });
            }

            // Try parsing as {"tasks": [...]} wrapper
            if let Ok(zed_tasks) = serde_json::from_str::<ZedTasks>(trimmed) {
                return Ok(zed_tasks);
            }

            // Otherwise return empty
            Ok(Self { tasks: vec![] })
        } else {
            Ok(Self { tasks: vec![] })
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), io::Error> {
        // Save as direct array (Zed expects array format)
        let content = serde_json::to_string_pretty(&self.tasks)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        fs::write(path, content)
    }

    pub fn tasks(&self) -> &[TaskEntry] {
        &self.tasks
    }

    pub fn tasks_mut(&mut self) -> &mut Vec<TaskEntry> {
        &mut self.tasks
    }

    pub fn merge_tasks(&mut self, new_tasks: Vec<crate::core::tasks::Task>) {
        let existing_labels: std::collections::HashSet<_> =
            self.tasks.iter().map(|t| t.label.clone()).collect();

        for task in new_tasks {
            if !existing_labels.contains(&task.label) {
                self.tasks.push(TaskEntry {
                    label: task.label,
                    command: task.command,
                    use_new_terminal: Some(task.use_new_terminal),
                    allow_concurrent_runs: Some(task.allow_concurrent_runs),
                    reveal: Some(task.reveal),
                });
            }
        }
    }
}
