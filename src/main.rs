use crate::cli::{AuthCommands, Cli, Commands, ProjectCommands, TaskCommands};
use crate::client::models::project::ProjectList;
use crate::client::models::task::{Task, TaskList};
use clap::Parser;
use client::TickTickClient;
use std::io;

mod cli;
mod client;
mod file;
mod oauth;

fn main() -> io::Result<()> {
    match Cli::parse().command {
        Commands::Auth(args) => match args.command {
            AuthCommands::Login { id, secret } => {
                let auth_token = oauth::authenticate(id.as_str(), secret.as_str());
                match auth_token {
                    Ok(auth_token) => {
                        file::store_auth_token(auth_token)?;
                        println!("Authentication complete!")
                    }
                    Err(error) => {
                        eprintln!("Authentication failed! Cause: {:?}", error)
                    }
                }
            }
            AuthCommands::Logout => {
                file::store_auth_token("".to_string())?;
            }
            AuthCommands::Token => {
                let auth_token = file::read_auth_token()?;
                println!("Token: {}", auth_token);
            }
        },
        Commands::Project(args) => match args.command {
            ProjectCommands::List => {
                let client = TickTickClient::new()?;
                let projects = ProjectList(client.list_projects()?);
                println!("{}", projects);
            }
            ProjectCommands::View { id } => {
                let client = TickTickClient::new()?;
                let id = if let Some(id) = id {
                    id
                } else {
                    let projects = client.list_projects()?;
                    let project_list = ProjectList(projects);
                    prompt_user_to_select_project(&project_list)?
                };
                let project_data = client.get_project_data(id.as_str())?;
                println!("{}", project_data);
            }
            ProjectCommands::Set { id } => {
                if let Some(id) = id {
                    file::store_active_project_id(id)?;
                } else {
                    let client = TickTickClient::new()?;
                    let projects = ProjectList(client.list_projects()?);
                    let id = prompt_user_to_select_project(&projects)?;
                    file::store_active_project_id(id)?;
                }
            }
            ProjectCommands::Unset => {
                file::remove_active_project_id()?;
            }
            ProjectCommands::New { name } => {
                let client = TickTickClient::new()?;
                let project = client.create_project(name.as_str())?;
                println!("Project created: {}", project);
            }
            ProjectCommands::Delete { id } => {
                let client = TickTickClient::new()?;
                if let Some(id) = id { 
                    client.delete_project(id.as_str())?;
                } else {
                    let projects = ProjectList(client.list_projects()?);
                    let id = prompt_user_to_select_project(&projects)?;
                    client.delete_project(id.as_str())?;
                }
            }
        },
        Commands::Task(args) => match args.command {
            TaskCommands::List => {
                let project_id = get_project_id()?;
                let client = TickTickClient::new()?;
                let tasks = TaskList(client.list_tasks(&project_id)?);
                println!("{}", tasks);
            }
            TaskCommands::View { id } => {
                let project_id = get_project_id()?;
                let task_id = get_task_id(id, &project_id)?;
                let client = TickTickClient::new()?;
                let task = client.get_task(&project_id, &task_id)?;
                println!("{}", task);
            }
            TaskCommands::New { name } => {
                let client = TickTickClient::new()?;
                let project_id = file::read_active_project_id().ok();
                let task = client.create_task(project_id, name.as_str())?;
                println!("Task created: \n{}", task);
            }
            TaskCommands::Edit { id } => {
                let project_id = get_project_id()?;
                let task_id = get_task_id(id, &project_id)?;
                let client = TickTickClient::new()?;
                let task = client.get_task(&project_id, &task_id)?;
                let updated_task = update_task_with_editor(&task)?;
                let task = client.update_task(project_id, updated_task)?;
                println!("Task updated: \n{}", task);
            }
            TaskCommands::Delete { id } => {
                let project_id = get_project_id()?;
                let task_id = get_task_id(id, &project_id)?;
                let client = TickTickClient::new()?;
                if let Err(err) = client.delete_task(&project_id, &task_id) {
                    eprintln!("Failed to delete task. Cause: {:?}", err);
                } else {
                    println!("Task deleted.");
                }
            }
        },
    }
    Ok(())
}

fn update_task_with_editor(task: &Task) -> io::Result<Task> {
    let task_str = serde_json::to_string_pretty(&task)?;
    let updated_task_str = edit::edit(task_str.as_str())?;
    serde_json::from_str(updated_task_str.as_str())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn get_project_id() -> io::Result<String> {
    if let Ok(project_id) = file::read_active_project_id() {
        Ok(project_id)
    } else {
        let client = TickTickClient::new()?;
        let projects = ProjectList(client.list_projects()?);
        prompt_user_to_select_project(&projects)
    }
}

fn prompt_user_to_select_project(projects: &ProjectList) -> io::Result<String> {
    println!("{}", projects);
    println!("Choose a project by entering its index:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index = input.trim().parse::<usize>();
    if let Ok(index) = index {
        Ok(projects.0.get(index).unwrap().id.clone())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid project index!"))
    }
}

fn get_task_id(id: Option<String>, project_id: &str) -> io::Result<String> {
    if let Some(id) = id {
        Ok(id)
    } else {
        let client = TickTickClient::new()?;
        let tasks = TaskList(client.list_tasks(project_id)?);
        prompt_user_to_select_task(&tasks)
    }
}

fn prompt_user_to_select_task(tasks: &TaskList) -> io::Result<String> {
    println!("{}", tasks);
    println!("Choose a task by entering its index:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index = input.trim().parse::<usize>();
    if let Ok(index) = index {
        Ok(tasks.0.get(index).unwrap().id.clone())
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid task index!"))
    }
}
