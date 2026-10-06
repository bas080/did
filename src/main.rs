mod cli;
mod commands;
mod repo;

use clap::Parser;
use cli::Cli;
use repo::Repo;
use std::env;
use std::process::ExitCode;

fn print_welcome_instructions() {
    println!(
        "did - file-system-native issue and dependency tracker\n\n\
USAGE GUIDANCE & AI AGENT WORKFLOW:\n\
  1. Initialize state tracker in project root:\n\
         $ did init\n\n\
  2. Create a task or issue (adds file under .did/):\n\
         $ did add backend/auth/jwt.md -m \"Implement JWT token validation\"\n\n\
  3. List actionable tasks (unblocked leaf tasks ready to work on):\n\
         $ did status\n\n\
  4. Inspect task contents (prints parent context top-down before task content):\n\
         $ did show backend/auth/jwt.md\n\n\
  5. Link dependencies (symlinks TARGET into DEST directory):\n\
         $ did link backend/auth/jwt.md frontend/ui\n\n\
  6. Resolve/complete a task (prefixes file with '.' and updates symlinks):\n\
         $ did done backend/auth/jwt.md\n\n\
  7. List all tasks including blocked and completed:\n\
         $ did status -a\n\n\
For command flags and details, run: did --help"
    );
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let repo_opt = Repo::find(&current_dir);

    match cli.command {
        Some(cmd) => commands::run(repo_opt, cmd, cli.all),
        None => {
            print_welcome_instructions();
            ExitCode::SUCCESS
        }
    }
}
