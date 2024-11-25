use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub color: Option<String>,
    pub sort_order: Option<i64>,
    pub closed: Option<bool>,
    pub group_id: Option<String>,
    pub view_mode: Option<String>,
    pub permission: Option<String>,
    pub kind: Option<String>,
}

pub struct ProjectList(pub Vec<Project>);

impl Display for ProjectList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (idx, project) in self.0.iter().enumerate() {
            writeln!(f, "({}) {}", idx, project)?;
        }
        Ok(())
    }
}

impl Display for Project {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0:<30}| ID: {1:<10} |", self.name, self.id)
    }
}