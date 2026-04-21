use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Markov {
    pub bigram: HashMap<String, HashMap<String, usize>>,
    #[serde_as(as = "Vec<((_, _), _)>")]
    pub trigram: HashMap<(String, String), HashMap<String, usize>>,
}

impl Markov {
    /// Calculate the probability of visiting the target from the source.
    pub fn calculate_probability_from(&self, prev: Option<&str>, from: &str, to: &str) -> f32 {
        // Try trigram first
        if let Some(prev) = prev {
            let key = (prev.to_string(), from.to_string());
            if let Some(dest_map) = self.trigram.get(&key) {
                let count = *dest_map.get(to).unwrap_or(&0) as f32;
                let total: usize = dest_map.values().sum();
                if total > 0 {
                    return count / total as f32;
                }
            }
        }

        // Fall back to bigram
        if let Some(dest_map) = self.bigram.get(from) {
            let count = *dest_map.get(to).unwrap_or(&0) as f32;
            let total: usize = dest_map.values().sum();
            if total > 0 {
                return count / total as f32;
            }
        }

        0.0
    }

    /// Register the amount of transitions from one path (or tuple) to another.
    pub fn register(&mut self, prev: Option<&PathBuf>, from: &PathBuf, to: &PathBuf) {
        let from_str = from.to_str().expect("Invalid UTF-8");
        let to_str = to.to_str().expect("Invalid UTF-8");

        if from_str == to_str {
            return;
        }

        // bigram: from → to
        let dest_map = self.bigram.entry(from_str.to_string()).or_default();
        *dest_map.entry(to_str.to_string()).or_insert(0) += 1;

        // trigram: (prev, from) → to
        if let Some(prev) = prev {
            if let Some(prev_str) = prev.to_str() {
                if prev_str != from_str {
                    let dest_map = self
                        .trigram
                        .entry((prev_str.to_string(), from_str.to_string()))
                        .or_default();
                    *dest_map.entry(to_str.to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    pub fn transition_count(&self) -> usize {
        let bigram_count: usize = self.bigram.values().map(|d| d.len()).sum();
        let trigram_count: usize = self.trigram.values().map(|d| d.len()).sum();
        bigram_count + trigram_count
    }
}

#[cfg(test)]
#[path = "markov_tests.rs"]
mod markov_tests;
