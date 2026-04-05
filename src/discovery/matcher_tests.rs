// use std::path::PathBuf;

// use crate::discovery::{Matchkind, matcher::match_candidate};


// fn path() -> PathBuf {
//     PathBuf::from("/some/path")
// }

// // --- match_candidate ---

// #[test]
// fn exact_match() {
//     let result = match_candidate(&path(), "config", "config");
//     assert!(result.is_some());
//     assert_eq!(result.unwrap().match_kind, Matchkind::Exact);
// }

// #[test]
// fn prefix_match() {
//     let result = match_candidate(&path(), "config", "con");
//     assert!(result.is_some());
//     assert_eq!(result.unwrap().match_kind, Matchkind::Prefix);
// }

// #[test]
// fn substring_match() {
//     let result = match_candidate(&path(), "myconfig", "conf");
//     assert!(result.is_some());
//     assert_eq!(result.unwrap().match_kind, Matchkind::Substring);
// }

// #[test]
// fn fuzzy_match() {
//     let result = match_candidate(&path(), "configuration", "cfn");
//     assert!(result.is_some());
//     assert_eq!(result.unwrap().match_kind, Matchkind::Fuzzy);
// }

// #[test]
// fn no_match() {
//     let result = match_candidate(&path(), "config", "xyz");
//     assert!(result.is_none());
// }

// // --- match kind priority ---

// #[test]
// fn exact_beats_prefix() {
//     let exact = match_candidate(&path(), "src", "src").unwrap();
//     let prefix = match_candidate(&path(), "srcfiles", "src").unwrap();
//     assert!(exact.score > prefix.score);
// }

// #[test]
// fn prefix_beats_substring() {
//     let prefix = match_candidate(&path(), "config", "con").unwrap();
//     let substring = match_candidate(&path(), "myconfig", "con").unwrap();
//     assert!(prefix.score > substring.score);
// }

// #[test]
// fn substring_beats_fuzzy() {
//     let substring = match_candidate(&path(), "myconfig", "con").unwrap();
//     let fuzzy = match_candidate(&path(), "configuration", "cfn").unwrap();
//     assert!(substring.score > fuzzy.score);
// }

// // --- fuzzy score cap ---

// #[test]
// fn fuzzy_score_never_exceeds_cap() {
//     let result = match_candidate(&path(), "abcdefghij", "abcdefghij");
//     // Even a near-perfect fuzzy match should not exceed 45.0
//     if let Some(candidate) = result {
//         if candidate.match_kind == Matchkind::Fuzzy {
//             assert!(candidate.score <= 45.0);
//         }
//     }
// }

// // --- edge cases ---

// #[test]
// fn empty_token_matches_nothing() {
//     let result = match_candidate(&path(), "config", "");
//     assert!(result.is_none());
// }

// #[test]
// fn empty_name_matches_nothing() {
//     let result = match_candidate(&path(), "", "config");
//     assert!(result.is_none());
// }

// #[test]
// fn single_character_exact_match() {
//     let result = match_candidate(&path(), "s", "s");
//     assert!(result.is_some());
//     assert_eq!(result.unwrap().match_kind, Matchkind::Exact);
// }

// #[test]
// fn token_longer_than_name_does_not_match() {
//     let result = match_candidate(&path(), "src", "source");
//     assert!(result.is_none());
// }

// #[test]
// fn fuzzy_requires_all_chars_in_order() {
//     // "cfg" should not match "config" fuzzily if characters aren't all present in order
//     let result = match_candidate(&path(), "con", "cfg");
//     assert!(result.is_none());
// }
