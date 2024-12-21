use crate::client::models::project::Project;
use crate::client::models::project_data::ProjectData;
use crate::client::models::task::Task;
use crate::file::read_auth_token;
use std::io::{Error, ErrorKind};
use reqwest::blocking::Response;

pub mod models;

const BASE_URL: &str = "https://api.ticktick.com/open/v1/";

#[derive(Debug)]
pub struct TickTickClient {
    http_client: reqwest::blocking::Client,
    auth_header: String,
    base_url: String,
}

impl TickTickClient {
    pub fn new() -> std::io::Result<Self> {
        let auth_header = format!("Bearer {}", read_auth_token()?);
        let http_client = reqwest::blocking::Client::new();
        Ok(TickTickClient {
            http_client,
            auth_header,
            base_url: BASE_URL.to_string(),
        })
    }
    /// Load all projects for user
    pub fn list_projects(&self) -> std::io::Result<Vec<Project>> {
        let response = self
            .http_client
            .get(format!("{}{}", self.base_url, "project"))
            .header("Authorization", &self.auth_header)
            .send()
            .unwrap(); // todo: handle error cases.

        let body = response.text().unwrap();
        let projects: Vec<Project> = serde_json::from_str(&body)?;
        Ok(projects)
    }

    /// Load a single project by id
    pub fn get_project(&self, id: &str) -> std::io::Result<Project> {
        let response = self
            .http_client
            .get(format!("{}{}{}", self.base_url, "project/", id))
            .header("Authorization", &self.auth_header)
            .send()
            .unwrap(); // todo: handle error cases.

        let body = response.text().unwrap();
        let project: Project = serde_json::from_str(&body)?;
        Ok(project)
    }

    /// Load project data by id
    pub fn get_project_data(&self, id: &str) -> std::io::Result<ProjectData> {
        let response = self
            .http_client
            .get(format!("{}{}{}{}", self.base_url, "project/", id, "/data"))
            .header("Authorization", &self.auth_header)
            .send()
            .unwrap(); // todo: handle error cases.

        let body = response.text().unwrap();
        let project_data: ProjectData = serde_json::from_str(&body)?;
        Ok(project_data)
    }

    /// Create a new project with name
    pub fn create_project(&self, name: &str) -> std::io::Result<Project> {
        let response = self
            .http_client
            .post(format!("{}{}", self.base_url, "project"))
            .header("Authorization", &self.auth_header)
            .json(&serde_json::json!({ "name": name }))
            .send()
            .unwrap(); // todo: handle error cases.

        let body = response.text().unwrap();
        let project: Project = serde_json::from_str(&body)?;
        Ok(project)
    }

    /// Delete a project by id
    pub fn delete_project(&self, id: &str) -> std::io::Result<()> {
        let response = self
            .http_client
            .delete(format!("{}{}{}", self.base_url, "project/", id))
            .header("Authorization", &self.auth_header)
            .send()
            .unwrap(); // todo: handle error cases.

        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Failed to delete project. Response code: HTTP {}",
                    response.status()
                ),
            ))
        }
    }

    /// Load all tasks for a given project
    pub fn list_tasks(&self, project_id: &str) -> std::io::Result<Vec<Task>> {
        self.get_project_data(project_id).map(|data| data.tasks)
    }

    /// Create a new task with title in optional project
    pub fn create_task(self, project: Option<String>, title: &str) -> std::io::Result<Task> {
        let response = self
            .http_client
            .post(format!("{}{}", self.base_url, "task"))
            .header("Authorization", &self.auth_header)
            .json(&serde_json::json!({
                "title": title,
                "projectId": project
            }))
            .send();
        match response {
            Ok(response) => {
                if response.status().is_success() {
                    let body = response.text().unwrap();
                    let task: Task = serde_json::from_str(&body)?;
                    Ok(task)
                } else {
                    Err(Error::new(
                        ErrorKind::Other,
                        format!(
                            "Failed to create task. Response code: HTTP {}",
                            response.status()
                        ),
                    ))
                }
            }
            Err(e) => Err(Error::new(
                ErrorKind::Other,
                format!("Failed to create task. Error: {}", e),
            )),
        }
    }

    /// Load a single task by project and task id
    pub fn get_task(&self, project_id: String, task_id: String) -> std::io::Result<Task> {
        let response = self
            .http_client
            .get(format!("{}{}{}{}{}", self.base_url, "project/", project_id, "/task/", task_id))
            .header("Authorization", &self.auth_header)
            .send()
            .unwrap(); // todo: handle error cases.

        let body = response.text().unwrap();
        let task: Task = serde_json::from_str(&body)?;
        Ok(task)
    }
}
