use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "did",
    version = "0.1.0",
    about = "file-system-native issue and dependency tracker"
)]
pub struct Cli {
    /// Include resolved (hidden) tasks and blocked nodes
    #[arg(short = 'a', long = "all", global = true)]
    pub all: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Create a task node or nested issue at PATH
    Add {
        /// Task path
        path: PathBuf,

        /// Message / content for the task
        #[arg(short = 'm', long = "message")]
        message: Option<String>,
    },

    /// List actionables (leaf nodes with all sub-items done)
    Status {
        /// Optional subtree path
        path: Option<PathBuf>,
    },

    /// Search task paths and task file contents for QUERY
    #[command(alias = "search")]
    Query {
        /// Search query string
        query: String,

        /// Optional subtree path
        path: Option<PathBuf>,
    },

    /// Print task contents at PATH (requires sub-items done unless -a)
    Show {
        /// Task file path
        path: PathBuf,
    },

    /// Mark PATH as resolved (hides it; fails if sub-items remain open)
    Done {
        /// Task file path
        path: PathBuf,
    },

    /// Mark a resolved PATH as open/undone (removes leading dot; updates symlinks)
    Undone {
        /// Resolved task file path
        path: PathBuf,
    },

    /// Symlink TARGET into DEST directory using TARGET's basename
    #[command(alias = "ln")]
    Link {
        /// Target path
        target: PathBuf,
        /// Destination directory path
        dest: PathBuf,
    },

    /// Move or rename a task file or directory at OLD_PATH to NEW_PATH
    #[command(alias = "move")]
    Mv {
        /// Source task path
        old_path: PathBuf,
        /// Target task path
        new_path: PathBuf,
    },

    /// Validate repository health and structure
    Test,

    /// Generate shell completion scripts (e.g., bash)
    #[command(hide = true)]
    Autocomplete {
        /// Shell name (e.g., bash)
        shell: String,
    },
}
