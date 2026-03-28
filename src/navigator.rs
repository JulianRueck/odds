use crate::{
    persistence::{History, Session, persistable::Persistable},
    picker,
    ranking::RankedCandidate,
};
use std::path::PathBuf;

/// Changes current directory and records it in short and longterm memory.
pub fn do_jump(dir: &PathBuf, history: &mut History, session: &mut Session) {
    println!("{}", dir.display());

    session.push(&dir);

    if let Err(e) = session.save() {
        eprintln!("Error saving session while jumping: {e}")
    }

    history.record_visit(&dir.to_path_buf());

    if let Err(e) = history.save() {
        eprintln!("Error saving history while jumping: {e}")
    }
}

/// Displays picker and changes current directory to user picked directory.
/// Records it in short and longterm memory.
pub fn pick_and_jump(candidates: &[RankedCandidate], history: &mut History, session: &mut Session) {
    // No need for picker if there's only one result.
    if candidates.len() == 1 {
        if let Some(candidate) = candidates.first() {
            do_jump(&candidate.path, history, session);
        };
    } else if let Some(picked) = picker::pick_directory(candidates) {
        do_jump(&picked.path, history, session);
    } else {
        println!("No directory selected.");
    }
}
