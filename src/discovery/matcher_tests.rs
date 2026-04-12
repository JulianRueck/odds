use crate::discovery::matcher::match_candidate_multi;
use std::path::PathBuf;

fn path(s: &str) -> PathBuf {
    PathBuf::from(s)
}

// --- single token ---

#[test]
fn single_token_exact() {
    let result = match_candidate_multi(&path("/home/user/config"), &["config"]);
    assert!(result.is_some());
    assert_eq!(result.unwrap().score, 10.0);
}

#[test]
fn single_token_prefix() {
    let result = match_candidate_multi(&path("/home/user/config"), &["con"]);
    assert!(result.is_some());
    assert!(result.unwrap().score > 0.0);
}

#[test]
fn single_token_no_match() {
    let result = match_candidate_multi(&path("/home/user/config"), &["xyz"]);
    assert!(result.is_none());
}

// --- multi token ---

#[test]
fn two_tokens_both_match_different_segments() {
    let result = match_candidate_multi(&path("/home/user/projects/api"), &["proj", "api"]);
    assert!(result.is_some());
}

#[test]
fn two_tokens_order_independent() {
    let a = match_candidate_multi(&path("/home/user/projects/api"), &["proj", "api"]);
    let b = match_candidate_multi(&path("/home/user/projects/api"), &["api", "proj"]);
    assert!(a.is_some());
    assert!(b.is_some());
    assert_eq!(a.unwrap().score, b.unwrap().score);
}

#[test]
fn two_tokens_one_matches() {
    let result = match_candidate_multi(&path("/home/user/projects/api"), &["api", "xyz"]);
    assert!(result.is_some());
    // partial match scores lower than full match
    let full = match_candidate_multi(&path("/home/user/projects/api"), &["proj", "api"]);
    assert!(result.unwrap().score < full.unwrap().score);
}

#[test]
fn two_tokens_neither_matches() {
    let result = match_candidate_multi(&path("/home/user/projects/api"), &["xyz", "abc"]);
    assert!(result.is_none());
}

#[test]
fn more_tokens_than_segments_clips_surplus() {
    // path has 2 meaningful segments, 4 tokens supplied — should not panic
    let result = match_candidate_multi(&path("/projects/api"), &["proj", "api", "foo", "bar"]);
    // surplus tokens penalise the score
    let fewer = match_candidate_multi(&path("/projects/api"), &["proj", "api"]);
    assert!(result.unwrap().score < fewer.unwrap().score);
}

// --- optimal assignment ---

#[test]
fn hungarian_finds_optimal_over_greedy() {
    // "con" prefix-matches "config" (7 * 3/6 = 3.5)
    // "pro" prefix-matches "project" (7 * 3/7 ≈ 3.0)
    // optimal avg: (3.5 + 3) / 2 = 3.25
    // greedy worst case: one token stranded, other scores at best 3.5 → avg 1.75
    let result = match_candidate_multi(&path("/project/config"), &["con", "pro"]);
    let score = result.unwrap().score;
    assert!(score > 1.75);
}

// --- score ordering ---

#[test]
fn exact_match_scores_higher_than_prefix() {
    let exact = match_candidate_multi(&path("/home/api"), &["api"]).unwrap();
    let prefix = match_candidate_multi(&path("/home/apiary"), &["api"]).unwrap();
    assert!(exact.score > prefix.score);
}

#[test]
fn prefix_scores_higher_than_substring() {
    let prefix = match_candidate_multi(&path("/home/config"), &["con"]).unwrap();
    let substring = match_candidate_multi(&path("/home/myconfig"), &["con"]).unwrap();
    assert!(prefix.score > substring.score);
}

#[test]
fn full_match_scores_higher_than_partial() {
    let full = match_candidate_multi(&path("/projects/api"), &["projects", "api"]).unwrap();
    let partial = match_candidate_multi(&path("/projects/api"), &["projects", "xyz"]).unwrap();
    assert!(full.score > partial.score);
}

// --- edge cases ---

#[test]
fn empty_tokens_returns_none() {
    let result = match_candidate_multi(&path("/home/user/config"), &[]);
    assert!(result.is_none());
}

#[test]
fn empty_path_returns_none() {
    let result = match_candidate_multi(&PathBuf::new(), &["config"]);
    assert!(result.is_none());
}

#[test]
fn single_token_short_is_penalised() {
    let short = match_candidate_multi(&path("/home/config"), &["c"]).unwrap();
    let long = match_candidate_multi(&path("/home/config"), &["con"]).unwrap();
    assert!(short.score < long.score);
}
