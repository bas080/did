#![allow(clippy::collapsible_if)]

use crate::cli::Commands;
use crate::repo::Repo;
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::{Command, ExitCode};
use walkdir::WalkDir;

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[cfg(windows)]
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(original: P, link: Q) -> std::io::Result<()> {
    if original.as_ref().is_dir() {
        std::os::windows::fs::symlink_dir(original, link)
    } else {
        std::os::windows::fs::symlink_file(original, link)
    }
}

pub fn run(repo_opt: Option<Repo>, command: Commands, global_all: bool) -> ExitCode {
    match command {
        Commands::Add { path, message } => {
            let repo = match repo_opt {
                Some(r) => r,
                None => {
                    let current_dir = match env::current_dir() {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("error getting current directory: {}", e);
                            return ExitCode::FAILURE;
                        }
                    };
                    match Repo::init(&current_dir) {
                        Ok(r) => r,
                        Err(e) => {
                            eprintln!("error initializing .did directory: {}", e);
                            return ExitCode::FAILURE;
                        }
                    }
                }
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
        Commands::Mv { old_path, new_path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_mv(&repo, &old_path, &new_path)
        }
        Commands::Status { path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_status(&repo, path.as_deref(), global_all)
        }
        Commands::Search { query, path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_search(&repo, &query, path.as_deref(), global_all)
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
        Commands::Undone { path } => {
            let repo = match require_repo(repo_opt) {
                Ok(r) => r,
                Err(code) => return code,
            };
            cmd_undone(&repo, &path)
        }
        Commands::Autocomplete { shell } => cmd_autocomplete(&shell),
    }
}

fn require_repo(repo_opt: Option<Repo>) -> Result<Repo, ExitCode> {
    match repo_opt {
        Some(repo) => Ok(repo),
        None => {
            eprintln!("error: no .did state directory found.");
            Err(ExitCode::FAILURE)
        }
    }
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

    // Collect ancestor directories top-down for add hooks
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

    let target_rel = repo.relative_display_path(&target_path);
    let msg_arg = message.as_deref().unwrap_or("");

    for dir in ancestor_dirs {
        let active_hooks = find_hook_files(&dir, "add");

        for hook_p in active_hooks {
            if is_executable(&hook_p) {
                let output = Command::new(&hook_p)
                    .arg(&target_rel)
                    .arg(msg_arg)
                    .output();
                match output {
                    Ok(out) => {
                        let stdout_str = String::from_utf8_lossy(&out.stdout);
                        if !stdout_str.is_empty() {
                            if stdout_str.ends_with('\n') {
                                print!("{}", stdout_str);
                            } else {
                                println!("{}", stdout_str);
                            }
                        }
                        let stderr_str = String::from_utf8_lossy(&out.stderr);
                        if !stderr_str.is_empty() {
                            eprintln!("{}", stderr_str);
                        }
                        if !out.status.success() {
                            return ExitCode::FAILURE;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "error executing hook {}: {}",
                            repo.relative_display_path(&hook_p),
                            e
                        );
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                print_file_content(repo, &hook_p);
            }
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

fn cmd_mv(repo: &Repo, old_raw: &Path, new_raw: &Path) -> ExitCode {
    let old_path = repo.resolve_path(old_raw);
    let new_path = repo.resolve_path(new_raw);

    if !old_path.exists() && fs::symlink_metadata(&old_path).is_err() {
        eprintln!(
            "error: path does not exist: {}",
            repo.relative_display_path(&old_path)
        );
        return ExitCode::FAILURE;
    }

    if new_path.exists() || fs::symlink_metadata(&new_path).is_ok() {
        eprintln!(
            "error: destination path already exists: {}",
            repo.relative_display_path(&new_path)
        );
        return ExitCode::FAILURE;
    }

    if let Some(parent) = new_path.parent() {
        if let Err(e) = fs::create_dir_all(parent) {
            eprintln!("error creating directories: {}", e);
            return ExitCode::FAILURE;
        }
    }

    if let Err(e) = fs::rename(&old_path, &new_path) {
        eprintln!("error moving path: {}", e);
        return ExitCode::FAILURE;
    }

    update_symlinks_for_mv(repo, &old_path, &new_path);

    ExitCode::SUCCESS
}

fn update_symlinks_for_mv(repo: &Repo, old_path: &Path, new_path: &Path) {
    let old_norm = normalize_path(old_path);
    let new_norm = normalize_path(new_path);

    for entry in WalkDir::new(&repo.did_dir).into_iter().filter_map(|e| e.ok()) {
        let sym_path = entry.path();
        if sym_path.is_symlink() {
            if let Ok(target) = fs::read_link(sym_path) {
                let parent = sym_path.parent().unwrap();
                let abs_target = if target.is_relative() {
                    parent.join(&target)
                } else {
                    target.clone()
                };

                let norm_target = normalize_path(&abs_target);

                if norm_target == old_norm || norm_target.starts_with(&old_norm) {
                    let sub_rel = norm_target.strip_prefix(&old_norm).unwrap_or(Path::new(""));
                    let new_abs_target = new_norm.join(sub_rel);

                    if let Some(new_rel_target) = compute_relative_path(parent, &new_abs_target) {
                        let _ = fs::remove_file(sym_path);
                        let _ = symlink(&new_rel_target, sym_path);
                    }
                } else if sym_path.starts_with(&new_norm) {
                    if let Some(new_rel_target) = compute_relative_path(parent, &norm_target) {
                        let _ = fs::remove_file(sym_path);
                        let _ = symlink(&new_rel_target, sym_path);
                    }
                }
            }
        }
    }
}

fn should_visit_entry(entry: &walkdir::DirEntry) -> bool {
    let file_name = entry.file_name().to_string_lossy();
    if file_name == ".hooks" || file_name == "hooks" || file_name == ".git" {
        return false;
    }
    true
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
    let mut blocked_count = 0;
    let mut closed_count = 0;

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_entry(should_visit_entry)
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == repo.did_dir || is_reserved_hook_file(path) {
            continue;
        }

        let is_symlink = entry.path_is_symlink();
        let is_file = path.is_file();

        if is_file || is_symlink {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let is_hidden = file_name.starts_with('.');

            if is_hidden {
                closed_count += 1;
                if all {
                    results.push(repo.relative_display_path(path));
                }
            } else if has_unresolved_subitems(repo, path) {
                blocked_count += 1;
                if all {
                    results.push(repo.relative_display_path(path));
                }
            } else {
                results.push(repo.relative_display_path(path));
            }
        }
    }

    results.sort();
    results.dedup();

    print_results_with_limit(results, "No actionable tasks found.");

    eprintln!("[{} blocked, {} closed]", blocked_count, closed_count);

    ExitCode::SUCCESS
}

fn cmd_search(repo: &Repo, query: &str, raw_path: Option<&Path>, all: bool) -> ExitCode {
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

    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    for entry in WalkDir::new(&root_path)
        .into_iter()
        .filter_entry(should_visit_entry)
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path == repo.did_dir || is_reserved_hook_file(path) {
            continue;
        }

        let is_symlink = entry.path_is_symlink();
        let is_file = path.is_file();

        if is_file || is_symlink {
            let file_name = path.file_name().unwrap_or_default().to_string_lossy();
            let is_hidden = file_name.starts_with('.');

            let should_include = if all {
                true
            } else {
                !is_hidden && !has_unresolved_subitems(repo, path)
            };

            if should_include {
                let rel_display = repo.relative_display_path(path);
                let matches_path = rel_display.to_lowercase().contains(&query_lower);
                let matches_content = if is_file {
                    fs::read_to_string(path)
                        .ok()
                        .map(|c| c.to_lowercase().contains(&query_lower))
                        .unwrap_or(false)
                } else {
                    false
                };

                if matches_path || matches_content {
                    results.push(rel_display);
                }
            }
        }
    }

    results.sort();
    results.dedup();

    print_results_with_limit(results, "No matching tasks found.");

    ExitCode::SUCCESS
}

fn get_status_limit() -> Option<usize> {
    if let Ok(val) = env::var("DID_STATUS_LIMIT") {
        if let Ok(limit) = val.parse::<usize>() {
            return Some(limit);
        }
    }
    if let Ok(val) = env::var("DID_LIMIT") {
        if let Ok(limit) = val.parse::<usize>() {
            return Some(limit);
        }
    }
    for (key, val) in env::vars() {
        if key.starts_with("DID_STATUS_") || key.starts_with("DID_LIMIT") {
            if let Ok(limit) = val.parse::<usize>() {
                return Some(limit);
            }
        }
    }
    None
}

fn print_results_with_limit(results: Vec<String>, empty_msg: &str) {
    if results.is_empty() {
        if !empty_msg.is_empty() {
            eprintln!("{}", empty_msg);
        }
        return;
    }

    let limit = get_status_limit();
    let total = results.len();

    let display_count = match limit {
        Some(l) if l < total => l,
        _ => total,
    };

    for res in &results[..display_count] {
        println!("{}", res);
    }

    if let Some(l) = limit {
        if l < total {
            eprintln!(
                "Notice: status limit reached ({}/{} items shown). Use status on a specific directory, search, or adjust limit.",
                l, total
            );
        }
    }
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            return meta.permissions().mode() & 0o111 != 0;
        }
    }
    #[cfg(windows)]
    {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if ext_lower == "exe" || ext_lower == "bat" || ext_lower == "cmd" {
                return true;
            }
        }
    }
    false
}

fn is_reserved_hook_file(path: &Path) -> bool {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    if matches!(
        stem.as_ref(),
        "show" | "status" | "done" | "add" | "link" | "mv"
    ) {
        if let Some(parent) = path.parent() {
            let parent_name = parent.file_name().unwrap_or_default().to_string_lossy();
            if parent_name == ".hooks" || parent_name == "hooks" {
                return true;
            }
        }
    }
    false
}

fn find_hook_files(ancestor_dir: &Path, event: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();

    for sub in &[".hooks", "hooks"] {
        let hook_dir = ancestor_dir.join(sub);
        if hook_dir.is_dir() {
            if let Ok(entries) = fs::read_dir(&hook_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(stem) = path.file_stem() {
                            if stem.to_string_lossy() == event {
                                matches.push(path);
                            }
                        }
                    }
                }
            }
        }
    }

    matches.sort();
    matches.dedup();
    matches
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
        let rel = repo.relative_display_path(&target_path);
        eprintln!(
            "error: 'did show' requires a task file, got directory: {}. Use 'did status {}' instead.",
            rel, rel
        );
        return ExitCode::FAILURE;
    }

    let blocking = get_unresolved_blocking_items(repo, &target_path);
    if !all && !blocking.is_empty() {
        eprintln!(
            "error: task '{}' is blocked by unresolved sub-items:",
            repo.relative_display_path(&target_path)
        );
        for item in blocking {
            eprintln!("  - {}", item);
        }
        return ExitCode::FAILURE;
    }

    // Collect ancestor directories top-down
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
        let active_hooks = find_hook_files(&dir, "show");

        for hook_p in active_hooks {
            if is_executable(&hook_p) {
                let output = Command::new(&hook_p).output();
                match output {
                    Ok(out) => {
                        if printed_any {
                            println!();
                        }
                        let stdout_str = String::from_utf8_lossy(&out.stdout);
                        if stdout_str.ends_with('\n') {
                            print!("{}", stdout_str);
                        } else {
                            println!("{}", stdout_str);
                        }
                        if !out.status.success() {
                            let stderr_str = String::from_utf8_lossy(&out.stderr);
                            if !stderr_str.is_empty() {
                                eprintln!("{}", stderr_str);
                            }
                            return ExitCode::FAILURE;
                        }
                        printed_any = true;
                    }
                    Err(e) => {
                        eprintln!(
                            "error executing hook {}: {}",
                            repo.relative_display_path(&hook_p),
                            e
                        );
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                if printed_any {
                    println!();
                }
                print_file_content(repo, &hook_p);
                printed_any = true;
            }
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

    let blocking = get_unresolved_blocking_items(repo, &target_path);
    if !blocking.is_empty() {
        eprintln!(
            "error: cannot mark task '{}' done: unresolved sub-items remain:",
            repo.relative_display_path(&target_path)
        );
        for item in blocking {
            eprintln!("  - {}", item);
        }
        return ExitCode::FAILURE;
    }

    // Collect ancestor directories top-down for done hooks
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

    let target_rel = repo.relative_display_path(&target_path);

    for dir in ancestor_dirs {
        let active_hooks = find_hook_files(&dir, "done");

        for hook_p in active_hooks {
            if is_executable(&hook_p) {
                let output = Command::new(&hook_p).arg(&target_rel).output();
                match output {
                    Ok(out) => {
                        let stdout_str = String::from_utf8_lossy(&out.stdout);
                        if !stdout_str.is_empty() {
                            if stdout_str.ends_with('\n') {
                                print!("{}", stdout_str);
                            } else {
                                println!("{}", stdout_str);
                            }
                        }
                        let stderr_str = String::from_utf8_lossy(&out.stderr);
                        if !stderr_str.is_empty() {
                            eprintln!("{}", stderr_str);
                        }
                        if !out.status.success() {
                            return ExitCode::FAILURE;
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "error executing hook {}: {}",
                            repo.relative_display_path(&hook_p),
                            e
                        );
                        return ExitCode::FAILURE;
                    }
                }
            } else {
                print_file_content(repo, &hook_p);
            }
        }
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

fn cmd_undone(repo: &Repo, raw_path: &Path) -> ExitCode {
    let mut target_path = repo.resolve_path(raw_path);

    if !target_path.exists() && fs::symlink_metadata(&target_path).is_err() {
        if let Some(parent) = target_path.parent() {
            if let Some(file_name) = target_path.file_name() {
                let name_str = file_name.to_string_lossy();
                if !name_str.starts_with('.') {
                    let dot_path = parent.join(format!(".{}", name_str));
                    if dot_path.exists() || fs::symlink_metadata(&dot_path).is_ok() {
                        target_path = dot_path;
                    }
                }
            }
        }
    }

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
            "error: cannot undone a directory: {}",
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

    if !file_name.starts_with('.') {
        eprintln!(
            "error: task '{}' is not resolved (does not start with a dot)",
            repo.relative_display_path(&target_path)
        );
        return ExitCode::FAILURE;
    }

    let new_filename = file_name.strip_prefix('.').unwrap_or(&file_name).to_string();
    let parent = target_path.parent().unwrap();
    let new_target_path = parent.join(&new_filename);

    if new_target_path.exists() || fs::symlink_metadata(&new_target_path).is_ok() {
        eprintln!(
            "error: target path already exists: {}",
            repo.relative_display_path(&new_target_path)
        );
        return ExitCode::FAILURE;
    }

    if let Err(e) = fs::rename(&target_path, &new_target_path) {
        eprintln!("error marking task undone: {}", e);
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

/// Returns a list of unresolved blocking items (relative paths and symlink targets) under child subdirectories of `task_file`.
fn get_unresolved_blocking_items(repo: &Repo, task_file: &Path) -> Vec<String> {
    let parent_dir = match task_file.parent() {
        Some(p) => p,
        None => return Vec::new(),
    };

    let is_root_dir = parent_dir == repo.did_dir;
    let task_stem = task_file.file_stem().unwrap_or_default().to_string_lossy();
    let mut items = Vec::new();

    if let Ok(entries) = fs::read_dir(parent_dir) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p == task_file {
                continue;
            }
            let is_symlink = p.is_symlink();
            if p.is_dir() && !is_symlink {
                let dir_name = p.file_name().unwrap_or_default().to_string_lossy();
                if dir_name.starts_with('.') {
                    continue;
                }
                if is_root_dir {
                    if dir_name == task_stem {
                        collect_unresolved_in_dir(repo, &p, &mut items);
                    }
                } else {
                    collect_unresolved_in_dir(repo, &p, &mut items);
                }
            }
        }
    }

    items.sort();
    items.dedup();
    items
}

fn collect_unresolved_in_dir(repo: &Repo, dir: &Path, items: &mut Vec<String>) {
    let dir_name = dir.file_name().unwrap_or_default().to_string_lossy();
    if dir_name.starts_with('.') {
        return;
    }
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if is_reserved_hook_file(p) {
            continue;
        }
        let name = p.file_name().unwrap_or_default().to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        if p.is_symlink() {
            let target_str = fs::read_link(p)
                .map(|t| t.to_string_lossy().to_string())
                .unwrap_or_default();
            items.push(format!("{} -> {}", repo.relative_display_path(p), target_str));
        } else if p.is_file() {
            items.push(repo.relative_display_path(p));
        }
    }
}

/// Checks if a task file has any unresolved sub-items (deeper subdirectories).
fn has_unresolved_subitems(repo: &Repo, task_file: &Path) -> bool {
    !get_unresolved_blocking_items(repo, task_file).is_empty()
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
    let is_old_dot = old_target
        .file_name()
        .map(|n| n.to_string_lossy().starts_with('.'))
        .unwrap_or(false);

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
                    let new_sym_name = if is_old_dot {
                        old_sym_name.strip_prefix('.').unwrap_or(&old_sym_name).to_string()
                    } else {
                        if old_sym_name.starts_with('.') {
                            old_sym_name.to_string()
                        } else {
                            format!(".{}", old_sym_name)
                        }
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
