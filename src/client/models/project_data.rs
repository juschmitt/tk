use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::client::models::column::Column;
use crate::client::models::project::Project;
use crate::client::models::task::{Task, TaskList};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectData {
    pub project: Project,
    pub tasks: Vec<Task>,
    pub columns: Vec<Column>,
}

impl Display for ProjectData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.project)?;
        writeln!(f, "Tasks:")?;
        writeln!(f, "{}", TaskList(&self.tasks))?;
        writeln!(f, "Columns:")?;
        for column in &self.columns {
            writeln!(f, "{}", column)?;
        }
        Ok(())
    }
}