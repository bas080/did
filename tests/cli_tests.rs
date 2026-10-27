use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn did_cmd_env(dir: &Path, args: &[&str], envs: &[(&str, &str)]) -> (bool, String, String) {
    let bin_path = env!("CARGO_BIN_EXE_did");
    let mut cmd = Command::new(bin_path);
    cmd.current_dir(dir).args(args);
    for (k, v) in envs {
        cmd.env(k, v);
    }
    let output = cmd.output().expect("Failed to execute did binary");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

fn did_cmd(dir: &Path, args: &[&str]) -> (bool, String, String) {
    did_cmd_env(dir, args, &[])
}

fn did_cmd_stdin(dir: &Path, args: &[&str], stdin_str: &str) -> (bool, String, String) {
    let bin_path = env!("CARGO_BIN_EXE_did");
    let mut cmd = Command::new(bin_path);
    cmd.current_dir(dir).args(args);
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let mut child = cmd.spawn().expect("Failed to spawn did binary");
    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(stdin_str.as_bytes());
    }
    let output = child.wait_with_output().expect("Failed to wait on child");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

#[test]
fn test_init_via_add() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // did add automatically creates .did/
    let (success, _, _) = did_cmd_stdin(root, &["add", "notes.md"], "Parent notes");
    assert!(success);
    assert!(root.join(".did").is_dir());
    assert!(root.join(".did/notes.md").is_file());
}

#[test]
fn test_add_and_show() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Add root task notes.md
    let (success1, _, _) = did_cmd_stdin(root, &["add", "notes.md"], "Parent notes");
    assert!(success1);
    assert!(root.join(".did/notes.md").is_file());

    // Add nested task
    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    let (success2, _, _) = did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    assert!(success2);
    assert!(root.join(".did/backend/auth/jwt.md").is_file());

    // Add duplicate task fails
    let (success_dup, _, stderr) = did_cmd_stdin(root, &["add", "notes.md"], "duplicate");
    assert!(!success_dup);
    assert!(stderr.contains("path already exists"));

    // Show directory fails
    let (success_show_dir, _, stderr_show_dir) = did_cmd(root, &["show", "backend"]);
    assert!(!success_show_dir);
    assert!(stderr_show_dir.contains("requires a note file, got directory"));
    assert!(stderr_show_dir.contains("Use 'did status backend' instead"));

    // Status -b lists blocked notes (empty in default pure note mode)
    did_cmd_stdin(root, &["add", "c.md"], "Task C");
    did_cmd(root, &["mv", "c.md", "backend/auth/sub/c.md"]);
    let (success_b, _, stderr_b) = did_cmd(root, &["status", "-b"]);
    assert!(success_b);
    assert!(stderr_b.contains("No blocked notes found."));
}

#[test]
fn test_show_hook_sibling_and_ancestor() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    // Create task
    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);

    // Create ancestor hook .did/backend/.hooks/show
    let ancestor_hook = root.join(".did/backend/.hooks/show");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Backend ancestor guidelines").unwrap();

    // Create sibling hook .did/backend/auth/.hooks/show
    let sibling_hook = root.join(".did/backend/auth/.hooks/show");
    fs::create_dir_all(sibling_hook.parent().unwrap()).unwrap();
    fs::write(&sibling_hook, "Auth sibling guidelines").unwrap();

    let (success, stdout, stderr) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success);
    assert!(stderr.contains("Backend ancestor guidelines"));
    assert!(stderr.contains("Auth sibling guidelines"));
    assert!(stdout.contains("backend/auth/jwt.md"));
    assert!(stdout.contains("JWT implementation"));
}

#[test]
fn test_show_hook_with_extension() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);

    // Create hook with extension .did/backend/.hooks/show.md
    let ancestor_hook = root.join(".did/backend/.hooks/show.md");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Markdown hook content").unwrap();

    let (success, stdout, stderr) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success);
    assert!(stderr.contains("Markdown hook content"));
    assert!(stdout.contains("JWT implementation"));
}

#[test]
fn test_status_and_hierarchy() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    // Add two leaf notes in backend/auth/
    did_cmd_stdin(root, &["add", "a.md"], "Task A");
    did_cmd(root, &["mv", "a.md", "backend/auth/a.md"]);
    did_cmd_stdin(root, &["add", "b.md"], "Task B");
    did_cmd(root, &["mv", "b.md", "backend/auth/b.md"]);

    // Status shows both a.md and b.md
    let (success_stat, stdout_stat, stderr_stat) = did_cmd(root, &["status"]);
    assert!(success_stat);
    assert_eq!(stdout_stat.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat.contains("[0 blocked, 0 closed]"));

    // Now add a deeper subdirectory backend/auth/sub/c.md
    did_cmd_stdin(root, &["add", "c.md"], "Task C");
    did_cmd(root, &["mv", "c.md", "backend/auth/sub/c.md"]);

    // In pure note mode, all open notes are listed in status
    let (_, stdout_stat2, stderr_stat2) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat2.trim(), "backend/auth/a.md\nbackend/auth/b.md\nbackend/auth/sub/c.md");
    assert!(stderr_stat2.contains("[0 blocked, 0 closed]"));

    // Complete sub/c.md
    let (success_done_c, _, _) = did_cmd(root, &["done", "backend/auth/sub/c.md"]);
    assert!(success_done_c);
    assert!(root.join(".did/backend/auth/sub/.c.md").is_file());

    // Status lists remaining open notes
    let (_, stdout_stat3, stderr_stat3) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat3.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat3.contains("[0 blocked, 1 closed]"));

    // Status -a shows resolved note as well
    let (_, stdout_stat_a, _) = did_cmd(root, &["status", "-a"]);
    assert!(stdout_stat_a.contains("backend/auth/sub/.c.md"));

    // Status -t (tree mode) shows open notes with status indicator [ ]
    let (success_tree, stdout_tree, _) = did_cmd(root, &["status", "-t"]);
    assert!(success_tree);
    assert!(stdout_tree.contains("[ ] a.md"));
    assert!(stdout_tree.contains("[ ] b.md"));

    // Status -t -a (tree mode with all) shows closed note with [x]
    let (success_tree_a, stdout_tree_a, _) = did_cmd(root, &["status", "-t", "-a"]);
    assert!(success_tree_a);
    assert!(stdout_tree_a.contains("- [x] sub/.c.md") || stdout_tree_a.contains("[x] sub/.c.md"));
}

#[test]
fn test_status_limit_env() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task1.md"], "Task 1");
    did_cmd_stdin(root, &["add", "task2.md"], "Task 2");
    did_cmd_stdin(root, &["add", "task3.md"], "Task 3");

    let (success, stdout, stderr) =
        did_cmd_env(root, &["status"], &[("DID_STATUS_LIMIT", "2")]);
    assert!(success);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(stderr.contains("status limit reached (2/3 items shown)"));
    assert!(stderr.contains("Use status on a specific directory, search, or adjust limit"));
}

#[test]
fn test_status_default_path_env() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd_stdin(root, &["add", "root_task.md"], "Root task");
    did_cmd_stdin(root, &["add", "task1.md"], "Subdir task 1");
    did_cmd(root, &["mv", "task1.md", "subdir/task1.md"]);

    // With DID_STATUS_PATH=subdir, status should target subdir
    let (success1, stdout1, _) = did_cmd_env(root, &["status"], &[("DID_STATUS_PATH", "subdir")]);
    assert!(success1);
    assert!(stdout1.contains("subdir/task1.md"));
    assert!(!stdout1.contains("root_task.md"));

    // Explicit path CLI argument takes precedence over DID_STATUS_PATH (with -a to show blocked item)
    let (success2, stdout2, _) = did_cmd_env(
        root,
        &["status", "-a", "root_task.md"],
        &[("DID_STATUS_PATH", "subdir")],
    );
    assert!(success2);
    assert!(stdout2.contains("root_task.md"));
    assert!(!stdout2.contains("subdir/task1.md"));
}

#[test]
fn test_show_and_done_blocked_output() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "auth.md"], "Auth note");
    did_cmd(root, &["mv", "auth.md", "backend/auth.md"]);
    did_cmd_stdin(root, &["add", "research.md"], "Research");
    did_cmd(root, &["mv", "research.md", "backend/auth/sub/research.md"]);

    // show backend/auth.md shows note content
    let (success_show, stdout_show, _) = did_cmd(root, &["show", "backend/auth.md"]);
    assert!(success_show);
    assert!(stdout_show.contains("Auth note"));

    // done backend/auth.md succeeds in pure note system
    let (success_done, _, _) = did_cmd(root, &["done", "backend/auth.md"]);
    assert!(success_done);
    assert!(root.join(".did/backend/.auth.md").is_file());
}

#[test]
fn test_link_and_done_updates_symlink() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "feature.md"], "Feature requirement");
    did_cmd(root, &["mv", "feature.md", "specs/feature.md"]);
    did_cmd_stdin(root, &["add", "server.md"], "Server implementation");
    did_cmd(root, &["mv", "server.md", "backend/server.md"]);

    // Link specs/feature.md into backend/
    let (success_link, _, _) = did_cmd(root, &["link", "specs/feature.md", "backend"]);
    assert!(success_link);

    let symlink_file = root.join(".did/backend/feature.md");
    assert!(symlink_file.is_symlink());

    // Complete specs/feature.md
    let (success_done_feat, _, _) = did_cmd(root, &["done", "specs/feature.md"]);
    assert!(success_done_feat);
    assert!(root.join(".did/specs/.feature.md").is_file());

    // Symlink inside backend should be updated to .feature.md!
    let updated_symlink = root.join(".did/backend/.feature.md");
    assert!(updated_symlink.is_symlink());
    let target = fs::read_link(&updated_symlink).unwrap();
    assert_eq!(target, Path::new("../specs/.feature.md"));

    // Now server.md is actionable!
    let (_, stdout_stat2, _) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat2.trim(), "backend/server.md");

    // Now complete server.md
    let (success_done_server2, _, _) = did_cmd(root, &["done", "backend/server.md"]);
    assert!(success_done_server2);
    assert!(root.join(".did/backend/.server.md").is_file());
}

#[test]
fn test_search_feature() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did/backend/auth")).unwrap();
    fs::create_dir_all(root.join(".did/frontend")).unwrap();
    fs::create_dir_all(root.join(".did/docs")).unwrap();

    fs::write(root.join(".did/backend/auth/jwt.md"), "Token verification").unwrap();
    fs::write(root.join(".did/frontend/login.md"), "Calls JWT auth endpoint").unwrap();
    fs::write(root.join(".did/docs/notes.md"), "General documentation").unwrap();

    // Search "jwt" matches backend/auth/jwt.md (path) and frontend/login.md (content snippet)
    let (success1, stdout1, _) = did_cmd(root, &["search", "jwt"]);
    assert!(success1);
    assert_eq!(stdout1.trim(), "backend/auth/jwt.md\nfrontend/login.md: Calls JWT auth endpoint");

    // Search "jwt" with -n / --line-number
    let (success_n, stdout_n, _) = did_cmd(root, &["search", "-n", "jwt"]);
    assert!(success_n);
    assert_eq!(stdout_n.trim(), "backend/auth/jwt.md\nfrontend/login.md:1: Calls JWT auth endpoint");

    // Case-insensitive search "TOKEN"
    let (success2, stdout2, _) = did_cmd(root, &["search", "TOKEN"]);
    assert!(success2);
    assert_eq!(stdout2.trim(), "backend/auth/jwt.md: Token verification");

    // Search with subtree path argument
    let (success3, stdout3, _) = did_cmd(root, &["search", "jwt", "frontend"]);
    assert!(success3);
    assert_eq!(stdout3.trim(), "frontend/login.md: Calls JWT auth endpoint");

    // Search with subtree path argument
    did_cmd_stdin(root, &["add", "subtask.md"], "JWT helper task");
    did_cmd(root, &["mv", "subtask.md", "backend/auth/jwt/subtask.md"]);
    let (_, stdout_search, _) = did_cmd(root, &["search", "TOKEN"]);
    assert!(stdout_search.contains("backend/auth/jwt.md"));

    let (_, stdout_with_a, _) = did_cmd(root, &["search", "TOKEN", "-a"]);
    assert!(stdout_with_a.contains("backend/auth/jwt"));

    // Status limit via env var DID_STATUS_LIMIT
    let (success_lim, stdout_lim, stderr_lim) = did_cmd_env(
        root,
        &["search", "jwt", "-a"],
        &[("DID_STATUS_LIMIT", "1")],
    );
    assert!(success_lim);
    let lines: Vec<&str> = stdout_lim.trim().lines().collect();
    assert_eq!(lines.len(), 1);
    assert!(stderr_lim.contains("query limit reached"));
}

#[test]
fn test_fuzzy_search_typo_tolerance() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did/backend/auth")).unwrap();
    fs::write(root.join(".did/backend/auth/jwt.md"), "JSON Web Token verification").unwrap();

    // Search typo "authjwt" matches backend/auth/jwt.md
    let (success1, stdout1, _) = did_cmd(root, &["query", "authjwt"]);
    assert!(success1);
    assert!(stdout1.contains("backend/auth/jwt.md"));

    // Search typo "jwtoken" matches line in backend/auth/jwt.md
    let (success2, stdout2, _) = did_cmd(root, &["query", "jwtoken"]);
    assert!(success2);
    assert!(stdout2.contains("backend/auth/jwt.md: JSON Web Token verification"));
}

#[test]
fn test_autocomplete() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &["autocomplete", "bash"]);
    assert!(success);
    assert!(stdout.contains("did"));
    assert!(stdout.contains("complete"));
}

#[test]
fn test_debug_logging() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (success, _, stderr) = did_cmd_env(
        root,
        &["add", "jwt.md"],
        &[("DID_DEBUG", "1")],
    );
    assert!(success);
    assert!(stderr.contains("[DEBUG] Note added:"));

    fs::create_dir_all(root.join(".did/backend/auth/jwt")).unwrap();
    fs::write(root.join(".did/backend/auth/jwt/index.md"), "JWT Parent").unwrap();
    fs::write(root.join(".did/backend/auth/jwt/sub.md"), "Subtask").unwrap();

    let (success_done, _, stderr_done) = did_cmd_env(
        root,
        &["done", "backend/auth/jwt/index.md"],
        &[("DID_DEBUG", "1")],
    );
    assert!(success_done);
    assert!(stderr_done.contains("[DEBUG] Repo path resolved:"));
}

#[test]
fn test_parent_repo_search() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let sub = root.join("some").join("nested").join("dir");
    fs::create_dir_all(&sub).unwrap();

    // Run add from subdirectory
    let (success, _, _) = did_cmd_stdin(&sub, &["add", "task.md"], "From sub");
    assert!(success);

    assert!(root.join(".did/task.md").is_file());
}

#[test]
fn test_execution_logging_xml() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let log_file_rel = ".did.log";
    let (success, _, _) = did_cmd_env(
        root,
        &["status"],
        &[("DID_LOG_PATH", log_file_rel)],
    );
    assert!(success);

    let log_path = root.join(log_file_rel);
    assert!(log_path.is_file());

    let content = fs::read_to_string(&log_path).unwrap();
    assert!(content.contains("<invocation timestamp="));
    assert!(content.contains("<command>"));
    assert!(content.contains("<args>"));
    assert!(content.contains("</invocation>"));
}

#[test]
fn test_did_mv_single_file_updates_inbound_symlinks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    did_cmd(root, &["link", "backend/auth/jwt.md", "frontend"]);

    let (success, _, _) = did_cmd(
        root,
        &["mv", "backend/auth/jwt.md", "backend/auth/token.md"],
    );
    assert!(success);

    assert!(!root.join(".did/backend/auth/jwt.md").exists());
    assert!(root.join(".did/backend/auth/token.md").is_file());

    let symlink_file = root.join(".did/frontend/jwt.md");
    assert!(symlink_file.is_symlink());
    let target = fs::read_link(&symlink_file).unwrap();
    assert_eq!(target, Path::new("../backend/auth/token.md"));
}

#[test]
fn test_did_mv_directory_updates_inbound_and_outbound_symlinks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "spec.md"], "Doc spec");
    did_cmd(root, &["mv", "spec.md", "docs/spec.md"]);
    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);

    did_cmd(root, &["link", "docs/spec.md", "backend/auth"]);
    did_cmd(root, &["link", "backend/auth/jwt.md", "frontend"]);

    let (success, _, _) = did_cmd(root, &["mv", "backend/auth", "backend/security"]);
    assert!(success);

    assert!(!root.join(".did/backend/auth").exists());
    assert!(root.join(".did/backend/security/jwt.md").is_file());

    let inbound_symlink = root.join(".did/frontend/jwt.md");
    assert!(inbound_symlink.is_symlink());
    let inbound_target = fs::read_link(&inbound_symlink).unwrap();
    assert_eq!(inbound_target, Path::new("../backend/security/jwt.md"));

    let outbound_symlink = root.join(".did/backend/security/spec.md");
    assert!(outbound_symlink.is_symlink());
    let outbound_target = fs::read_link(&outbound_symlink).unwrap();
    assert_eq!(outbound_target, Path::new("../../docs/spec.md"));
}

#[test]
fn test_did_mv_destination_collision_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "a.md"], "Task A");
    did_cmd_stdin(root, &["add", "b.md"], "Task B");

    let (success, _, stderr) = did_cmd(root, &["mv", "a.md", "b.md"]);
    assert!(!success);
    assert!(stderr.contains("destination path already exists"));
}

#[test]
fn test_did_mv_auto_creates_parent_directories() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "a.md"], "Task A");

    let (success, _, _) = did_cmd(root, &["mv", "a.md", "nested/deep/folder/a.md"]);
    assert!(success);

    assert!(root.join(".did/nested/deep/folder/a.md").is_file());
}

#[test]
fn test_did_mv_resolved_dot_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    did_cmd(root, &["link", "backend/auth/jwt.md", "frontend"]);
    did_cmd(root, &["done", "backend/auth/jwt.md"]);

    let (success, _, _) = did_cmd(
        root,
        &["mv", "backend/auth/.jwt.md", "backend/auth/.token.md"],
    );
    assert!(success);

    assert!(root.join(".did/backend/auth/.token.md").is_file());
    let symlink_file = root.join(".did/frontend/.jwt.md");
    assert!(symlink_file.is_symlink());
    let target = fs::read_link(&symlink_file).unwrap();
    assert_eq!(target, Path::new("../backend/auth/.token.md"));
}

#[test]
fn test_did_mv_non_existent_source_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (success, _, stderr) = did_cmd(root, &["mv", "non_existent.md", "target.md"]);
    assert!(!success);
    assert!(stderr.contains("path does not exist"));
}

#[test]
fn test_bare_invocation_help() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &[]);
    assert!(success);
    assert!(stdout.contains("filesystem-native note and workflow tracker with hooks for defining local software factories"));
    assert!(stdout.contains("Usage:") || stdout.contains("Commands:"));
}

#[test]
fn test_executable_add_hook_abort() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/add");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, _) = did_cmd_stdin(root, &["add", "blocked_task.md"], "Blocked");
        assert!(!success);
        assert!(!root.join(".did/blocked_task.md").exists());
    }
}

#[test]
fn test_add_fallback_header_when_empty() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (success, _, _) = did_cmd(root, &["add", "auth-fix.md"]);
    assert!(success);

    let task_path = root.join(".did/auth-fix.md");
    assert!(task_path.is_file());
    let content = fs::read_to_string(task_path).unwrap();
    assert_eq!(content, "# auth-fix.md\n");
}

#[test]
fn test_link_non_existent_target_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (success, _, stderr) = did_cmd(root, &["link", "non_existent.md", "dest_dir"]);
    assert!(!success);
    assert!(stderr.contains("target does not exist"));
}

#[test]
fn test_autocomplete_unsupported_shell_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, _, stderr) = did_cmd(root, &["autocomplete", "invalid_shell"]);
    assert!(!success);
    assert!(stderr.contains("unsupported shell"));
}

#[test]
fn test_executable_hook_failure_fails_show() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task content");

    let hook = root.join(".did/.hooks/show");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, _) = did_cmd(root, &["show", "task.md"]);
        assert!(!success);
    }
}

#[test]
fn test_did_undone_restores_task_and_symlinks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "JWT implementation");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    did_cmd(root, &["link", "backend/auth/jwt.md", "frontend"]);
    did_cmd(root, &["done", "backend/auth/jwt.md"]);

    assert!(root.join(".did/backend/auth/.jwt.md").is_file());

    // Mark undone
    let (success, _, _) = did_cmd(root, &["undone", "backend/auth/jwt.md"]);
    assert!(success);

    assert!(root.join(".did/backend/auth/jwt.md").is_file());
    assert!(!root.join(".did/backend/auth/.jwt.md").exists());

    let symlink_file = root.join(".did/frontend/jwt.md");
    assert!(symlink_file.is_symlink());
    let target = fs::read_link(&symlink_file).unwrap();
    assert_eq!(target, Path::new("../backend/auth/jwt.md"));
}

#[test]
fn test_executable_done_hook_abort() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task content");

    let hook = root.join(".did/.hooks/done");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nexit 1\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, _) = did_cmd(root, &["done", "task.md"]);
        assert!(!success);
        assert!(root.join(".did/task.md").is_file());
    }
}

#[test]
fn test_hook_environment_variables() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/show");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(
        &hook,
        "#!/bin/sh\necho EVENT=$DID_EVENT\necho TARGET=$DID_TARGET\necho REPO=$DID_REPO_ROOT\necho STATE=$DID_STATE_DIR\n",
    )
    .unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        did_cmd_stdin(root, &["add", "task.md"], "Task content");

        let (success, _, stderr) = did_cmd(root, &["show", "task.md"]);
        assert!(success);
        assert!(stderr.contains("EVENT=show"));
        assert!(stderr.contains("TARGET=task.md"));
        assert!(stderr.contains("REPO="));
        assert!(stderr.contains("STATE="));
    }
}

#[test]
fn test_commands_require_repo_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (s1, _, err1) = did_cmd(root, &["status"]);
    assert!(!s1);
    assert!(err1.contains("error: no .did state directory found"));

    let (s2, _, err2) = did_cmd(root, &["show", "task.md"]);
    assert!(!s2);
    assert!(err2.contains("error: no .did state directory found"));

    let (s3, _, err3) = did_cmd(root, &["done", "task.md"]);
    assert!(!s3);
    assert!(err3.contains("error: no .did state directory found"));

    let (s4, _, err4) = did_cmd(root, &["undone", "task.md"]);
    assert!(!s4);
    assert!(err4.contains("error: no .did state directory found"));

    let (s5, _, err5) = did_cmd(root, &["link", "target.md", "dest"]);
    assert!(!s5);
    assert!(err5.contains("error: no .did state directory found"));

    let (s6, _, err6) = did_cmd(root, &["mv", "old.md", "new.md"]);
    assert!(!s6);
    assert!(err6.contains("error: no .did state directory found"));

    let (s7, _, err7) = did_cmd(root, &["search", "query"]);
    assert!(!s7);
    assert!(err7.contains("error: no .did state directory found"));
}

#[test]
fn test_add_non_executable_hook_and_no_newline_msg() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/add");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "Static Add Hook Guidance").unwrap();

    let (success, _, stderr) = did_cmd_stdin(root, &["add", "task1.md"], "No newline message");
    assert!(success);
    assert!(stderr.contains("Static Add Hook Guidance"));

    let task_content = fs::read_to_string(root.join(".did/task1.md")).unwrap();
    assert!(task_content.contains("No newline message"));
}

#[test]
fn test_add_hook_stdout_stderr_handling() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/add");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nprintf 'hook stdout'\n>&2 printf 'hook stderr'\nexit 0\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, stderr) = did_cmd_stdin(root, &["add", "task2.md"], "Content");
        assert!(success);
        assert!(stderr.contains("hook stdout"));
        assert!(stderr.contains("hook stderr"));
    }
}

#[test]
fn test_link_destination_exists_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "target.md"], "Target");
    did_cmd_stdin(root, &["add", "dest_target.md"], "Already there");
    did_cmd(root, &["mv", "dest_target.md", "dest/target.md"]);

    let (success, _, stderr) = did_cmd(root, &["link", "target.md", "dest"]);
    assert!(!success);
    assert!(stderr.contains("destination path already exists"));
}

#[test]
fn test_show_non_existent_file_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (success, _, stderr) = did_cmd(root, &["show", "non_existent.md"]);
    assert!(!success);
    assert!(stderr.contains("path does not exist"));
}

#[test]
fn test_done_recursive_directory() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "a.md"], "Task A");
    did_cmd(root, &["mv", "a.md", "feature/a.md"]);
    did_cmd_stdin(root, &["add", "b.md"], "Task B");
    did_cmd(root, &["mv", "b.md", "feature/b.md"]);
    did_cmd_stdin(root, &["add", "c.md"], "Task C");
    did_cmd(root, &["mv", "c.md", "feature/sub/c.md"]);

    let (success, stdout, stderr) = did_cmd(root, &["done", "-r", "feature"]);
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    assert!(success);

    assert!(root.join(".did/feature/.a.md").is_file());
    assert!(root.join(".did/feature/.b.md").is_file());
    assert!(root.join(".did/feature/sub/.c.md").is_file());
}

#[test]
fn test_done_directory_non_existent_and_already_done() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did/sub")).unwrap();

    let (s1, _, err1) = did_cmd(root, &["done", "sub"]);
    assert!(!s1);
    assert!(err1.contains("is a directory. Use 'did close -r sub' to close recursively"));

    let (s2, _, err2) = did_cmd(root, &["done", "non_existent.md"]);
    assert!(!s2);
    assert!(err2.contains("path does not exist"));

    did_cmd_stdin(root, &["add", "task.md"], "Done task");
    did_cmd(root, &["done", "task.md"]);

    let (s3, _, _) = did_cmd(root, &["done", ".task.md"]);
    assert!(s3);
}

#[test]
fn test_undone_directory_non_existent_not_resolved_collision() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did/sub")).unwrap();

    let (s1, _, err1) = did_cmd(root, &["undone", "sub"]);
    assert!(!s1);
    assert!(err1.contains("is a directory. Use 'did open -r sub' to reopen recursively"));

    let (s2, _, err2) = did_cmd(root, &["undone", "non_existent.md"]);
    assert!(!s2);
    assert!(err2.contains("path does not exist"));

    did_cmd_stdin(root, &["add", "open_task.md"], "Open task");

    let (s3, _, err3) = did_cmd(root, &["undone", "open_task.md"]);
    assert!(!s3);
    assert!(err3.contains("is not resolved"));

    did_cmd_stdin(root, &["add", "colliding.md"], "Open collision");
    fs::write(root.join(".did/.colliding.md"), "Done collision").unwrap();

    let (s4, _, err4) = did_cmd(root, &["undone", ".colliding.md"]);
    assert!(!s4);
    assert!(err4.contains("target path already exists"));
}

#[test]
fn test_status_and_search_non_existent_path_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (s1, _, err1) = did_cmd(root, &["status", "non_existent"]);
    assert!(!s1);
    assert!(err1.contains("path does not exist"));

    let (s2, _, err2) = did_cmd(root, &["search", "query", "non_existent"]);
    assert!(!s2);
    assert!(err2.contains("path does not exist"));
}

#[test]
fn test_execution_logging_absolute_path() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let abs_log = root.join("did_abs.log");
    let (success, _, _) = did_cmd_env(
        root,
        &["status"],
        &[("DID_LOG_PATH", abs_log.to_str().unwrap())],
    );
    assert!(success);
    assert!(abs_log.is_file());
}

#[test]
fn test_autocomplete_all_shells() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    for shell in &["zsh", "fish", "powershell", "elvish"] {
        let (success, stdout, _) = did_cmd(root, &["autocomplete", shell]);
        assert!(success);
        assert!(!stdout.is_empty());
    }
}


#[test]
fn test_done_hook_stdout_and_stderr() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task");

    let hook = root.join(".did/.hooks/done");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nprintf 'done stdout'\n>&2 printf 'done stderr'\nexit 0\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, stderr) = did_cmd(root, &["done", "task.md"]);
        assert!(success);
        assert!(stderr.contains("done stdout"));
        assert!(stderr.contains("done stderr"));
    }
}

#[test]
fn test_show_hook_stdout_no_newline_and_stderr_failure() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task");

    let hook = root.join(".did/.hooks/show");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\nprintf 'show stdout'\n>&2 printf 'show error'\nexit 1\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, stderr) = did_cmd(root, &["show", "task.md"]);
        assert!(!success);
        assert!(stderr.contains("show stdout"));
        assert!(stderr.contains("show error"));
    }
}

#[test]
fn test_cli_aliases_ln_and_move() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "a.md"], "A");

    let (s_move, _, _) = did_cmd(root, &["move", "a.md", "b.md"]);
    assert!(s_move);
    assert!(root.join(".did/b.md").is_file());

    let (s_ln, _, _) = did_cmd(root, &["ln", "b.md", "sub"]);
    assert!(s_ln);
    assert!(root.join(".did/sub/b.md").is_symlink());
}

#[test]
fn test_status_link_and_mv_hooks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let link_hook = root.join(".did/.hooks/link");
    fs::create_dir_all(link_hook.parent().unwrap()).unwrap();
    fs::write(
        &link_hook,
        "#!/bin/sh\nif echo \"$DID_TARGET\" | grep -q 'blocked'; then exit 1; fi\n",
    )
    .unwrap();

    let mv_hook = root.join(".did/.hooks/move");
    fs::write(
        &mv_hook,
        "#!/bin/sh\nif echo \"$DID_NEW\" | grep -q 'blocked'; then exit 1; fi\n",
    )
    .unwrap();

    let status_hook = root.join(".did/.hooks/status");
    fs::write(&status_hook, "#!/bin/sh\necho 'Status hook running'\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        for h in &[&link_hook, &mv_hook, &status_hook] {
            let mut perms = fs::metadata(h).unwrap().permissions();
            perms.set_mode(0o755);
            fs::set_permissions(h, perms).unwrap();
        }

        did_cmd_stdin(root, &["add", "task.md"], "Task");
        did_cmd_stdin(root, &["add", "blocked_task.md"], "Blocked");

        let (s_stat, _, stderr_stat) = did_cmd(root, &["status"]);
        assert!(s_stat);
        assert!(stderr_stat.contains("Status hook running"));

        let (s_link_ok, _, _) = did_cmd(root, &["link", "task.md", "dest1"]);
        assert!(s_link_ok);

        let (s_link_fail, _, _) = did_cmd(root, &["link", "blocked_task.md", "dest2"]);
        assert!(!s_link_fail);
        assert!(!root.join(".did/dest2/blocked_task.md").exists());

        let (s_mv_ok, _, _) = did_cmd(root, &["mv", "task.md", "task_renamed.md"]);
        assert!(s_mv_ok);

        let (s_mv_fail, _, _) = did_cmd(root, &["mv", "task_renamed.md", "blocked_renamed.md"]);
        assert!(!s_mv_fail);
        assert!(root.join(".did/task_renamed.md").exists());
    }
}

#[test]
fn test_did_test_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task");

    let (s_clean, stdout_clean, _) = did_cmd(root, &["test"]);
    assert!(s_clean);
    assert!(stdout_clean.contains(".did state directory is clean"));

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        symlink(root.join(".did/non_existent.md"), root.join(".did/broken.md")).unwrap();

        let (s_fail, _, stderr_fail) = did_cmd(root, &["test"]);
        assert!(!s_fail);
        assert!(stderr_fail.contains("Broken symlink:"));
    }
}

#[test]
fn test_did_test_empty_file_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    fs::write(root.join(".did/empty_task.md"), "   \n\t ").unwrap();

    let (s_fail, _, stderr_fail) = did_cmd(root, &["test"]);
    assert!(!s_fail);
    assert!(stderr_fail.contains("Empty or whitespace-only note file found: 'empty_task.md'"));
}

#[test]
fn test_did_test_executable_without_shebang_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let invalid_script = root.join(".did/.hooks/show");
    fs::create_dir_all(invalid_script.parent().unwrap()).unwrap();
    fs::write(&invalid_script, "echo 'No shebang'\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&invalid_script).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&invalid_script, perms).unwrap();

        let (success, _, stderr) = did_cmd(root, &["test"]);
        assert!(!success);
        assert!(stderr.contains("Executable file lacks shebang line (#!) or binary header:"));
    }
}

#[test]
fn test_did_rm_and_remove_hook() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "file.md"], "Content");
    did_cmd_stdin(root, &["add", "sub.md"], "Subcontent");
    did_cmd(root, &["mv", "sub.md", "dir/sub.md"]);

    let (s_dir_fail, _, err_dir) = did_cmd(root, &["rm", "dir"]);
    assert!(!s_dir_fail);
    assert!(err_dir.contains("is a directory. Use 'did rm -r dir'"));

    let remove_hook = root.join(".did/.hooks/remove");
    fs::create_dir_all(remove_hook.parent().unwrap()).unwrap();
    fs::write(
        &remove_hook,
        "#!/bin/sh\nif echo \"$DID_TARGET\" | grep -q 'protected'; then exit 1; fi\n",
    )
    .unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&remove_hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&remove_hook, perms).unwrap();

        did_cmd_stdin(root, &["add", "protected.md"], "Protected");

        let (s_rm_protected, _, _) = did_cmd(root, &["remove", "protected.md"]);
        assert!(!s_rm_protected);
        assert!(root.join(".did/protected.md").is_file());
    }

    let (s_rm_file, _, _) = did_cmd(root, &["rm", "file.md"]);
    assert!(s_rm_file);
    assert!(!root.join(".did/file.md").exists());

    let (s_rm_dir, _, _) = did_cmd(root, &["remove", "-r", "dir"]);
    assert!(s_rm_dir);
    assert!(!root.join(".did/dir").exists());
}

#[test]
fn test_non_executable_done_hook_reminds() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "Task content");

    let hook = root.join(".did/.hooks/done");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "REMINDER: Mark undone if incomplete").unwrap();

    let (success, _, stderr) = did_cmd(root, &["done", "task.md"]);
    assert!(success);
    assert!(stderr.contains("REMINDER: Mark undone if incomplete"));
    assert!(root.join(".did/.task.md").is_file());
}

#[test]
fn test_help_command_and_hook() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/help");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "Static Help Guidance").unwrap();

    let (success, stdout, stderr) = did_cmd(root, &["help"]);
    assert!(success);
    assert!(stderr.contains("Static Help Guidance"));
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_help_topic_hooks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &["help", "hooks"]);
    assert!(success);
    assert!(stdout.contains("# Lifecycle Hooks Documentation"));
    assert!(stdout.contains("Supported Lifecycle Hooks"));
    assert!(stdout.contains("help"));
}

#[test]
fn test_help_topic_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &["help", "status"]);
    assert!(success);
    assert!(stdout.contains("List open notes"));
}

#[test]
fn test_help_topic_alias() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    for (alias, expected_term) in [
        ("ln", "Symlink TARGET"),
        ("search", "Search note paths"),
        ("move", "Move or rename"),
        ("remove", "Remove a note file"),
    ] {
        let (success, stdout, _) = did_cmd(root, &["help", alias]);
        assert!(success, "did help {} should succeed", alias);
        assert!(
            stdout.contains(expected_term),
            "did help {} output should contain '{}', got: {}",
            alias,
            expected_term,
            stdout
        );
    }
}

#[test]
fn test_executable_help_hook() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let hook = root.join(".did/.hooks/help");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "#!/bin/sh\necho 'CUSTOM HELP HOOK RUNNING'\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();

        let (success, _, stderr) = did_cmd(root, &["help"]);
        assert!(success);
        assert!(stderr.contains("CUSTOM HELP HOOK RUNNING"));
    }
}

#[test]
fn test_show_content_unboxed_when_nocolor() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "task.md"], "# Task Title\nTask body");

    let (_, stdout_md, _) = did_cmd(root, &["show", "task.md"]);
    assert!(stdout_md.contains("task.md"));
    assert!(stdout_md.contains("# Task Title"));
    assert!(stdout_md.contains("Task body"));
}

#[test]
fn test_markdown_renderer_box_rendering_and_color_optout() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    did_cmd_stdin(root, &["add", "task.md"], "# Title");

    let ancestor_hook = root.join(".did/.hooks/show.md");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Hook text").unwrap();

    // With FORCE_COLOR=1, color rendering & red box borders for hooks are drawn on stderr
    let (s_force, _, stderr_force) = did_cmd_env(root, &["show", "task.md"], &[("FORCE_COLOR", "1")]);
    assert!(s_force);
    assert!(stderr_force.contains("╭─"));
    assert!(stderr_force.contains("╰─"));
    assert!(stderr_force.contains("\x1b[")); // ANSI color codes present

    // With DID_NO_BOX=1 & FORCE_COLOR=1, sections are separated with ---
    let (s_nobox, _, stderr_nobox) =
        did_cmd_env(root, &["show", "task.md"], &[("FORCE_COLOR", "1"), ("DID_NO_BOX", "1")]);
    assert!(s_nobox);
    assert!(stderr_nobox.contains("Hook text"));
    assert!(!stderr_nobox.contains("╭─"));

    // With NO_COLOR=1 & DID_NO_BOX=1, unboxed plain text is returned
    let (s_nocolor, stdout_nocolor, _) =
        did_cmd_env(root, &["show", "task.md"], &[("NO_COLOR", "1"), ("DID_NO_BOX", "1")]);
    assert!(s_nocolor);
    assert!(stdout_nocolor.contains("task.md"));
    assert!(!stdout_nocolor.contains("╭─"));

    // With DID_NO_COLOR=1, color rendering is opted out
    let (s_didno, stdout_didno, _) =
        did_cmd_env(root, &["show", "task.md"], &[("FORCE_COLOR", "1"), ("DID_NO_COLOR", "1")]);
    assert!(s_didno);
    assert!(!stdout_didno.contains("\x1b[31m"));

    // With DID_COLOR=0, color rendering is opted out
    let (s_didc0, stdout_didc0, _) =
        did_cmd_env(root, &["show", "task.md"], &[("FORCE_COLOR", "1"), ("DID_COLOR", "0")]);
    assert!(s_didc0);
    assert!(!stdout_didc0.contains("\x1b[31m"));
}

#[test]
fn test_codeblock_syntax_highlighting_and_boxing() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    did_cmd_stdin(
        root,
        &["add", "tryit.md"],
        "# Lovely\n\n```html\n<body>\n  <i>Hi</i>\n</body>\n```",
    );

    // With FORCE_COLOR=1, the task file is rendered with syntax highlighting
    let (s_force, stdout_force, _) =
        did_cmd_env(root, &["show", "tryit.md"], &[("FORCE_COLOR", "1")]);
    assert!(s_force);
    assert!(stdout_force.contains("body"));
    assert!(stdout_force.contains("Hi"));
    assert!(stdout_force.contains("\x1b[")); // ANSI color codes present

    // With NO_COLOR=1, unboxed raw markdown is returned
    let (s_nocolor, stdout_nocolor, _) = did_cmd_env(
        root,
        &["show", "tryit.md"],
        &[("FORCE_COLOR", "1"), ("NO_COLOR", "1")],
    );
    assert!(s_nocolor);
    assert!(stdout_nocolor.contains("```html"));
    assert!(!stdout_nocolor.contains("\x1b["));
}

#[test]
fn test_all_did_environment_variables() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "Auth JWT Implementation\nDetails");
    did_cmd(root, &["mv", "jwt.md", "auth/jwt.md"]);
    did_cmd_stdin(root, &["add", "session.md"], "Auth Session Management\nDetails");
    did_cmd(root, &["mv", "session.md", "auth/session.md"]);

    let hook = root.join(".did/.hooks/show");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(
        &hook,
        "#!/bin/sh\necho EVENT=$DID_EVENT\necho TARGET=$DID_TARGET\necho REPO=$DID_REPO_ROOT\necho STATE=$DID_STATE_DIR\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook, perms).unwrap();
    }

    // 1. DID_STATUS_LIMIT
    let (s1, stdout1, stderr1) = did_cmd_env(root, &["status"], &[("DID_STATUS_LIMIT", "1")]);
    assert!(s1);
    assert_eq!(stdout1.trim().lines().count(), 1);
    assert!(stderr1.contains("status limit reached"));

    // 2. DID_STATUS_PATH
    let (s2, stdout2, _) = did_cmd_env(root, &["status"], &[("DID_STATUS_PATH", "auth")]);
    assert!(s2);
    assert!(stdout2.contains("auth/jwt.md"));

    // 3. DID_LOG_PATH
    let log_file = root.join("execution.log");
    let (s3, _, _) = did_cmd_env(root, &["status"], &[("DID_LOG_PATH", log_file.to_str().unwrap())]);
    assert!(s3);
    assert!(log_file.is_file());

    // 4. DID_DEBUG
    let (s4, _, _stderr4) = did_cmd_env(root, &["status"], &[("DID_DEBUG", "1")]);
    assert!(s4);

    // 5. DID_COLOR & 6. DID_NO_COLOR
    let (s5, stdout5, _) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_COLOR", "1")]);
    assert!(s5);
    assert!(stdout5.contains("\x1b[")); // ANSI colors active

    let (s6, stdout6, _) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_NO_COLOR", "1")]);
    assert!(s6);
    assert!(!stdout6.contains("\x1b[31m")); // Color opted out

    // 7. DID_BOX & 8. DID_NO_BOX
    let (s7, _, stderr7) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_BOX", "1")]);
    assert!(s7);
    assert!(stderr7.contains("╭─")); // Box enabled for hook

    let (s8, _, stderr8) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_NO_BOX", "1")]);
    assert!(s8);
    assert!(!stderr8.contains("╭─")); // Box disabled

    // 9. DID_THEME & DID_SYNTAX_THEME
    for theme in &["light", "github", "solarized", "dark"] {
        let (st, stdout_t, _) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_THEME", theme)]);
        assert!(st);
        assert!(!stdout_t.is_empty());
    }

    // 10. DID_RELATED_LIMIT
    let (s10, _, stderr10) = did_cmd_env(root, &["show", "auth/jwt.md"], &[("FORCE_COLOR", "1"), ("DID_RELATED_LIMIT", "1")]);
    assert!(s10);
    assert!(stderr10.contains("Related"));
    assert!(stderr10.contains("auth/session.md"));

    // 11. Hook Env Vars
    let (shook, _, stderrhook) = did_cmd(root, &["show", "auth/jwt.md"]);
    assert!(shook);
    assert!(stderrhook.contains("EVENT=show"));
    assert!(stderrhook.contains("TARGET=auth/jwt.md"));
    assert!(stderrhook.contains("REPO="));
    assert!(stderrhook.contains("STATE="));
}

#[test]
fn test_did_log_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let status_git = Command::new("git")
        .args(["init"])
        .current_dir(root)
        .status()
        .expect("git init failed");
    assert!(status_git.success());

    Command::new("git").args(["config", "user.name", "Test User"]).current_dir(root).status().unwrap();
    Command::new("git").args(["config", "user.email", "test@example.com"]).current_dir(root).status().unwrap();

    let (s_add1, _, _) = did_cmd_stdin(root, &["add", "auth.md"], "Auth task");
    assert!(s_add1);
    did_cmd(root, &["mv", "auth.md", "refine/auth.md"]);

    did_cmd_stdin(root, &["add", "ui.md"], "UI task");
    did_cmd(root, &["mv", "ui.md", "refine/ui.md"]);

    Command::new("git").args(["add", "."]).current_dir(root).status().unwrap();
    Command::new("git").args(["commit", "-m", "Add initial tasks"]).current_dir(root).status().unwrap();

    // Link refine/auth.md into refine/ui -> blocks refine/ui.md
    let (s_link, _, _) = did_cmd(root, &["link", "refine/auth.md", "refine/ui"]);
    assert!(s_link);

    Command::new("git").args(["add", "."]).current_dir(root).status().unwrap();
    Command::new("git").args(["commit", "-m", "Link auth into ui"]).current_dir(root).status().unwrap();

    // Move task to implement/
    let (s_mv, _, _) = did_cmd(root, &["mv", "refine/auth.md", "implement/auth.md"]);
    assert!(s_mv);

    Command::new("git").args(["add", "."]).current_dir(root).status().unwrap();
    Command::new("git").args(["commit", "-m", "Move auth task"]).current_dir(root).status().unwrap();

    // Remove symlink in refine/ui
    let (s_rm, _, _) = did_cmd(root, &["rm", "refine/ui/auth.md"]);
    assert!(s_rm);

    Command::new("git").args(["add", "."]).current_dir(root).status().unwrap();
    Command::new("git").args(["commit", "-m", "Remove auth symlink from ui"]).current_dir(root).status().unwrap();

    // Complete task
    let (s_done, _, _) = did_cmd(root, &["done", "implement/auth.md"]);
    assert!(s_done);

    Command::new("git").args(["add", "."]).current_dir(root).status().unwrap();
    Command::new("git").args(["commit", "-m", "Close auth task"]).current_dir(root).status().unwrap();

    let (s_log, stdout_log, stderr_log) = did_cmd(root, &["log"]);
    assert!(s_log);
    assert!(stderr_log.contains("did log:"));
    assert!(stderr_log.contains("Breakdown:"));
    assert!(stdout_log.contains("Note *created*: `auth.md` by *Test User*") || stdout_log.contains("Note *created*: `refine/auth.md` by *Test User*"));
    assert!(stdout_log.contains("Note *linked*: `refine/ui/auth.md` by `refine/auth.md` by *Test User*") || stdout_log.contains("Note *blocked*: `refine/ui/auth.md` by `refine/auth.md` by *Test User*"));
    assert!(stdout_log.contains("Note *unlinked*: `refine/ui/auth.md` from `implement/auth.md` by *Test User*") || stdout_log.contains("Note *unblocked*: `refine/ui/auth.md` from `implement/auth.md` by *Test User*"));
    assert!(stdout_log.contains("Note *closed*: `implement/auth.md` -> `implement/.auth.md` by *Test User*"));

    // Test --json flag
    let (s_json, stdout_json, stderr_json) = did_cmd(root, &["log", "--json"]);
    assert!(s_json);
    assert!(stderr_json.contains("did log:"));
    assert!(stdout_json.contains("\"action\": \"blocked\""));
    assert!(stdout_json.contains("\"action\": \"unblocked\""));
    assert!(stdout_json.contains("\"action\": \"closed\""));
    assert!(stdout_json.starts_with("["));
    assert!(stdout_json.trim_end().ends_with("]"));

    let (s_nobox, _, stderr_nobox) = did_cmd_env(root, &["log"], &[("NO_BOX", "1")]);
    assert!(s_nobox);
    assert!(stderr_nobox.contains("---"));

    let (s_help, stdout_help, _) = did_cmd(root, &["help", "log"]);
    assert!(s_help);
    assert!(stdout_help.contains("View note Git history and lifecycle events"));
}

#[test]
fn test_add_with_title_only() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, _, _) = did_cmd(root, &["add", "My First Task"]);
    assert!(success);

    let created = root.join(".did/My-First-Task.md");
    assert!(created.is_file());
    let content = fs::read_to_string(&created).unwrap();
    assert_eq!(content, "# My First Task\n");
}

#[test]
fn test_add_with_title_and_piped_stdin() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, _, _) = did_cmd_stdin(root, &["add", "My Second Task"], "This is the task body.");
    assert!(success);

    let created = root.join(".did/My-Second-Task.md");
    assert!(created.is_file());
    let content = fs::read_to_string(&created).unwrap();
    assert_eq!(content, "# My Second Task\n\nThis is the task body.\n");
}

#[test]
fn test_add_with_piped_stdin_no_title_arg() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let stdin_input = "Title From Stdin\n\nBody details go here.";
    let (success, _, _) = did_cmd_stdin(root, &["add"], stdin_input);
    assert!(success);

    let created = root.join(".did/Title-From-Stdin.md");
    assert!(created.is_file());
    let content = fs::read_to_string(&created).unwrap();
    assert!(content.contains("# Title From Stdin"));
    assert!(content.contains("Body details go here."));
}

#[test]
fn test_bare_pipe_into_did() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let stdin_input = "Bare Pipe Feature\n\nFeature description from bare pipe.";
    let (success, _, _) = did_cmd_stdin(root, &[], stdin_input);
    assert!(success);

    let created = root.join(".did/Bare-Pipe-Feature.md");
    assert!(created.is_file());
    let content = fs::read_to_string(&created).unwrap();
    assert!(content.contains("# Bare Pipe Feature"));
    assert!(content.contains("Feature description from bare pipe."));
}

#[test]
fn test_bare_pipe_creates_repo_if_missing() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    assert!(!root.join(".did").exists());
    let (success, _, _) = did_cmd_stdin(root, &[], "Piped content without existing repo");
    assert!(success);
    assert!(root.join(".did").is_dir());
}

#[test]
fn test_add_empty_stdin_error() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    let (success, _, stderr) = did_cmd_stdin(root, &["add"], "   \n\t ");
    assert!(!success);
    assert!(stderr.contains("note title or piped stdin required"));
    assert!(stderr.contains("To create a note non-interactively"));
    assert!(stderr.contains("1. Pass a title argument:"));
}

#[test]
fn test_add_hash_header_title_stdin() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    let (success, _, _) = did_cmd_stdin(root, &["add"], "# Header Title\n\nTask details");
    assert!(success);
    assert!(root.join(".did/Header-Title.md").is_file());
}

#[test]
fn test_did_fzf_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    did_cmd(root, &["add", "fzf_task.md"]);

    let (s_help, stdout_help, _) = did_cmd(root, &["help", "fzf"]);
    assert!(s_help);
    assert!(stdout_help.contains("fuzzy finder") || stdout_help.contains("fzf"));
}

#[test]
fn test_fzf_with_path_arg_and_empty_check() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let (s_empty, _, stderr_empty) = did_cmd(root, &["fzf"]);
    if Command::new("fzf").arg("--version").output().is_ok() {
        assert!(s_empty);
        assert!(stderr_empty.contains("No notes found."));

        let (s_nonexist, _, stderr_nonexist) = did_cmd(root, &["fzf", "non_existent"]);
        assert!(!s_nonexist);
        assert!(stderr_nonexist.contains("path does not exist"));
    } else {
        assert!(!s_empty);
        assert!(stderr_empty.contains("fzf' executable was not found in PATH"));
    }
}

#[test]
fn test_did_fzf_missing_binary_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    did_cmd(root, &["add", "fzf_task.md"]);

    let (success, _, stderr) = did_cmd_env(root, &["fzf"], &[("PATH", "")]);
    assert!(!success);
    assert!(stderr.contains("fzf' executable was not found in PATH"));
}

#[test]
fn test_more_error_branches_for_coverage() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    // open -r non-existent
    let (s1, _, err1) = did_cmd(root, &["open", "-r", "non_existent"]);
    assert!(!s1);
    assert!(err1.contains("path does not exist"));

    // open -r file (not dir)
    did_cmd_stdin(root, &["add", "task.md"], "Task");
    did_cmd(root, &["done", "task.md"]);
    let (s2, _, _) = did_cmd(root, &["open", "-r", ".task.md"]);
    assert!(s2);

    // blocks on non-existent
    let (s3, _, err3) = did_cmd(root, &["blocks", "non_existent.md", "dest.md"]);
    assert!(!s3);
    assert!(err3.contains("target does not exist"));

    // status with non-existent path
    let (s4, _, err4) = did_cmd(root, &["status", "non_existent"]);
    assert!(!s4);
    assert!(err4.contains("path does not exist"));

    // rm non-existent
    let (s5, _, err5) = did_cmd(root, &["rm", "non_existent.md"]);
    assert!(!s5);
    assert!(err5.contains("path does not exist"));

    // fzf with tasks present
    let _ = did_cmd(root, &["fzf", "task.md"]);
}

#[test]
fn test_hook_box_colors_by_exit_code() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();
    did_cmd_stdin(root, &["add", "task.md"], "Task content");

    let hook_dir = root.join(".did/.hooks");
    fs::create_dir_all(&hook_dir).unwrap();

    // 1. Executable hook with exit 0 -> Green box (\x1b[32m)
    let hook_show = hook_dir.join("show");
    fs::write(&hook_show, "#!/bin/sh\necho \"hook stdout success\"\nexit 0\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_show).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_show, perms).unwrap();

        let (success, _, stderr) = did_cmd_env(
            root,
            &["show", "task.md"],
            &[("FORCE_COLOR", "1"), ("DID_BOX", "1")],
        );
        assert!(success);
        assert!(stderr.contains("hook stdout success"));
        assert!(stderr.contains("\x1b[32m")); // Green border for exit 0
    }

    // 2. Executable hook with exit 1 -> Red box (\x1b[31m)
    fs::write(&hook_show, "#!/bin/sh\necho \"hook stdout failure\"\nexit 1\n").unwrap();

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_show).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&hook_show, perms).unwrap();

        let (success, _, stderr) = did_cmd_env(
            root,
            &["show", "task.md"],
            &[("FORCE_COLOR", "1"), ("DID_BOX", "1")],
        );
        assert!(!success);
        assert!(stderr.contains("hook stdout failure"));
        assert!(stderr.contains("\x1b[31m")); // Red border for exit 1
    }

    // 3. Static non-executable hook file -> Green box (\x1b[32m)
    fs::write(&hook_show, "Static hook guidelines").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&hook_show).unwrap().permissions();
        perms.set_mode(0o644);
        fs::set_permissions(&hook_show, perms).unwrap();
    }

    let (success, _, stderr) = did_cmd_env(
        root,
        &["show", "task.md"],
        &[("FORCE_COLOR", "1"), ("DID_BOX", "1")],
    );
    assert!(success);
    assert!(stderr.contains("Static hook guidelines"));
    assert!(stderr.contains("\x1b[32m")); // Green border for static file
}

#[test]
fn test_did_init_command() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    assert!(!root.join(".did").exists());

    // 1. Initializing in an empty directory succeeds
    let (success1, stdout1, _) = did_cmd(root, &["init"]);
    assert!(success1);
    assert!(root.join(".did").is_dir());
    assert!(stdout1.contains("Initialized empty .did state directory"));

    // 2. Initializing when .did already exists fails
    let (success2, _, stderr2) = did_cmd(root, &["init"]);
    assert!(!success2);
    assert!(stderr2.contains("error: .did state directory already exists"));

    // 3. Initializing with explicit path argument
    let sub = root.join("project");
    fs::create_dir_all(&sub).unwrap();
    let (success3, stdout3, _) = did_cmd(root, &["init", sub.to_str().unwrap()]);
    assert!(success3);
    assert!(sub.join(".did").is_dir());
    assert!(stdout3.contains("Initialized empty .did state directory"));
}
