use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tk")]
#[command(about = "Unofficial ticktick.com CLI")]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Login, logout, and manage tokens
    Auth(AuthArgs),
    /// Manage projects
    Project(ProjectArgs),
    /// Manage tasks
    Task,
}

#[derive(Debug, Args)]
struct AuthArgs {
    #[command(subcommand)]
    command: AuthCommands,
}

#[derive(Debug, Subcommand)]
enum AuthCommands {
    /// Login to ticktick.com and store the token for future requests
    Login {
        #[arg(short, long, required = true)]
        client_id: String,
        #[arg(short, long, required = true)]
        client_secret: String 
    },
    /// Logout and remove the stored token
    Logout,
    /// Show the stored token
    Token,
}

#[derive(Debug, Args)]
struct ProjectArgs {
    #[command(subcommand)]
    command: ProjectCommands,
}

#[derive(Debug, Subcommand)]
enum ProjectCommands {
    /// List all projects
    List,
    /// View a project and its tasks
    View { id: String },
    /// Set a project as the active
    Set { id: String },
    /// Create a new project
    Create { name: String },
    /// Delete a project
    Delete { id: String },
}