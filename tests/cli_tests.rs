use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

fn did_cmd(dir: &Path, args: &[&str]) -> (bool, String, String) {
    let bin_path = env!("CARGO_BIN_EXE_did");
    let output = Command::new(bin_path)
        .current_dir(dir)
        .args(args)
        .output()
        .expect("Failed to execute did binary");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.success(), stdout, stderr)
}

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

#[test]
fn test_init() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, _, _) = did_cmd(root, &["init"]);
    assert!(success);
    assert!(root.join(".did").is_dir());

    // Calling init again is a no-op
    let (success2, _, _) = did_cmd(root, &["init"]);
    assert!(success2);
}

#[test]
fn test_add_and_show() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    // Add root task notes.md
    let (success1, _, _) = did_cmd(root, &["add", "notes.md", "-m", "Parent notes"]);
    assert!(success1);
    assert!(root.join(".did/notes.md").is_file());

    // Add nested task
    let (success2, _, _) = did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );
    assert!(success2);
    assert!(root.join(".did/backend/auth/jwt.md").is_file());

    // Add duplicate task fails
    let (success_dup, _, stderr) = did_cmd(root, &["add", "notes.md", "-m", "duplicate"]);
    assert!(!success_dup);
    assert!(stderr.contains("path already exists"));

    // Show directory fails
    let (success_show_dir, _, stderr_show_dir) = did_cmd(root, &["show", "backend"]);
    assert!(!success_show_dir);
    assert!(stderr_show_dir.contains("requires a task file, got directory"));
    assert!(stderr_show_dir.contains("Use 'did status backend' instead"));
}

#[test]
fn test_show_hook_sibling_and_ancestor() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    // Create task
    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );

    // Create ancestor hook .did/backend/.hooks/show
    let ancestor_hook = root.join(".did/backend/.hooks/show");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Backend ancestor guidelines").unwrap();

    // Create sibling hook .did/backend/auth/.hooks/show
    let sibling_hook = root.join(".did/backend/auth/.hooks/show");
    fs::create_dir_all(sibling_hook.parent().unwrap()).unwrap();
    fs::write(&sibling_hook, "Auth sibling guidelines").unwrap();

    let (success, stdout, _) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success);
    assert_eq!(
        stdout.trim(),
        "backend/.hooks/show\nBackend ancestor guidelines\n\nbackend/auth/.hooks/show\nAuth sibling guidelines\n\nbackend/auth/jwt.md\nJWT implementation"
    );
}

#[test]
fn test_show_hook_with_extension() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );

    // Create hook with extension .did/backend/.hooks/show.md
    let ancestor_hook = root.join(".did/backend/.hooks/show.md");
    fs::create_dir_all(ancestor_hook.parent().unwrap()).unwrap();
    fs::write(&ancestor_hook, "Markdown hook content").unwrap();

    let (success, stdout, _) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success);
    assert!(stdout.contains("backend/.hooks/show.md"));
    assert!(stdout.contains("Markdown hook content"));
}

#[test]
fn test_status_and_hierarchy() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    // Add two leaf tasks in backend/auth/
    did_cmd(root, &["add", "backend/auth/a.md", "-m", "Task A"]);
    did_cmd(root, &["add", "backend/auth/b.md", "-m", "Task B"]);

    // Status shows both a.md and b.md as actionable
    let (success_stat, stdout_stat, stderr_stat) = did_cmd(root, &["status"]);
    assert!(success_stat);
    assert_eq!(stdout_stat.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat.contains("[0 blocked, 0 closed]"));

    // Now add a deeper subdirectory backend/auth/sub/c.md
    did_cmd(root, &["add", "backend/auth/sub/c.md", "-m", "Task C"]);

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
    println!("stderr_stat3: {:?}", stderr_stat3);
    assert_eq!(stdout_stat3.trim(), "backend/auth/a.md\nbackend/auth/b.md");
    assert!(stderr_stat3.contains("[0 blocked, 1 closed]"));

    // Status -a shows resolved task as well
    let (_, stdout_stat_a, _) = did_cmd(root, &["status", "-a"]);
    assert!(stdout_stat_a.contains("backend/auth/sub/.c.md"));
}

#[test]
fn test_status_limit_env() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "task1.md", "-m", "Task 1"]);
    did_cmd(root, &["add", "task2.md", "-m", "Task 2"]);
    did_cmd(root, &["add", "task3.md", "-m", "Task 3"]);

    let (success, stdout, stderr) =
        did_cmd_env(root, &["status"], &[("DID_STATUS_LIMIT", "2")]);
    assert!(success);
    let lines: Vec<&str> = stdout.trim().lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(stderr.contains("status limit reached (2/3 items shown)"));
    assert!(stderr.contains("Use status on a specific directory, search, or adjust limit"));
}

#[test]
fn test_show_and_done_blocked_output() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "backend/auth.md", "-m", "Auth task"]);
    did_cmd(root, &["add", "backend/auth/sub/research.md", "-m", "Research"]);

    // show backend/auth.md fails and lists blocking sub-items in stderr
    let (success_show, _, stderr_show) = did_cmd(root, &["show", "backend/auth.md"]);
    assert!(!success_show);
    assert!(stderr_show.contains("error: task 'backend/auth.md' is blocked by unresolved sub-items:"));
    assert!(stderr_show.contains("- backend/auth/sub/research.md"));

    // done backend/auth.md fails and lists blocking sub-items in stderr
    let (success_done, _, stderr_done) = did_cmd(root, &["done", "backend/auth.md"]);
    assert!(!success_done);
    assert!(stderr_done.contains("error: cannot mark task 'backend/auth.md' done: unresolved sub-items remain:"));
    assert!(stderr_done.contains("- backend/auth/sub/research.md"));
}

#[test]
fn test_link_and_done_updates_symlink() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "feature.md", "-m", "Feature requirement"]);
    did_cmd(
        root,
        &["add", "backend/server.md", "-m", "Server implementation"],
    );

    // Link feature.md into backend/
    let (success_link, _, _) = did_cmd(root, &["link", "feature.md", "backend"]);
    assert!(success_link);

    let symlink_file = root.join(".did/backend/feature.md");
    assert!(symlink_file.is_symlink());

    // Complete feature.md
    let (success_done_feat, _, _) = did_cmd(root, &["done", "feature.md"]);
    assert!(success_done_feat);
    assert!(root.join(".did/.feature.md").is_file());

    // Symlink inside backend should be updated to .feature.md!
    let updated_symlink = root.join(".did/backend/.feature.md");
    assert!(updated_symlink.is_symlink());
    let target = fs::read_link(&updated_symlink).unwrap();
    assert_eq!(target, Path::new("../.feature.md"));

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

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "backend/auth/jwt.md", "-m", "Token verification"]);
    did_cmd(root, &["add", "frontend/login.md", "-m", "Calls JWT auth endpoint"]);
    did_cmd(root, &["add", "docs/notes.md", "-m", "General documentation"]);

    // Search "jwt" matches backend/auth/jwt.md (path) and frontend/login.md (content)
    let (success1, stdout1, _) = did_cmd(root, &["search", "jwt"]);
    assert!(success1);
    assert_eq!(stdout1.trim(), "backend/auth/jwt.md\nfrontend/login.md");

    // Case-insensitive search "TOKEN"
    let (success2, stdout2, _) = did_cmd(root, &["search", "TOKEN"]);
    assert!(success2);
    assert_eq!(stdout2.trim(), "backend/auth/jwt.md");

    // Search with subtree path argument
    let (success3, stdout3, _) = did_cmd(root, &["search", "jwt", "frontend"]);
    assert!(success3);
    assert_eq!(stdout3.trim(), "frontend/login.md");

    // Search with -a flag and blocked items
    did_cmd(root, &["add", "backend/auth/jwt/subtask.md", "-m", "JWT helper task"]);
    // backend/auth/jwt.md is now blocked by jwt/subtask.md
    let (_, stdout_no_a, stderr_no_a) = did_cmd(root, &["search", "TOKEN"]);
    assert_eq!(stdout_no_a.trim(), "");
    assert!(stderr_no_a.contains("No matching tasks found."));

    let (_, stdout_with_a, _) = did_cmd(root, &["search", "TOKEN", "-a"]);
    assert!(stdout_with_a.contains("backend/auth/jwt.md"));

    // Status limit via env var DID_STATUS_LIMIT
    let (success_lim, stdout_lim, stderr_lim) = did_cmd_env(
        root,
        &["search", "jwt", "-a"],
        &[("DID_STATUS_LIMIT", "1")],
    );
    assert!(success_lim);
    let lines: Vec<&str> = stdout_lim.trim().lines().collect();
    assert_eq!(lines.len(), 1);
    assert!(stderr_lim.contains("status limit reached"));
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
fn test_parent_repo_search() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    let sub = root.join("some").join("nested").join("dir");
    fs::create_dir_all(&sub).unwrap();

    // Run add from subdirectory
    let (success, _, _) = did_cmd(&sub, &["add", "task.md", "-m", "From sub"]);
    assert!(success);

    assert!(root.join(".did/task.md").is_file());
}

#[test]
fn test_execution_logging_xml() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

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

    did_cmd(root, &["init"]);

    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );
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

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "docs/spec.md", "-m", "Doc spec"]);
    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );

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

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "a.md", "-m", "Task A"]);
    did_cmd(root, &["add", "b.md", "-m", "Task B"]);

    let (success, _, stderr) = did_cmd(root, &["mv", "a.md", "b.md"]);
    assert!(!success);
    assert!(stderr.contains("destination path already exists"));
}

#[test]
fn test_did_mv_auto_creates_parent_directories() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "a.md", "-m", "Task A"]);

    let (success, _, _) = did_cmd(root, &["mv", "a.md", "nested/deep/folder/a.md"]);
    assert!(success);

    assert!(root.join(".did/nested/deep/folder/a.md").is_file());
}

#[test]
fn test_did_mv_resolved_dot_files() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );
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

    did_cmd(root, &["init"]);

    let (success, _, stderr) = did_cmd(root, &["mv", "non_existent.md", "target.md"]);
    assert!(!success);
    assert!(stderr.contains("path does not exist"));
}

#[test]
fn test_bare_invocation_welcome() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    let (success, stdout, _) = did_cmd(root, &[]);
    assert!(success);
    assert!(stdout.contains("did - file-system-native issue and dependency tracker"));
    assert!(stdout.contains("USAGE GUIDANCE & AI AGENT WORKFLOW"));
}

#[test]
fn test_editor_failure_fails_add() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    let (success, _, stderr) = did_cmd_env(root, &["add", "failing_task.md"], &[("EDITOR", "false")]);
    assert!(!success);
    assert!(stderr.contains("editor 'false' failed"));
}

#[test]
fn test_link_non_existent_target_fails() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

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

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "task.md", "-m", "Task content"]);

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

    did_cmd(root, &["init"]);

    did_cmd(
        root,
        &["add", "backend/auth/jwt.md", "-m", "JWT implementation"],
    );
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

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "task.md", "-m", "Task content"]);

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
fn test_non_executable_done_hook_reminds() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    did_cmd(root, &["add", "task.md", "-m", "Task content"]);

    let hook = root.join(".did/.hooks/done");
    fs::create_dir_all(hook.parent().unwrap()).unwrap();
    fs::write(&hook, "REMINDER: Mark undone if incomplete").unwrap();

    let (success, stdout, _) = did_cmd(root, &["done", "task.md"]);
    assert!(success);
    assert!(stdout.contains("REMINDER: Mark undone if incomplete"));
    assert!(root.join(".did/.task.md").is_file());
}
