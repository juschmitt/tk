use std::fmt::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub id: String,
    pub name: String,
    pub project_id: Option<String>,
    pub sort_order: Option<i64>,
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:<30}", self.name)
    }
}