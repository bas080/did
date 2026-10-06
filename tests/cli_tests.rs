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
fn test_status_and_hierarchy() {
    let dir = tempdir().unwrap();
    let root = dir.path();

    did_cmd(root, &["init"]);

    // Add two leaf tasks in backend/auth/
    did_cmd(root, &["add", "backend/auth/a.md", "-m", "Task A"]);
    did_cmd(root, &["add", "backend/auth/b.md", "-m", "Task B"]);

    // Status shows both a.md and b.md as actionable
    let (success_stat, stdout_stat, _) = did_cmd(root, &["status"]);
    assert!(success_stat);
    assert_eq!(stdout_stat.trim(), "backend/auth/a.md\nbackend/auth/b.md");

    // Now add a deeper subdirectory backend/auth/sub/c.md
    did_cmd(root, &["add", "backend/auth/sub/c.md", "-m", "Task C"]);

    // Now a.md and b.md are blocked by sub/c.md! Actionable is only sub/c.md
    let (_, stdout_stat2, _) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat2.trim(), "backend/auth/sub/c.md");

    // Complete sub/c.md
    let (success_done_c, _, _) = did_cmd(root, &["done", "backend/auth/sub/c.md"]);
    assert!(success_done_c);
    assert!(root.join(".did/backend/auth/sub/.c.md").is_file());

    // Now a.md and b.md are actionable again!
    let (_, stdout_stat3, _) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat3.trim(), "backend/auth/a.md\nbackend/auth/b.md");

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
    assert!(stderr.contains("Use search or adjust limit"));
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
