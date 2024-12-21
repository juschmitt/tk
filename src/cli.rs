use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "tk")]
#[command(about = "Unofficial ticktick.com CLI")]
#[command(flatten_help = true)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Login, logout, and manage tokens
    #[command(flatten_help = true)]
    Auth(AuthArgs),
    /// Manage projects
    #[command(flatten_help = true)]
    Project(ProjectArgs),
    /// Manage tasks. All commands work with the active project. Use `tk project set` to set a project as active.
    #[command(flatten_help = true)]
    Task(TaskArgs),
}

#[derive(Debug, Args)]
pub struct AuthArgs {
    #[command(subcommand)]
    pub(crate) command: AuthCommands,
}

#[derive(Debug, Subcommand)]
pub enum AuthCommands {
    /// Login to ticktick.com and store the token for future requests
    #[command(arg_required_else_help = true)]
    Login {
        #[arg(short, long, required = true)]
        id: String,
        #[arg(short, long, required = true)]
        secret: String
    },
    /// Logout and remove the stored token
    Logout,
    /// Show the stored token
    Token,
}

#[derive(Debug, Args)]
pub struct ProjectArgs {
    #[command(subcommand)]
    pub command: ProjectCommands,
}

#[derive(Debug, Subcommand)]
pub enum ProjectCommands {
    /// List all projects.
    List,
    /// View a project and its tasks. If no id is provided, choose the project to view interactively.
    View { 
        id: Option<String> 
    },
    /// Set a project as the active. If no id is provided, choose the project to set as active interactively.
    Set {
        id: Option<String>
    },
    /// Unset the active project.
    Unset,
    /// Create a new project.
    New {
        #[arg(short, long, required = true)]
        name: String 
    },
    /// Delete a project. If no id is provided, choose the project to delete interactively.
    Delete { 
        id: Option<String> 
    },
}

#[derive(Debug, Args)]
pub struct TaskArgs {
    #[command(subcommand)]
    pub command: TaskCommands,
}

#[derive(Debug, Subcommand)]
pub enum TaskCommands {
    /// List all undone tasks of the active project.
    List,
    /// View a task from the active project. 
    /// If no project is set, choose the project interactively.
    /// If no id is provided, choose the task to view interactively.
    View { 
        id: Option<String> 
    },
    /// Create a new task in the active project. If no project is set, the task will be added to inbox.
    New {
        #[arg(short, long, required = true)]
        name: String 
    },
    /// Modify a task. If no id is provided, choose the task to modify interactively.
    Edit { 
        id: Option<String> 
    },
    /// Delete a task from the active project.
    /// If no project is set, choose the project interactively.
    /// If no id is provided, choose the task to delete interactively.
    Delete { 
        id: Option<String> 
    },
}