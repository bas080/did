use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Repo {
    /// Absolute path to the .did directory (e.g. /workspace/project/.did)
    pub did_dir: PathBuf,
}

impl Repo {
    /// Search parent directories starting from `start_dir` for a `.did` directory.
    pub fn find(start_dir: &Path) -> Option<Self> {
        let mut current = start_dir
            .canonicalize()
            .ok()
            .unwrap_or_else(|| start_dir.to_path_buf());
        loop {
            let candidate = current.join(".did");
            if candidate.is_dir() {
                return Some(Self { did_dir: candidate });
            }
            if !current.pop() {
                break;
            }
        }
        None
    }

    /// Initialize a new .did directory in `target_dir`.
    /// Does nothing if `.did` already exists in `target_dir` or parent.
    pub fn init(target_dir: &Path) -> std::io::Result<Self> {
        let did_dir = target_dir.join(".did");
        if !did_dir.exists() {
            fs::create_dir_all(&did_dir)?;
        }
        Ok(Self { did_dir })
    }

    /// Maps a raw path given by the user to an absolute path inside `.did`.
    ///
    /// Rules:
    /// - If `path` is already inside `did_dir` or starts with `.did`, map appropriately.
    /// - Otherwise, prefix with `.did` relative to project root (`did_dir.parent()`).
    pub fn resolve_path(&self, raw_path: &Path) -> PathBuf {
        let raw_str = raw_path.to_string_lossy();
        let target = if raw_str.starts_with(".did") {
            // Strip ".did" or ".did/" prefix if relative
            let relative = raw_path.strip_prefix(".did").unwrap_or(raw_path);
            self.did_dir.join(relative)
        } else if raw_path.is_absolute() {
            // Check if absolute path is inside did_dir
            if raw_path.starts_with(&self.did_dir) {
                raw_path.to_path_buf()
            } else {
                // Prepend did_dir components if relative path was given as absolute by mistake,
                // or strip leading slash
                let relative = raw_path.strip_prefix("/").unwrap_or(raw_path);
                self.did_dir.join(relative)
            }
        } else {
            self.did_dir.join(raw_path)
        };

        if std::env::var("DID_DEBUG").map(|v| !v.is_empty()).unwrap_or(false) {
            eprintln!(
                "[DEBUG] Repo path resolved: raw='{}' -> target='{}'",
                raw_path.display(),
                target.display()
            );
        }

        target
    }

    /// Returns the path relative to `did_dir` (e.g. "backend/auth/jwt.md").
    pub fn relative_display_path(&self, full_path: &Path) -> String {
        if let Ok(rel) = full_path.strip_prefix(&self.did_dir) {
            rel.to_string_lossy().to_string()
        } else {
            full_path.to_string_lossy().to_string()
        }
    }

    /// Returns a list of unresolved blocking items (relative paths and symlink targets) under child subdirectories of `task_file`.
    pub fn get_unresolved_blocking_items(&self, task_file: &Path) -> Vec<String> {
        let parent_dir = match task_file.parent() {
            Some(p) => p,
            None => return Vec::new(),
        };

        let is_top_category = parent_dir.parent() == Some(&self.did_dir) || parent_dir == self.did_dir;
        let task_stem = task_file.file_stem().unwrap_or_default().to_string_lossy();
        let mut items = Vec::new();

        if task_stem == "index" {
            self.collect_unresolved_in_dir(parent_dir, &mut items);
            let self_rel = self.relative_display_path(task_file);
            items.retain(|item| item != &self_rel);
        } else if let Ok(entries) = fs::read_dir(parent_dir) {
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
                    if is_top_category {
                        if dir_name == task_stem {
                            self.collect_unresolved_in_dir(&p, &mut items);
                        }
                    } else {
                        self.collect_unresolved_in_dir(&p, &mut items);
                    }
                }
            }
        }

        items.sort();
        items.dedup();

        if !items.is_empty() && std::env::var("DID_DEBUG").map(|v| !v.is_empty()).unwrap_or(false) {
            eprintln!(
                "[DEBUG] Blocking check for '{}': {} unresolved child sub-items found",
                self.relative_display_path(task_file),
                items.len()
            );
        }

        items
    }

    fn collect_unresolved_in_dir(&self, dir: &Path, items: &mut Vec<String>) {
        let dir_name = dir.file_name().unwrap_or_default().to_string_lossy();
        if dir_name.starts_with('.') {
            return;
        }
        for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
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
                items.push(format!("{} -> {}", self.relative_display_path(p), target_str));
            } else if p.is_file() {
                items.push(self.relative_display_path(p));
            }
        }
    }

    /// Checks if a task file has any unresolved sub-items (deeper subdirectories).
    pub fn has_unresolved_subitems(&self, task_file: &Path) -> bool {
        !self.get_unresolved_blocking_items(task_file).is_empty()
    }
}

fn is_reserved_hook_file(path: &Path) -> bool {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    if matches!(
        stem.as_ref(),
        "show" | "status" | "done" | "close" | "undone" | "open" | "add" | "link" | "blocks" | "mv" | "move" | "test" | "query" | "remove" | "help"
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_repo_find_and_init() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        assert!(Repo::find(root).is_none());

        let repo = Repo::init(root).unwrap();
        assert!(repo.did_dir.exists());

        let sub_dir = root.join("a").join("b");
        fs::create_dir_all(&sub_dir).unwrap();

        let found = Repo::find(&sub_dir).expect("Should find .did in parent");
        assert_eq!(found.did_dir, repo.did_dir);
    }

    #[test]
    fn test_resolve_path() {
        let dir = tempdir().unwrap();
        let repo = Repo::init(dir.path()).unwrap();

        let path1 = repo.resolve_path(Path::new("backend/auth/jwt.md"));
        assert_eq!(path1, repo.did_dir.join("backend/auth/jwt.md"));

        let path2 = repo.resolve_path(Path::new(".did/backend/auth/jwt.md"));
        assert_eq!(path2, repo.did_dir.join("backend/auth/jwt.md"));

        let rel_display = repo.relative_display_path(&path1);
        assert_eq!(rel_display, "backend/auth/jwt.md");
    }

    #[test]
    fn test_resolve_absolute_did_path() {
        let dir = tempdir().unwrap();
        let repo = Repo::init(dir.path()).unwrap();
        let abs_did_path = repo.did_dir.join("sub/task.md");
        let resolved = repo.resolve_path(&abs_did_path);
        assert_eq!(resolved, abs_did_path);
    }

    #[test]
    fn test_resolve_and_display_external_path() {
        let dir = tempdir().unwrap();
        let repo = Repo::init(dir.path()).unwrap();

        let external_path = Path::new("/some/external/file.txt");
        let resolved = repo.resolve_path(external_path);
        assert!(resolved.starts_with(&repo.did_dir));

        let display = repo.relative_display_path(external_path);
        assert_eq!(display, external_path.to_string_lossy());
    }
}
