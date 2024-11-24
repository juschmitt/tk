use serde::{Deserialize, Serialize};
use crate::client::models::checklist_item::ChecklistItem;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub project_id: String,
    pub title: String,
    pub is_all_day: bool,
    pub completed_time: String,
    pub content: String,
    pub desc: String,
    pub due_date: String,
    pub items: Vec<ChecklistItem>,
    pub priority: i32,
    pub reminders: Vec<String>,
    pub repeat_flag: String,
    pub sort_order: i64,
    pub start_date: Option<String>,
    pub status: i32,
    pub time_zone: Option<String>,
}