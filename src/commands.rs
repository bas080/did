#![allow(clippy::collapsible_if)]

use crate::cli::Commands;
use crate::repo::Repo;
use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitCode};
use walkdir::WalkDir;

pub fn run(repo_opt: Option<Repo>, command: Commands, global_all: bool) -> ExitCode {
    match command {
        Commands::Init => cmd_init(),
        Commands::Add { path, message } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_add(&repo, &path, message)
        }
        Commands::Link { target, dest } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_link(&repo, &target, &dest)
        }
        Commands::Status { path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_status(&repo, path.as_deref(), global_all)
        }
        Commands::Show { path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_show(&repo, &path, global_all)
        }
        Commands::Done { path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_done(&repo, &path)
        }
        Commands::Autocomplete { shell } => cmd_autocomplete(&shell),
    }
}

fn require_repo(repo_opt: Option<Repo>) -> Result<Repo, ExitCode> {
    match repo_opt {
        Some(repo) => Ok(repo),
        None => {
            eprintln!("error: no .did state directory found. Run 'did init' first.");
            Err(ExitCode::FAILURE)
        }
    }
}

fn cmd_init() -> ExitCode {
    let current_dir = match env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error getting current directory: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = Repo::init(&current_dir) {
        eprintln!("error initializing .did: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn cmd_add(repo: &Repo, raw_path: &Path, message: Option<String>) -> ExitCode {
    let target_path = repo.resolve_path(raw_path);

    if target_path.exists() || fs::symlink_metadata(&target_path).is_ok() {
        eprintln!(
            "error: path already exists: {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    if let Some(parent) = target_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("error creating directories: {}", e);
            return ExitCode::FAILURE;
        }
    }

    if let Some(msg) = message {
        let content = if msg.ends_with('\n') {
            msg
        } else {
            format!("{}\n", msg)
        };
        if let Err(e) = fs::write(&target_path, content) {
            eprintln!("error writing file: {}", e);
            return ExitCode::FAILURE;
        }
    } else {
        // Open $EDITOR
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        // Touch file first
        if let Err(e) = fs::write(&target_path, "") {
            eprintln!("error creating file: {}", e);
            return ExitCode::FAILURE;
        }

        let status = Command::new(&editor).arg(&target_path).status();

        match status {
            Ok(s) if s.success() => {}
            _ => {
                eprintln!("error: editor '{}' failed or exited with error", editor);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

fn cmd_link(repo: &Repo, target_raw: &Path, dest_raw: &Path) -> ExitCode {
    let target_path = repo.resolve_path(target_raw);
    let dest_dir = repo.resolve_path(dest_raw);

    if !target_path.exists() && fs::symlink_metadata(&target_path).is_err() {
        eprintln!(
            "error: target does not exist: {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    let target_name = match target_path.file_name() {
        Some(name) => name,
        None => {
            eprintln!("error: invalid target path");
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = fs::create_dir_all(&dest_dir) {
        eprintln!("error creating destination directory: {}", e);
        return ExitCode::FAILURE;
    }

    let symlink_path = dest_dir.join(target_name);

    if symlink_path.exists() || fs::symlink_metadata(&symlink_path).is_ok() {
        eprintln!(
            "error: destination path already exists: {}",
            repo.relative_display_path(&symlink_path)
        );
        return ExitCode::FAILURE;
    }

    let rel_target = match compute_relative_path(&dest_dir, &target_path) {
        Some(p) => p,
        None => target_path.clone(),
    };

    if let Err(e) = symlink(&rel_target, &symlink_path) {
        eprintln!("error creating symlink: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn cmd_status(repo: &Repo, raw_path: Option<&Path>, all: bool) -> ExitCode {
    let root_path = match raw_path {
        Some(p) => repo.resolve_path(p),
        None => repo.did_dir.clone(),
    };

    if !root_path.exists() {
        eprintln!(
            "error: path does not exist: {}",
            repo.relative_display_path(&root_path)
        );
        return ExitCode::FAILURE;
    }

    let mut results = Vec::new();

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_entry(|e| should_visit_entry(e, all))
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == repo.did_dir {
            continue;
        }

        let is_symlink = entry.path_is_symlink();
        let is_file = path.is_file();

        if is_file || is_symlink {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let is_hidden = file_name.starts_with('.');

            if all || (!is_hidden && !has_unresolved_subitems(repo, path)) {
                results.push(repo.relative_display_path(path));
            }
        }
    }

    results.sort();
    results.dedup();

    if results.is_empty() {
        eprintln!("No actionable tasks found.");
    } else {
        for res in results {
            println!("{}", res);
        }
    }

    ExitCode::SUCCESS
}

fn cmd_show(repo: &Repo, raw_path: &Path, all: bool) -> ExitCode {
    let target_path = repo.resolve_path(raw_path);

    let meta = match fs::symlink_metadata(&target_path) {
        Ok(m) => m,
        Err(_) => {
            eprintln!(
                "error: path does not exist: {}",
                repo.relative_display_path(&target_path)
            );
            return ExitCode::FAILURE;
        }
    };

    if meta.is_dir() {
        eprintln!(
            "error: 'did show' requires a task file, got directory: {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    if !all && has_unresolved_subitems(repo, &target_path) {
        eprintln!(
            "error: task has unresolved sub-items: {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    // Collect files in parent directories top-down
    let mut ancestor_dirs = Vec::new();
    let mut curr = target_path.parent();
    while let Some(dir) = curr {
        if dir == repo.did_dir {
            ancestor_dirs.push(dir.to_path_buf());
            break;
        }
        if dir.starts_with(&repo.did_dir) {
            ancestor_dirs.push(dir.to_path_buf());
        }
        curr = dir.parent();
    }
    ancestor_dirs.reverse();

    let mut printed_any = false;

    for dir in ancestor_dirs {
        let mut files_in_dir = Vec::new();
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_file() {
                    let name = p.file_name().unwrap_or_default().to_string_lossy();
                    if all || !name.starts_with('.') {
                        files_in_dir.push(p);
                    }
                }
            }
        }
        files_in_dir.sort();

        for f in files_in_dir {
            if f == target_path {
                continue;
            }
            if printed_any {
                println!();
            }
            print_file_content(repo, &f);
            printed_any = true;
        }
    }

    if printed_any {
        println!();
    }
    print_file_content(repo, &target_path);

    ExitCode::SUCCESS
}

fn cmd_done(repo: &Repo, raw_path: &Path) -> ExitCode {
    let target_path = repo.resolve_path(raw_path);

    let meta = match fs::symlink_metadata(&target_path) {
        Ok(m) => m,
        Err(_) => {
            eprintln!(
                "error: path does not exist: {}",
                repo.relative_display_path(&target_path)
            );
            return ExitCode::FAILURE;
        }
    };

    if meta.is_dir() {
        eprintln!(
            "error: cannot complete a directory: {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    let file_name = match target_path.file_name() {
        Some(name) => name.to_string_lossy(),
        None => {
            eprintln!("error: invalid path");
            return ExitCode::FAILURE;
        }
    };

    if file_name.starts_with('.') {
        // Already done
        return ExitCode::SUCCESS;
    }

    if has_unresolved_subitems(repo, &target_path) {
        eprintln!(
            "error: cannot mark task done: unresolved sub-items remain for {}",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    let new_filename = format!(".{}", file_name);
    let parent = target_path.parent().unwrap();
    let new_target_path = parent.join(&new_filename);

    if let Err(e) = fs::rename(&target_path, &new_target_path) {
        eprintln!("error marking task done: {}", e);
        return ExitCode::FAILURE;
    }

    // Update symlinks pointing to target_path across .did
    update_symlinks(repo, &target_path, &new_target_path);

    ExitCode::SUCCESS
}

fn cmd_autocomplete(shell: &str) -> ExitCode {
    use clap::CommandFactory;
    use clap_complete::{generate, Shell};

    let mut cmd = crate::cli::Cli::command();
    let parsed_shell = match shell.to_lowercase().as_str() {
        "bash" => Shell::Bash,
        "zsh" => Shell::Zsh,
        "fish" => Shell::Fish,
        "powershell" => Shell::PowerShell,
        "elvish" => Shell::Elvish,
        _ => {
            eprintln!("error: unsupported shell: {}", shell);
            return ExitCode::FAILURE;
        }
    };

    generate(parsed_shell, &mut cmd, "did", &mut std::io::stdout());
    ExitCode::SUCCESS
}

fn should_visit_entry(entry: &walkdir::DirEntry, all: bool) -> bool {
    if all {
        return true;
    }
    let file_name = entry.file_name().to_string_lossy();
    if file_name.starts_with('.') && file_name != ".did" {
        return false;
    }
    true
}

fn print_file_content(repo: &Repo, path: &Path) {
    let rel = repo.relative_display_path(path);
    println!("{}", rel);
    match fs::read_to_string(path) {
        Ok(content) => {
            if content.ends_with('\n') {
                print!("{}", content);
            } else {
                println!("{}", content);
            }
        }
        Err(e) => {
            eprintln!("error reading file {}: {}", rel, e);
        }
    }
}

/// Checks if a task file has any unresolved sub-items (deeper subdirectories).
fn has_unresolved_subitems(repo: &Repo, task_file: &Path) -> bool {
    let parent_dir = match task_file.parent() {
        Some(p) => p,
        None => return false,
    };

    let task_stem = task_file.file_stem().unwrap_or_default().to_string_lossy();

    if let Ok(entries) = fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p == task_file {
                continue;
            }
            let is_symlink = p.is_symlink();
            if p.is_dir() && !is_symlink {
                let dir_name = p.file_name().unwrap_or_default().to_string_lossy();
                if !dir_name.starts_with('.') {
                    // Check if dir_name belongs to task_file:
                    // Matches task_stem OR parent_dir is not root and no sibling task file named dir_name exists
                    let belongs_to_task = dir_name == task_stem
                        || (parent_dir != repo.did_dir
                            && !has_sibling_task_file(parent_dir, &dir_name));

                    if belongs_to_task && directory_has_unresolved_files(&p) {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn has_sibling_task_file(parent_dir: &Path, dir_name: &str) -> bool {
    if let Ok(entries) = fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() {
                let stem = p.file_stem().unwrap_or_default().to_string_lossy();
                if stem == dir_name {
                    return true;
                }
            }
        }
    }
    false
}

fn directory_has_unresolved_files(dir: &Path) -> bool {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        if p.is_file() || p.is_symlink() {
            return true;
        }
    }
    false
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                components.pop();
            }
            c => components.push(c),
        }
    }
    components.into_iter().collect()
}

fn update_symlinks(repo: &Repo, old_target: &Path, new_target: &Path) {
    let old_norm = normalize_path(old_target);

    for entry in WalkDir::new(&repo.did_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_symlink() {
            if let Ok(target) = fs::read_link(path) {
                let abs_target = if target.is_relative() {
                    path.parent().unwrap().join(&target)
                } else {
                    target.clone()
                };

                let norm_target = normalize_path(&abs_target);

                if norm_target == old_norm {
                    let parent = path.parent().unwrap();
                    let old_sym_name = path.file_name().unwrap().to_string_lossy();
                    let new_sym_name = if old_sym_name.starts_with('.') {
                        old_sym_name.to_string()
                    } else {
                        format!(".{}", old_sym_name)
                    };
                    let new_sym_path = parent.join(new_sym_name);

                    let new_rel_target = compute_relative_path(parent, new_target)
                        .unwrap_or_else(|| new_target.to_path_buf());

                    let _ = fs::remove_file(path);
                    let _ = symlink(&new_rel_target, &new_sym_path);
                }
            }
        }
    }
}

fn compute_relative_path(from_dir: &Path, to_file: &Path) -> Option<PathBuf> {
    let from = from_dir
        .canonicalize()
        .ok()
        .unwrap_or_else(|| from_dir.to_path_buf());
    let to = to_file
        .canonicalize()
        .ok()
        .unwrap_or_else(|| to_file.to_path_buf());

    let from_comps: Vec<_> = from.components().collect();
    let to_comps: Vec<_> = to.components().collect();

    let mut common_len = 0;
    while common_len < from_comps.len()
        && common_len < to_comps.len()
        && from_comps[common_len] == to_comps[common_len]
    {
        common_len += 1;
    }

    let mut rel = PathBuf::new();
    for _ in common_len..from_comps.len() {
        rel.push("..");
    }
    for comp in &to_comps[common_len..] {
        rel.push(comp.as_os_str());
    }

    if rel.as_os_str().is_empty() {
        Some(PathBuf::from("."))
    } else {
        Some(rel)
    }
}
