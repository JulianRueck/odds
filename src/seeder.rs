use std::{
    collections::HashMap,
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
    println!("Seeding from {}", hist_file.display());

    let contents = fs::read_to_string(&hist_file)?;
    let paths = extract_paths(&contents);

    if paths.is_empty() {
        println!("No cd commands found in history file.");
        return Ok(());
    }

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    let mut history = seed_history(&paths, now);
    let mut session = seed_session(&paths);

    history.save()?;
    session.save()?;

    println!(
        "Seeded {} directories and {} transitions.",
        history.entries.len(),
        session.transition_count()
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
        //format!("{home}/.local/share/fish/fish_history"), // TODO: impl Fish support
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

    contents
        .lines()
        .filter_map(|line| {
            // Handle zsh extended history format: `: timestamp:elapsed;command`
            let line = if line.starts_with(": ") {
                line.splitn(2, ';').nth(1)?
            } else {
                line
            };

            let line = line.trim();

            // Extract cd target.
            let dir = if let Some(rest) = line.strip_prefix("cd ") {
                rest.trim()
            } else {
                return None;
            };

            // Expand ~ and skip special targets.
            let expanded = if dir == "~" {
                home.clone()
            } else if dir == "-" || dir.is_empty() {
                return None;
            } else {
                dir.replace('~', &home)
            };

            let path = PathBuf::from(&expanded);

            // Only keep directories that exist on disk.
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect()
}

fn seed_history(paths: &[PathBuf], now: u64) -> History {
    let mut counts: HashMap<&PathBuf, usize> = HashMap::new();
    for path in paths {
        *counts.entry(path).or_insert(0) += 1;
    }

    let entries = counts
        .into_iter()
        .map(|(path, visits)| HistoryEntry {
            path: path.clone(),
            visits,
            last_visited: now,
        })
        .collect();

    History { entries }
}

fn seed_session(paths: &[PathBuf]) -> Session {
    let mut session = Session::load_or_new();

    for window in paths.windows(2) {
        let from = &window[0];
        let to = &window[1];
        if from != to {
            session.register_markov_chain(from, to);
        }
    }

    session
}