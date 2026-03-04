use crate::{persistence::History, persistence::SessionStack, picker, ranking::RankedCandidate};
use std::path::Path;

/// Changes current directory and records it in short and longterm memory.
pub fn do_jump(dir: &Path, history: &mut History, session_stack: &mut SessionStack) {
    println!("{}", dir.display());

    session_stack.push(&dir);
    // TODO: maybe handle potential errors
    let _ = session_stack.save();

    history.record_visit(&dir.to_path_buf());
    // TODO: maybe handle potential errors
    let _ = history.save();
}

/// Displays picker and changes current directory to user picked directory.
/// Records it in short and longterm memory.
pub fn pick_and_jump(
    candidates: &[RankedCandidate],
    history: &mut History,
    session_stack: &mut SessionStack,
) {
    // No need for picker if there's only one result.
    if candidates.len() == 1 {
        if let Some(candidate) = candidates.first() {
            do_jump(&candidate.path, history, session_stack);
        };
    } else if let Some(picked) = picker::pick_directory(candidates) {
        do_jump(&picked.path, history, session_stack);
    } else {
        println!("No directory selected.");
    }
}
