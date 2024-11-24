use crate::client::models::project::Project;

pub mod models;

const BASE_URL: &str = "api.ticktick.com/open/v1/";

/// Load all projects for user
pub fn list_projects() -> std::io::Result<Vec<Project>> {
    todo!("Load all projects for user")
}

