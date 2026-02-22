use crate::{history::History, picker, ranking::RankedCandidate, session::SessionStack};
use std::path::Path;

/// Changes current directory and records it in short and longterm memory.
pub fn do_jump(dir: &Path, history: &mut History, session_stack: &mut SessionStack) {
    println!("{}", dir.display());

    session_stack.push(&dir);

    history.record_visit(&dir.to_path_buf());
    history.save().ok();
}

/// Displays picker and changes current directory to user picked directory.
/// Records it in short and longterm memory.
pub fn pick_and_jump(
    candidates: &[RankedCandidate],
    history: &mut History,
    session_stack: &mut SessionStack,
) {
    if let Some(picked) = picker::pick_directory(candidates) {
        do_jump(&picked.path, history, session_stack);
    } else {
        println!("No directory selected.");
    }
}
