use std::path::PathBuf;

use crate::persistence::markov::Markov;

fn p(s: &str) -> PathBuf {
    PathBuf::from(s)
}

#[test]
fn bigram_records_transition() {
    let mut markov = Markov::default();
    markov.register(
        None,
        &p("/home/user/projects"),
        &p("/home/user/projects/odds"),
    );
    assert_eq!(
        markov.calculate_probability_from(None, "/home/user/projects", "/home/user/projects/odds"),
        1.0
    );
}

#[test]
fn trigram_takes_priority_over_bigram() {
    let mut markov = Markov::default();
    markov.register(
        None,
        &p("/home/user/projects"),
        &p("/home/user/projects/odds"),
    );
    markov.register(
        Some(&p("/home/user")),
        &p("/home/user/projects"),
        &p("/home/user/projects/api"),
    );

    // with trigram context, api should win
    let prob = markov.calculate_probability_from(
        Some("/home/user"),
        "/home/user/projects",
        "/home/user/projects/api",
    );
    assert!(prob > 0.0);

    // without trigram context, bigram is used — odds and api both registered so 0.5 each
    let prob =
        markov.calculate_probability_from(None, "/home/user/projects", "/home/user/projects/odds");
    assert_eq!(prob, 0.5);
}

#[test]
fn falls_back_to_bigram_when_no_trigram_data() {
    let mut markov = Markov::default();
    markov.register(
        None,
        &p("/home/user/projects"),
        &p("/home/user/projects/odds"),
    );

    let prob = markov.calculate_probability_from(
        Some("/home/user"),
        "/home/user/projects",
        "/home/user/projects/odds",
    );
    assert_eq!(prob, 1.0);
}

#[test]
fn unknown_transition_returns_zero() {
    let markov = Markov::default();
    assert_eq!(
        markov.calculate_probability_from(None, "/home/user", "/home/user/projects"),
        0.0
    );
}

#[test]
fn transition_count_includes_both_bigram_and_trigram() {
    let mut markov = Markov::default();
    markov.register(None, &p("/home/user"), &p("/home/user/projects"));
    markov.register(
        Some(&p("/home")),
        &p("/home/user"),
        &p("/home/user/projects"),
    );
    assert_eq!(markov.transition_count(), 2);
}

#[test]
fn probabilities_sum_to_one() {
    let mut markov = Markov::default();
    markov.register(None, &p("/home/user"), &p("/home/user/projects"));
    markov.register(None, &p("/home/user"), &p("/home/user/downloads"));
    markov.register(None, &p("/home/user"), &p("/home/user/.config"));

    let p1 = markov.calculate_probability_from(None, "/home/user", "/home/user/projects");
    let p2 = markov.calculate_probability_from(None, "/home/user", "/home/user/downloads");
    let p3 = markov.calculate_probability_from(None, "/home/user", "/home/user/.config");

    assert!((p1 + p2 + p3 - 1.0).abs() < 0.001);
}
