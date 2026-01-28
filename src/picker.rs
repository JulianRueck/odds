use std::{
    io::{self, Write},
    path::PathBuf,
};

pub fn pick_directory(candidates: &[PathBuf]) -> Option<PathBuf> {
    if candidates.is_empty() {
        return None;
    }

    println!(
        "Select a directory (1-{}), or 0 to cancel:",
        candidates.len()
    );
    for (i, dir) in candidates.iter().enumerate() {
        println!("{}) {}", i + 1, dir.display());
    }

    print!("Enter number: ");
    io::stdout().flush().ok()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    let choice: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => return None,
    };

    if choice == 0 || choice > candidates.len() {
        return None;
    } else {
        Some(candidates[choice - 1].clone())
    }
}
