use std::sync::Mutex;
use std::path::PathBuf;
use tempfile::TempDir;
use odds::persistence::{History, Session, persistable::Persistable};
use odds::ranking::ranker;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

struct TestEnv {
    _dir: TempDir,
}

impl TestEnv {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        unsafe { std::env::set_var("HOME", dir.path()); }
        Self { _dir: dir }
    }

    fn register(&self, to: &PathBuf) {
        let mut history = History::load_or_new();
        let mut session = Session::load_or_new();

        if let Some(current) = session.current() {
            let prev = session.previous().cloned();
            history.chain.register(
                prev.as_ref(),
                current,
                to,
            );
        }

        history.record_visit(to);
        session.push(to);
        history.save().unwrap();
        session.save().unwrap();
    }

    fn top_candidate(&self, tokens: &[&str]) -> Option<PathBuf> {
        let history = History::load_or_new();
        let session = Session::load_or_new();
        let candidates = history.history_candidates(tokens);
        let ranked = ranker::rank_candidates(candidates, &history, &session, 1);
        ranked.into_iter().next().map(|c| c.candidate.path)
    }
}

fn simulate_journey(env: &TestEnv, journey: &[&str], times: usize) {
    for _ in 0..times {
        for path in journey {
            env.register(&PathBuf::from(path));
        }
    }
}

#[test]
fn frequent_project_surfaces_first() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(&env, &["/home/user/Projects/odds"], 50);
    simulate_journey(&env, &["/home/user/Backup/odds"], 5);

    let result = env.top_candidate(&["odds"]);
    assert_eq!(result, Some(PathBuf::from("/home/user/Projects/odds")));
}

#[test]
fn markov_predicts_next_directory() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(&env, &[
        "/home/user/Projects",
        "/home/user/Projects/odds",
        "/home/user/Projects/odds/src",
    ], 40);

    simulate_journey(&env, &[
        "/home/user/Projects",
        "/home/user/Projects/other",
    ], 5);

    env.register(&PathBuf::from("/home/user/Projects"));

    let history = History::load_or_new();

    let prob_odds = history.chain.calculate_probability_from(
        None,
        "/home/user/Projects",
        "/home/user/Projects/odds",
    );
    let prob_other = history.chain.calculate_probability_from(
        None,
        "/home/user/Projects",
        "/home/user/Projects/other",
    );

    assert!(prob_odds > prob_other);
}

#[test]
fn trigram_improves_prediction_over_bigram() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    // From home -> Projects, user always goes to odds
    simulate_journey(&env, &[
        "/home/user",
        "/home/user/Projects",
        "/home/user/Projects/odds",
    ], 30);

    // From config -> Projects, user always goes to other
    simulate_journey(&env, &[
        "/home/user/.config",
        "/home/user/Projects",
        "/home/user/Projects/other",
    ], 30);

    let history = History::load_or_new();

    // Bigram alone is ambiguous — Projects leads to both equally
    let bigram_odds = history.chain.calculate_probability_from(
        None,
        "/home/user/Projects",
        "/home/user/Projects/odds",
    );
    assert_eq!(bigram_odds, 0.5, "bigram should be exactly ambiguous");

    // Trigram resolves the ambiguity — coming from home strongly predicts odds
    let trigram_odds_from_home = history.chain.calculate_probability_from(
        Some("/home/user"),
        "/home/user/Projects",
        "/home/user/Projects/odds",
    );
    assert!(trigram_odds_from_home > 0.8,
        "trigram from home should strongly predict odds: {}", trigram_odds_from_home);
}

#[test]
fn recency_beats_frequency_for_stale_directories() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(&env, &["/home/user/Projects/old"], 100);

    {
        let mut history = History::load_or_new();
        for entry in history.entries.iter_mut() {
            if entry.path == PathBuf::from("/home/user/Projects/old") {
                entry.last_visited = 0; // Jan 1st 1970
            }
        }
        history.save().unwrap();
    }

    simulate_journey(&env, &["/home/user/Projects/new"], 5);

    let history = History::load_or_new();
    let session = Session::load_or_new();

    let old_candidates = history.history_candidates(&["old"]);
    let new_candidates = history.history_candidates(&["new"]);

    let old_ranked = ranker::rank_candidates(old_candidates, &history, &session, 1);
    let new_ranked = ranker::rank_candidates(new_candidates, &history, &session, 1);

    let old_score = old_ranked.first().map(|c| c.ranked_score).unwrap_or(0.0);
    let new_score = new_ranked.first().map(|c| c.ranked_score).unwrap_or(0.0);

    assert!(new_score > old_score, "recent should beat stale: new={} old={}", new_score, old_score);
}

#[test]
fn multi_token_finds_correct_directory() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(&env, &[
        "/home/user/Projects/odds",
        "/home/user/Projects/odds/src",
        "/home/user/Projects/other/src",
    ], 20);

    let result = env.top_candidate(&["odds", "src"]);
    assert_eq!(result, Some(PathBuf::from("/home/user/Projects/odds/src")));
}
