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
                        oauth::start_oauth_process(client_id.as_str(), client_secret.as_str()).unwrap();
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
