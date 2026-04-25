use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;

pub const MARKOV_N: usize = 4;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Markov {
    #[serde_as(as = "Vec<(_, _)>")]
    pub chain: HashMap<Vec<String>, HashMap<String, usize>>,
}

impl Markov {
    /// Calculate the probability of visiting the target from the source.
    pub fn calculate_probability_from(&self, context: &[&str], from: &str, to: &str) -> f32 {
        for n in (1..=MARKOV_N).rev() {
            let key = self.build_key_str(context, from, n);
            if let Some(dest_map) = self.chain.get(&key) {
                let count = *dest_map.get(to).unwrap_or(&0) as f32;
                let total: usize = dest_map.values().sum();
                if total > 0 {
                    return count / total as f32;
                }
            }
        }
        0.0
    }

    /// Register the amount of transitions from one path (or tuple) to another.
    pub fn register(&mut self, context: &[&PathBuf], from: &PathBuf, to: &PathBuf) {
        let from_str = from.to_str().expect("Invalid UTF-8");
        let to_str = to.to_str().expect("Invalid UTF-8");

        if from_str == to_str {
            return;
        }

        for n in 1..=MARKOV_N {
            // Skip if context is too short to produce a unique key for this n.
            // Without this, empty context collapses all n-gram keys to the same
            // value, artificially inflating counts.
            if n > 1 && context.len() < n - 1 {
                continue;
            }

            let key = self.build_key(context, from_str, n);
            let dest_map = self.chain.entry(key).or_default();
            *dest_map.entry(to_str.to_string()).or_insert(0) += 1;
        }
    }

    fn build_key(&self, context: &[&PathBuf], from: &str, n: usize) -> Vec<String> {
        let mut key: Vec<String> = context
            .iter()
            .rev()
            .take(n - 1)
            .rev()
            .map(|p| p.to_str().unwrap_or("").to_string())
            .collect();
        key.push(from.to_string());
        key
    }

    fn build_key_str(&self, context: &[&str], from: &str, n: usize) -> Vec<String> {
        let mut key: Vec<String> = context.iter().rev().take(n - 1).rev().map(|s| s.to_string()).collect();
        key.push(from.to_string());
        key
    }

    pub fn transition_count(&self) -> usize {
        self.chain.values().map(|d| d.len()).sum()
    }
}

#[cfg(test)]
#[path = "markov_tests.rs"]
mod markov_tests;
