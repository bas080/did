use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "did",
    version = "0.1.0",
    about = "filesystem-native issue tracker with hooks for defining local software factories",
    disable_help_subcommand = true
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
    #[command(alias = "list")]
    Status {
        /// Optional subtree path
        path: Option<PathBuf>,

        /// Display tasks in a tree structure
        #[arg(short = 't', long = "tree")]
        tree: bool,

        /// List blocked tasks
        #[arg(short = 'b', long = "blocked")]
        blocked: bool,
    },

    /// Search task paths and task file contents for QUERY
    #[command(alias = "search")]
    Query {
        /// Search query string
        query: String,

        /// Optional subtree path
        path: Option<PathBuf>,

        /// Print 1-based line numbers for matching content snippets
        #[arg(short = 'n', long = "line-number")]
        line_number: bool,

        /// Display search results in a tree structure
        #[arg(short = 't', long = "tree")]
        tree: bool,
    },

    /// Print task contents at PATH (requires sub-items done unless -a)
    Show {
        /// Task file path
        path: PathBuf,
    },

    /// Mark PATH as resolved (hides it; fails if sub-items remain open)
    #[command(alias = "done")]
    Close {
        /// Task file path or directory
        path: PathBuf,

        /// Mark all open tasks inside directory resolved recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Mark a resolved PATH as open/undone (removes leading dot; updates symlinks)
    #[command(alias = "undone")]
    Open {
        /// Resolved task file path or directory
        path: PathBuf,

        /// Reopen all resolved tasks inside directory recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Symlink TARGET into DEST directory using TARGET's basename
    #[command(alias = "link", alias = "ln")]
    Blocks {
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

    /// Remove a task file or directory at PATH
    #[command(alias = "remove")]
    Rm {
        /// Task file or directory path to remove
        path: PathBuf,

        /// Remove directories and their contents recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// View task Git history and lifecycle events
    Log {
        /// Optional task path or directory to filter history
        path: Option<PathBuf>,

        /// Show events since specified date/time
        #[arg(long)]
        since: Option<String>,

        /// Show events until specified date/time
        #[arg(long)]
        until: Option<String>,
    },

    /// Validate repository health and structure
    Test,

    /// Print help information, topic guides, or run help lifecycle hooks
    Help {
        /// Optional topic or subcommand name
        topic: Option<String>,
    },

    /// Generate shell completion scripts (e.g., bash)
    #[command(hide = true)]
    Autocomplete {
        /// Shell name (e.g., bash)
        shell: String,
    },
}
