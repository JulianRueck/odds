use std::{
    env, fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::persistence::{
    History, Session,
    history::HistoryEntry,
    persistable::Persistable,
};

pub fn seed() -> anyhow::Result<()> {
    let hist_file = detect_hist_file()?;
    eprintln!("Seeding from {}", hist_file.display());

    let bytes = fs::read(&hist_file)?;
    let contents = String::from_utf8_lossy(&bytes).into_owned();
    let paths = extract_paths(&contents);

    if paths.is_empty() {
        eprintln!("No cd commands found in history file.");
        return Ok(());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let mut history = History::load_or_new();
    let mut session = Session::load_or_new();

    merge_history(&mut history, &paths, now);
    merge_session(&mut history, &paths);

    history.save()?;
    session.save()?;

    eprintln!(
        "Seeded {} directories and {} transitions.",
        history.entries.len(),
        history.transition_count()
    );

    Ok(())
}

fn detect_hist_file() -> anyhow::Result<PathBuf> {
    // Respect $HISTFILE if set.
    if let Ok(path) = env::var("HISTFILE") {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    // Fall back to common locations.
    let home = env::var("HOME")?;
    let candidates = [
        format!("{home}/.zsh_history"),
        format!("{home}/.bash_history"),
    ];

    for path in &candidates {
        let p = PathBuf::from(path);
        if p.exists() {
            return Ok(p);
        }
    }

    anyhow::bail!("Could not find a shell history file. Set $HISTFILE to point to yours.")
}

fn extract_paths(contents: &str) -> Vec<PathBuf> {
    let home = env::var("HOME").unwrap_or_default();
    let mut current = PathBuf::from(&home);
    let mut paths = Vec::new();
    let mut total_cd = 0;
    let mut skipped_nonexistent = 0;

    for line in contents.lines() {
        let line = if line.starts_with(": ") {
            match line.splitn(2, ';').nth(1) {
                Some(l) => l,
                None => continue,
            }
        } else {
            line
        };

        let line = line.trim();
        let dir = match line.strip_prefix("cd ") {
            Some(d) => d.trim(),
            None => continue,
        };

        total_cd += 1;

        if dir.is_empty() || dir == "-" {
            continue;
        }

        let expanded = if dir == "~" {
            PathBuf::from(&home)
        } else {
            PathBuf::from(dir.replace('~', &home))
        };

        // Resolve relative paths against current directory.
        let resolved = if expanded.is_absolute() {
            expanded
        } else {
            current.join(&expanded)
        };

        // Canonicalize to resolve `..` and `.` segments.
        let canonical = match resolved.canonicalize() {
            Ok(p) => p,
            Err(_) => {
                skipped_nonexistent += 1;
                continue;
            }
        };

        if canonical.is_dir() {
            current = canonical.clone();
            paths.push(canonical);
        } else {
            skipped_nonexistent += 1;
        }
    }

    eprintln!(
        "Found {} cd commands → {} valid paths ({} no longer exist)",
        total_cd,
        paths.len(),
        skipped_nonexistent,
    );

    paths
}

fn merge_history(history: &mut History, paths: &[PathBuf], now: u64) {
    for path in paths {
        if let Some(entry) = history.entries.iter_mut().find(|e| &e.path == path) {
            entry.visits += 1;
        } else {
            history.entries.push(HistoryEntry {
                path: path.clone(),
                visits: 1,
                last_visited: now,
            });
        }
    }
}

fn merge_session(history: &mut History, paths: &[PathBuf]) {
    for window in paths.windows(2) {
        history.register_markov_chain(&window[0], &window[1]);
    }
}
