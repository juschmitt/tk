use std::fmt::Display;
use serde::{Deserialize, Serialize};
use crate::client::models::checklist_item::ChecklistItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub project_id: Option<String>,
    pub is_all_day: Option<bool>,
    pub completed_time: Option<String>,
    pub content: Option<String>,
    pub desc: Option<String>,
    pub due_date: Option<String>,
    pub items: Option<Vec<ChecklistItem>>,
    pub priority: Option<i32>,
    pub reminders: Option<Vec<String>>,
    pub repeat_flag: Option<String>,
    pub sort_order: Option<i64>,
    pub start_date: Option<String>,
    pub status: Option<i32>,
    pub time_zone: Option<String>,
}

pub struct TaskList(pub Vec<Task>);

impl Display for TaskList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, task) in self.0.iter().enumerate() {
            writeln!(f, "({}) {}", idx, task)?;
        }
        Ok(())
    }
}

impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:<30}| ID: {1:<10} |", self.title, self.id)
    }
}