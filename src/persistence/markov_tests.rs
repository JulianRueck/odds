use crate::persistence::markov::Markov;
use std::path::PathBuf;

fn p(s: &str) -> PathBuf {
    PathBuf::from(s)
}

#[test]
fn bigram_records_transition() {
    let mut markov = Markov::default();
    markov.register(&[], &p("/home/user/projects"), &p("/home/user/projects/odds"));

    assert_eq!(
        markov.calculate_probability_from(&[], "/home/user/projects", "/home/user/projects/odds"),
        1.0
    );
}

#[test]
fn trigram_takes_priority_over_bigram() {
    let mut markov = Markov::default();
    markov.register(&[], &p("/home/user/projects"), &p("/home/user/projects/odds"));
    markov.register(
        &[&p("/home/user")],
        &p("/home/user/projects"),
        &p("/home/user/projects/api"),
    );

    // with trigram context, api should win
    let prob = markov.calculate_probability_from(&["/home/user"], "/home/user/projects", "/home/user/projects/api");
    assert_eq!(prob, 1.0, "trigram should exclusively predict api given home context");

    // without context, bigram is ambiguous
    let prob = markov.calculate_probability_from(&[], "/home/user/projects", "/home/user/projects/odds");
    assert_eq!(
        prob, 0.5,
        "bigram should be exactly ambiguous with two equal destinations"
    );
}

#[test]
fn falls_back_to_bigram_when_no_trigram_data() {
    let mut markov = Markov::default();
    markov.register(&[], &p("/home/user/projects"), &p("/home/user/projects/odds"));

    // trigram context provided but no trigram data exists — should fall back to bigram
    let prob = markov.calculate_probability_from(&["/home/user"], "/home/user/projects", "/home/user/projects/odds");
    assert_eq!(prob, 1.0, "should fall back to bigram probability of 1.0");
}

#[test]
fn unknown_transition_returns_zero() {
    let markov = Markov::default();
    assert_eq!(
        markov.calculate_probability_from(&[], "/home/user", "/home/user/projects"),
        0.0,
        "unknown transition should return 0.0"
    );
}

#[test]
fn transition_count_includes_all_ngrams() {
    let mut markov = Markov::default();
    markov.register(&[], &p("/home/user"), &p("/home/user/projects"));
    markov.register(&[&p("/home")], &p("/home/user"), &p("/home/user/projects"));

    // Each register call records MARKOV_N entries so count > number of register calls
    assert!(markov.transition_count() > 0);
}

#[test]
fn probabilities_sum_to_one() {
    let mut markov = Markov::default();
    markov.register(&[], &p("/home/user"), &p("/home/user/projects"));
    markov.register(&[], &p("/home/user"), &p("/home/user/downloads"));
    markov.register(&[], &p("/home/user"), &p("/home/user/.config"));

    let p1 = markov.calculate_probability_from(&[], "/home/user", "/home/user/projects");
    let p2 = markov.calculate_probability_from(&[], "/home/user", "/home/user/downloads");
    let p3 = markov.calculate_probability_from(&[], "/home/user", "/home/user/.config");

    assert!(
        (p1 + p2 + p3 - 1.0).abs() < 0.001,
        "probabilities should sum to 1.0, got {}",
        p1 + p2 + p3
    );
}
