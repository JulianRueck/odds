use std::{
    fs::canonicalize,
    path::{Path, PathBuf},
};

const STORAGE_PATH: &str = ".local/share/odds/";

pub fn detect_explicit_path(input: &str) -> Option<PathBuf> {
    let path = Path::new(input);
    if path.exists() && path.is_dir() {
        Some(path.to_path_buf())
    } else {
        None
    }
}
/// Returns the roots from which the program is going to search for candidates;
/// which are: the current working directory, home
/// and potentialy a git repository i.e. a folder contaning a .git file.
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

/// Prefixes file name with the machines home plus storage path e.g.
/// ~/.local/share/cdd/<file>
pub fn persistence_path(file: &str) -> PathBuf {
    home_dir().join(STORAGE_PATH).join(file)
}

pub fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            eprintln!("Warning: $HOME is not set, falling back to current directory.");
            PathBuf::from(".")
        })
}

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
