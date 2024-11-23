use std::io;
use clap::Parser;
use crate::cli::{AuthCommands, Cli, Commands, ProjectCommands};

mod cli;
mod oauth;

fn main() -> io::Result<()> {
        match Cli::parse().command {
            Commands::Auth(args) => {
                match args.command {
                    AuthCommands::Login { client_id, client_secret } => {
                        let auth_token = oauth::authenticate(client_id.as_str(), client_secret.as_str());
                        match auth_token {
                            Ok(auth_token) => { store_auth_token(auth_token); }
                            Err(error) => { eprintln!("Authentication failed! Cause: {:?}", error) }
                        }
                    }
                    AuthCommands::Logout => {}
                    AuthCommands::Token => {}
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

fn store_auth_token(auth_token: String) {
    todo!("Store the auth token in a file");
}