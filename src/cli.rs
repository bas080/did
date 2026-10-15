use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "did",
    version = "0.1.0",
    about = "filesystem-native note and workflow tracker with hooks for defining local software factories",
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Include resolved (hidden) notes and linked nodes
    #[arg(short = 'a', long = "all", global = true)]
    pub all: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new .did state directory
    Init {
        /// Optional target directory
        path: Option<PathBuf>,
    },

    /// Create a note
    Add {
        /// Optional note title
        title: Option<String>,
    },

    /// List open notes
    #[command(alias = "list")]
    Status {
        /// Optional subtree path
        path: Option<PathBuf>,

        /// Display notes in a tree structure
        #[arg(short = 't', long = "tree")]
        tree: bool,

        /// List blocked notes
        #[arg(short = 'b', long = "blocked")]
        blocked: bool,
    },

    /// Search note paths and note file contents for QUERY
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

    /// Print note contents at PATH
    Show {
        /// Note file path
        path: PathBuf,
    },

    /// Mark PATH as resolved (hides it)
    #[command(alias = "done")]
    Close {
        /// Note file path or directory
        path: PathBuf,

        /// Mark all open notes inside directory resolved recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Mark a resolved PATH as open/undone (removes leading dot; updates symlinks)
    #[command(alias = "undone")]
    Open {
        /// Resolved note file path or directory
        path: PathBuf,

        /// Reopen all resolved notes inside directory recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Symlink TARGET into DEST directory using TARGET's basename
    #[command(alias = "blocks", alias = "ln")]
    Link {
        /// Target path
        target: PathBuf,
        /// Destination directory path
        dest: PathBuf,
    },

    /// Move or rename a note file or directory at OLD_PATH to NEW_PATH
    #[command(alias = "move")]
    Mv {
        /// Source note path
        old_path: PathBuf,
        /// Target note path
        new_path: PathBuf,
    },

    /// Remove a note file or directory at PATH
    #[command(alias = "remove")]
    Rm {
        /// Note file or directory path to remove
        path: PathBuf,

        /// Remove directories and their contents recursively
        #[arg(short = 'r', long = "recursive")]
        recursive: bool,
    },

    /// Interactive fuzzy finder with note preview (requires fzf)
    Fzf {
        /// Optional note path or directory to filter
        path: Option<PathBuf>,
    },

    /// View note Git history and lifecycle events
    Log {
        /// Optional note path or directory to filter history
        path: Option<PathBuf>,

        /// Show events since specified date/time
        #[arg(long)]
        since: Option<String>,

        /// Show events until specified date/time
        #[arg(long)]
        until: Option<String>,

        /// Output log entries as structured JSON array
        #[arg(long)]
        json: bool,
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
