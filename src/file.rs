use directories::UserDirs;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

pub fn store_auth_token(auth_token: String) -> std::io::Result<()> {
    let tk_dir = get_tk_dir()?;
    let auth_token_file = tk_dir.join("auth");
    std::fs::write(auth_token_file, auth_token)
}

pub fn read_auth_token() -> std::io::Result<String> {
    let tk_dir = get_tk_dir()?;
    let auth_token_file = tk_dir.join("auth");
    std::fs::read_to_string(auth_token_file)
}

pub fn store_active_project_id(id: String) -> std::io::Result<()> {
    let tk_dir = get_tk_dir()?;
    let active_project_file = tk_dir.join("active_project");
    std::fs::write(active_project_file, id)
}

pub fn read_active_project_id() -> std::io::Result<String> {
    let tk_dir = get_tk_dir()?;
    let active_project_file = tk_dir.join("active_project");
    std::fs::read_to_string(active_project_file)
}

fn get_tk_dir() -> std::io::Result<PathBuf> {
    let home_dir = get_home_dir()?;
    let tk_dir = home_dir.join(".tk");
    if !tk_dir.exists() {
        std::fs::create_dir(&tk_dir)?;
    }
    Ok(tk_dir)
}

fn get_home_dir() -> std::io::Result<PathBuf> {
    if let Some(user_dirs) = UserDirs::new() {
        Ok(user_dirs.home_dir().to_owned())
    } else {
        Err(Error::new(ErrorKind::NotFound, "Home dir not found"))
    }
}
