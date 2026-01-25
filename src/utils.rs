use std::path::Path;

pub fn detect_explicit_path(input: &str) -> Option<String> {
    let path = Path::new(input);
    if path.exists() && path.is_dir() {
        Some(path.to_string_lossy().to_string())
    } else {
        None
    }
}
