use crate::cli::{AuthCommands, Cli, Commands, ProjectCommands};
use crate::client::models::project::ProjectList;
use clap::Parser;
use client::TickTickClient;
use std::fmt::{Display, Formatter};
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
            ProjectCommands::Create { .. } => {}
            ProjectCommands::Delete { .. } => {}
        },
        Commands::Task => {}
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
