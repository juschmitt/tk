use crate::cli::{AuthCommands, Cli, Commands, ProjectCommands, TaskCommands};
use crate::client::models::project::ProjectList;
use clap::Parser;
use client::TickTickClient;
use std::io;
use crate::client::models::task::TaskList;

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
                    let client = TickTickClient::new()?;
                    client.delete_project(id.as_str())?;
                }
            },
        },
        Commands::Task(args) => match args.command {
            TaskCommands::List => {
                let client = TickTickClient::new()?;
                let project_id = file::read_active_project_id()?;
                let tasks = TaskList(client.list_tasks(project_id.as_str())?);
                println!("{}", tasks);
            }
            TaskCommands::View { id } => {
                let project_id = if let Ok(project_id) = file::read_active_project_id() {
                    project_id
                } else {
                    let client = TickTickClient::new()?;
                    let projects = ProjectList(client.list_projects()?);
                    prompt_user_to_select_project(&projects)?
                };
                let task_id = if let Some(id) = id {
                    id
                } else {
                    let client = TickTickClient::new()?;
                    let tasks = TaskList(client.list_tasks(project_id.as_str())?);
                    prompt_user_to_select_task(&tasks)?
                };
                let client = TickTickClient::new()?;
                let task = client.get_task(project_id, task_id)?;
                println!("{}", task);
            }
            TaskCommands::New { name } => {
                let client = TickTickClient::new()?;
                let project_id = file::read_active_project_id().ok();
                let task = client.create_task(project_id, name.as_str())?;
                println!("Task created: \n{}", task);
            }
            TaskCommands::Edit { .. } => {}
            TaskCommands::Delete { .. } => {}
        }
    }
    Ok(())
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
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
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
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid input"))
    }
}