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

    // Show nested task
    let (success_show, stdout_show, _) = did_cmd(root, &["show", "backend/auth/jwt.md"]);
    assert!(success_show);
    assert_eq!(
        stdout_show.trim(),
        "notes.md\nParent notes\n\nbackend/auth/jwt.md\nJWT implementation"
    );

    // Show directory fails
    let (success_show_dir, _, stderr_show_dir) = did_cmd(root, &["show", "backend"]);
    assert!(!success_show_dir);
    assert!(stderr_show_dir.contains("requires a task file, got directory"));
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

    // backend/server.md is blocked because backend/ has open symlink pointing to feature.md.
    // Actionable leaves are backend/feature.md and feature.md
    let (_, stdout_stat, _) = did_cmd(root, &["status"]);
    assert_eq!(stdout_stat.trim(), "backend/feature.md\nfeature.md");

    // Status specifically for backend subtree:
    let (_, stdout_stat_backend, _) = did_cmd(root, &["status", "backend"]);
    assert_eq!(stdout_stat_backend.trim(), "backend/feature.md");

    // Trying to complete server.md fails
    let (success_done_server, _, stderr_done_server) =
        did_cmd(root, &["done", "backend/server.md"]);
    assert!(!success_done_server);
    assert!(stderr_done_server.contains("unresolved sub-items remain"));

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
