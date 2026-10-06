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
        if raw_str.starts_with(".did") {
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
        }
    }

    /// Returns the path relative to `did_dir` (e.g. "backend/auth/jwt.md").
    pub fn relative_display_path(&self, full_path: &Path) -> String {
        if let Ok(rel) = full_path.strip_prefix(&self.did_dir) {
            rel.to_string_lossy().to_string()
        } else {
            full_path.to_string_lossy().to_string()
        }
    }
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
}
