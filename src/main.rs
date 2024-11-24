use std::io;
use clap::Parser;
use crate::cli::{AuthCommands, Cli, Commands, ProjectCommands};

mod cli;
mod oauth;
mod file;

fn main() -> io::Result<()> {
        match Cli::parse().command {
            Commands::Auth(args) => {
                match args.command {
                    AuthCommands::Login { client_id, client_secret } => {
                        let auth_token = oauth::authenticate(client_id.as_str(), client_secret.as_str());
                        match auth_token {
                            Ok(auth_token) => { file::store_auth_token(auth_token)?; }
                            Err(error) => { eprintln!("Authentication failed! Cause: {:?}", error) }
                        }
                    }
                    AuthCommands::Logout => {
                        file::store_auth_token("".to_string())?;
                    }
                    AuthCommands::Token => {
                        let auth_token = file::read_auth_token()?;
                        println!("Token: {}", auth_token);
                    }
                }
            }
            Commands::Project(args) => {
                match args.command {
                    ProjectCommands::List => {}
                    ProjectCommands::View { .. } => {}
                    ProjectCommands::Set { .. } => {}
                    ProjectCommands::Create { .. } => {}
                    ProjectCommands::Delete { .. } => {}
                }
            }
            Commands::Task => {}
        }
    Ok(())
}
