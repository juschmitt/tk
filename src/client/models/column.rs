use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub project_id: String,
    pub sort_order: i64,
}