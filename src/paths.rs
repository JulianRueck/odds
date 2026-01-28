use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
};

pub fn detect_explicit_path(input: &str) -> Option<PathBuf> {
    let path = Path::new(input);
    if path.exists() && path.is_dir() {
        Some(path.to_path_buf())
    } else {
        None
    }
}

pub fn search_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();

    if let Ok(pwd) = std::env::current_dir() {
        roots.push(pwd.clone());

        if let Some(git_root) = find_git_root(&pwd) {
            if git_root != pwd {
                roots.push(git_root);
            }
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        roots.push(PathBuf::from(home));
    }

    roots.sort();
    roots.dedup();

    roots
}

pub fn normalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let p = path.as_ref();

    canonicalize(p).unwrap_or_else(|_| p.to_path_buf())
}

/// Experimental feature
fn find_git_root(start: &Path) -> Option<PathBuf> {
    let mut current = Some(start);

    while let Some(dir) = current {
        if dir.join(".git").exists() {
            return Some(dir.to_path_buf());
        }
        current = dir.parent();
    }

    None
}
