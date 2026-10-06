mod cli;
mod commands;
mod repo;

use clap::Parser;
use cli::Cli;
use repo::Repo;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let repo_opt = Repo::find(&current_dir);

    match cli.command {
        Some(cmd) => commands::run(repo_opt, cmd, cli.all),
        None => {
            use clap::CommandFactory;
            let _ = Cli::command().print_help();
            println!();
            ExitCode::SUCCESS
        }
    }
}
