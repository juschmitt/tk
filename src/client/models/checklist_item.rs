use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChecklistItem {
    pub id: String,
    pub title: String,
    pub status: i32,
    pub completed_time: String,
    pub is_all_day: bool,
    pub sort_order: i64,
    pub start_date: String,
    pub time_zone: String,
}