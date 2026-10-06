mod cli;
mod commands;
mod repo;

use clap::Parser;
use cli::Cli;
use repo::Repo;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
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
    let args: Vec<String> = env::args().collect();
    let cli = Cli::parse();

    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let repo_opt = Repo::find(&current_dir);

    let (cmd_name, exit_code) = match cli.command {
        Some(cmd) => {
            let name = format!("{:?}", cmd);
            let code = commands::run(repo_opt.clone(), cmd, cli.all);
            (name, code)
        }
        None => {
            print_welcome_instructions();
            ("welcome".to_string(), ExitCode::SUCCESS)
        }
    };

    log_execution(repo_opt.as_ref(), &cmd_name, &args, exit_code);

    exit_code
}

fn log_execution(
    repo_opt: Option<&Repo>,
    cmd_name: &str,
    args: &[String],
    exit_code: ExitCode,
) {
    let log_env = match env::var("DID_LOG_PATH") {
        Ok(v) if !v.is_empty() => v,
        _ => return,
    };

    let log_path = if Path::new(&log_env).is_relative() {
        if let Some(repo) = repo_opt {
            repo.did_dir.join(&log_env)
        } else {
            PathBuf::from(&log_env)
        }
    } else {
        PathBuf::from(&log_env)
    };

    if let Some(parent) = log_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let code_int = if exit_code == ExitCode::SUCCESS {
        0
    } else {
        1
    };

    let timestamp = match std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
        Ok(dur) => dur.as_secs().to_string(),
        Err(_) => "0".to_string(),
    };

    let mut xml = String::new();
    xml.push_str(&format!(
        "<invocation timestamp=\"{}\" exit_code=\"{}\">\n",
        timestamp, code_int
    ));
    xml.push_str(&format!(
        "  <command>{}</command>\n",
        escape_xml(cmd_name)
    ));
    xml.push_str("  <args>\n");
    for arg in args {
        xml.push_str(&format!("    <arg>{}</arg>\n", escape_xml(arg)));
    }
    xml.push_str("  </args>\n");
    xml.push_str("</invocation>\n");

    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_path) {
        let _ = file.write_all(xml.as_bytes());
    }
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
