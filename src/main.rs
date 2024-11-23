use clap::Parser;
use cli::Cli;

mod cli;
mod oauth;

fn main() {
    // let accessToken = oauth::start_oauth_process().unwrap();
    let args = cli::Cli::parse();
}
