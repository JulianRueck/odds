use crate::{history::History, session::SessionStack, picker};
use std::path::{Path, PathBuf};

pub struct Navigator<'a> {
    pub session_stack: &'a mut SessionStack,
    pub history: &'a mut History,
}

impl<'a> Navigator<'a> {
    /// Changes current directory and records it in short and longterm memory.
    pub fn do_jump(&mut self, dir: &Path) {
        println!("{}", dir.display());

        self.session_stack.push(&dir);

        self.history.record_visit(&dir.to_path_buf());
        self.history.save().ok();
    }

    /// Displays picker and changes current directory to user picked directory.
    /// Records it in short and longterm memory.
    pub fn pick_and_jump(&mut self, candidates: &[PathBuf]) {
        if let Some(dir) = picker::pick_directory(candidates) {
            self.do_jump(&dir);
        } else {
            println!("No directory selected.");
        }
    }
}
