use serde::{Deserialize, Serialize};
use crate::client::models::column::Column;
use crate::client::models::project::Project;
use crate::client::models::task::Task;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectData {
    pub project: Project,
    pub tasks: Vec<Task>,
    pub columns: Vec<Column>,
}