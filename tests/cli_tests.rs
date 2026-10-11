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
    let (success, _, _) = did_cmd(root, &["add", "notes.md"]);
    assert!(success);
    assert!(root.join(".did").is_dir());
    assert!(root.join(".did/notes.md").is_file());
}

#[test]
fn test_add_and_show() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    // Add root task notes.md
    let (success1, _, _) = did_cmd(root, &["add", "notes.md"]);
    assert!(success1);
    assert!(root.join(".did/notes.md").is_file());

    // Add nested task
    did_cmd(root, &["add", "jwt.md"]);
    let (success2, _, _) = did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    assert!(success2);
    assert!(root.join(".did/backend/auth/jwt.md").is_file());

    // Add duplicate task fails
    let (success_dup, _, stderr) = did_cmd(root, &["add", "notes.md"]);
    assert!(!success_dup);
    assert!(stderr.contains("path already exists"));

    // Show directory fails
    let (success_show_dir, _, stderr_show_dir) = did_cmd(root, &["show", "backend"]);
    assert!(!success_show_dir);
    assert!(stderr_show_dir.contains("requires a task file, got directory"));
    assert!(stderr_show_dir.contains("Use 'did status backend' instead"));

    // Status -b lists blocked tasks
    did_cmd(root, &["add", "c.md"]);
    did_cmd(root, &["mv", "c.md", "backend/auth/sub/c.md"]);
    let (success_b, stdout_b, _) = did_cmd(root, &["status", "-b"]);
    assert!(success_b);
    assert!(stdout_b.contains("backend/auth/jwt.md"));
}

#[test]
fn test_show_hook_sibling_and_ancestor() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    // Create task
    did_cmd(root, &["add", "jwt.md"]);
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
}

#[test]
fn test_show_hook_with_extension() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "jwt.md"]);
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);

    // Create hook with extension .did/backend/.hooks/show.md
    let ancestor_hook = root.join(".did/backend/.hooks/show.md");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Markdown hook content").unwrap();

    let (success, _, stderr) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success);
    assert!(stderr.contains("Markdown hook content"));
}

#[test]
fn test_status_and_hierarchy() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    // Add two leaf tasks in backend/auth/
    did_cmd(root, &["add", "a.md"]);
    did_cmd(root, &["mv", "a.md", "backend/auth/a.md"]);
    did_cmd(root, &["add", "b.md"]);
    did_cmd(root, &["mv", "b.md", "backend/auth/b.md"]);

    // Status shows both a.md and b.md as actionable
    let (success_stat, stdout_stat, stderr_stat) = did_cmd(root, &["status"]);
    assert!(success_stat);
    assert_eq!(stdout_stat.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat.contains("[0 blocked, 0 closed]"));

    // Now add a deeper subdirectory backend/auth/sub/c.md
    did_cmd(root, &["add", "c.md"]);
    did_cmd(root, &["mv", "c.md", "backend/auth/sub/c.md"]);

    // Now a.md and b.md are blocked by sub/c.md! Actionable is only sub/c.md
    let (_, stdout_stat2, stderr_stat2) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat2.trim(), "backend/auth/sub/c.md");
    assert!(stderr_stat2.contains("[2 blocked, 0 closed]"));

    // Complete sub/c.md
    let (success_done_c, _, _) = did_cmd(root, &["done", "backend/auth/sub/c.md"]);
    assert!(success_done_c);
    assert!(root.join(".did/backend/auth/sub/.c.md").is_file());

    // Now a.md and b.md are actionable again!
    let (_, stdout_stat3, stderr_stat3) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat3.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat3.contains("[0 blocked, 1 closed]"));

    // Status -a shows resolved task as well
    let (_, stdout_stat_a, _) = did_cmd(root, &["status", "-a"]);
    assert!(stdout_stat_a.contains("backend/auth/sub/.c.md"));

    // Status -t (tree mode) shows actionable task with status indicator [ ]
    let (success_tree, stdout_tree, _) = did_cmd(root, &["status", "-t"]);
    assert!(success_tree);
    assert!(stdout_tree.contains("[ ] a.md"));
    assert!(stdout_tree.contains("[ ] b.md"));

    // Status -t -a (tree mode with all) shows closed task with [x]
    let (success_tree_a, stdout_tree_a, _) = did_cmd(root, &["status", "-t", "-a"]);
    assert!(success_tree_a);
    assert!(stdout_tree_a.contains("- [x] sub/.c.md") || stdout_tree_a.contains("[x] sub/.c.md"));
}

#[test]
fn test_status_limit_env() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "task1.md"]);
    did_cmd(root, &["add", "task2.md"]);
    did_cmd(root, &["add", "task3.md"]);

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

    did_cmd(root, &["add", "root_task.md"]);
    did_cmd(root, &["add", "task1.md"]);
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

    did_cmd(root, &["add", "auth.md"]);
    did_cmd(root, &["mv", "auth.md", "backend/auth.md"]);
    did_cmd(root, &["add", "research.md"]);
    did_cmd(root, &["mv", "research.md", "backend/auth/sub/research.md"]);

    // show backend/auth.md succeeds by default on blocked node, showing task content on stdout and warnings on stderr
    let (success_show, stdout_show, stderr_show) = did_cmd(root, &["show", "backend/auth.md"]);
    assert!(success_show);
    assert!(stderr_show.contains("[Blocked]") || stdout_show.contains("Blocked"));

    // done backend/auth.md fails and lists blocking sub-items in stderr
    let (success_done, _, stderr_done) = did_cmd(root, &["done", "backend/auth.md"]);
    assert!(!success_done);
    assert!(stderr_done.contains("error: cannot mark task 'backend/auth.md' done: unresolved sub-items remain:"));
}

#[test]
fn test_link_and_done_updates_symlink() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "feature.md"]);
    did_cmd(root, &["mv", "feature.md", "specs/feature.md"]);
    did_cmd(root, &["add", "server.md"]);
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

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd_stdin(root, &["add", "jwt.md"], "Token verification");
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    did_cmd_stdin(root, &["add", "login.md"], "Calls JWT auth endpoint");
    did_cmd(root, &["mv", "login.md", "frontend/login.md"]);
    did_cmd_stdin(root, &["add", "notes.md"], "General documentation");
    did_cmd(root, &["mv", "notes.md", "docs/notes.md"]);

    // Search "jwt" matches backend/auth/jwt.md (path) and frontend/login.md (content snippet)
    let (success1, stdout1, _) = did_cmd(root, &["search", "jwt"]);
    assert!(success1);
    assert!(stdout1.contains("backend/auth/jwt.md"));
    assert!(stdout1.contains("frontend/login.md: Calls JWT auth endpoint"));

    // Case-insensitive search "TOKEN"
    let (success2, stdout2, _) = did_cmd(root, &["search", "TOKEN"]);
    assert!(success2);
    assert!(stdout2.contains("backend/auth/jwt.md: Token verification"));
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
    assert!(stderr.contains("[DEBUG]"));
}

#[test]
fn test_parent_repo_search() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    let sub = root.join("some").join("nested").join("dir");
    fs::create_dir_all(&sub).unwrap();

    // Run add from subdirectory
    let (success, _, _) = did_cmd(&sub, &["add", "task.md"]);
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

    did_cmd(root, &["add", "jwt.md"]);
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
fn test_did_mv_destination_collision_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "a.md"]);
    did_cmd(root, &["add", "b.md"]);

    let (success, _, stderr) = did_cmd(root, &["mv", "a.md", "b.md"]);
    assert!(!success);
    assert!(stderr.contains("destination path already exists"));
}

#[test]
fn test_did_mv_auto_creates_parent_directories() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "a.md"]);

    let (success, _, _) = did_cmd(root, &["mv", "a.md", "nested/deep/folder/a.md"]);
    assert!(success);

    assert!(root.join(".did/nested/deep/folder/a.md").is_file());
}

#[test]
fn test_bare_invocation_help() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &[]);
    assert!(success);
    assert!(stdout.contains("filesystem-native issue tracker with hooks for defining local software factories"));
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

        let (success, _, _) = did_cmd(root, &["add", "blocked_task.md"]);
        assert!(!success);
        assert!(!root.join(".did/blocked_task.md").exists());
    }
}

#[test]
fn test_did_undone_restores_task_and_symlinks() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "jwt.md"]);
    did_cmd(root, &["mv", "jwt.md", "backend/auth/jwt.md"]);
    did_cmd(root, &["link", "backend/auth/jwt.md", "frontend"]);
    did_cmd(root, &["done", "backend/auth/jwt.md"]);

    assert!(root.join(".did/backend/auth/.jwt.md").is_file());

    // Mark undone
    let (success, _, _) = did_cmd(root, &["undone", "backend/auth/jwt.md"]);
    assert!(success);

    assert!(root.join(".did/backend/auth/jwt.md").is_file());
    assert!(!root.join(".did/backend/auth/.jwt.md").exists());
}

#[test]
fn test_executable_done_hook_abort() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "task.md"]);

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

        did_cmd(root, &["add", "task.md"]);

        let (success, _, stderr) = did_cmd(root, &["show", "task.md"]);
        assert!(success);
        assert!(stderr.contains("EVENT=show"));
        assert!(stderr.contains("TARGET=task.md"));
        assert!(stderr.contains("REPO="));
        assert!(stderr.contains("STATE="));
    }
}

#[test]
fn test_add_non_executable_hook() {
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

    did_cmd(root, &["add", "target.md"]);
    did_cmd(root, &["add", "target_dest.md"]);
    did_cmd(root, &["mv", "target_dest.md", "dest/target.md"]);

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

    did_cmd(root, &["add", "a.md"]);
    did_cmd(root, &["mv", "a.md", "feature/a.md"]);
    did_cmd(root, &["add", "b.md"]);
    did_cmd(root, &["mv", "b.md", "feature/b.md"]);
    did_cmd(root, &["add", "c.md"]);
    did_cmd(root, &["mv", "c.md", "feature/sub/c.md"]);

    let (success, _, _) = did_cmd(root, &["done", "-r", "feature"]);
    assert!(success);

    assert!(root.join(".did/feature/.a.md").is_file());
    assert!(root.join(".did/feature/.b.md").is_file());
    assert!(root.join(".did/feature/sub/.c.md").is_file());
}

#[test]
fn test_did_test_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "task.md"]);

    let (s_clean, stdout_clean, _) = did_cmd(root, &["test"]);
    assert!(s_clean);
    assert!(stdout_clean.contains(".did state directory is clean"));
}

#[test]
fn test_did_rm_and_remove_hook() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    fs::create_dir_all(root.join(".did")).unwrap();

    did_cmd(root, &["add", "file.md"]);

    let (s_rm_file, _, _) = did_cmd(root, &["rm", "file.md"]);
    assert!(s_rm_file);
    assert!(!root.join(".did/file.md").exists());
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
}

#[test]
fn test_help_topic_subcommand() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &["help", "status"]);
    assert!(success);
    assert!(stdout.contains("List actionables"));
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
