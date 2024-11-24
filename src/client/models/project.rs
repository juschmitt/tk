use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub sort_order: i64,
    pub closed: bool,
    pub group_id: String,
    pub view_mode: String,
    pub permission: String,
    pub kind: String,
}