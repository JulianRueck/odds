use odds::persistence::markov::MARKOV_N;
use odds::persistence::{History, Session, persistable::Persistable};
use odds::ranking::ranker;
use std::path::PathBuf;
use std::sync::Mutex;
use tempfile::TempDir;

static TEST_MUTEX: Mutex<()> = Mutex::new(());

struct TestEnv {
    _dir: TempDir,
}

impl TestEnv {
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        unsafe {
            std::env::set_var("HOME", dir.path());
        }
        Self { _dir: dir }
    }

    fn real_path(&self, fake: &str) -> PathBuf {
        let relative = fake.trim_start_matches('/');
        let path = self._dir.path().join(relative);
        std::fs::create_dir_all(&path).unwrap();
        path.canonicalize().unwrap()
    }

    fn register(&self, to: &PathBuf) {
        let mut history = History::load_or_new();
        let mut session = Session::load_or_new();

        if let Some(current) = session.current() {
            let context: Vec<&PathBuf> = session
                .entries
                .iter()
                .skip(1)
                .take(MARKOV_N - 1)
                .map(|e| &e.path)
                .collect();
            history.chain.register(&context, current, to);
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
        let mut session = Session::load_or_new();
        session.entries.clear();
        session.save().unwrap();

        for path in journey {
            env.register(&env.real_path(path));
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
    assert_eq!(result, Some(env.real_path("/home/user/Projects/odds")));
}

#[test]
fn markov_predicts_next_directory() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(
        &env,
        &[
            "/home/user/Projects",
            "/home/user/Projects/odds",
            "/home/user/Projects/odds/src",
        ],
        40,
    );

    simulate_journey(
        &env,
        &["/home/user/Projects", "/home/user/Projects/other"],
        5,
    );

    let history = History::load_or_new();

    let prob_odds = history.chain.calculate_probability_from(
        &[],
        env.real_path("/home/user/Projects").to_str().unwrap(),
        env.real_path("/home/user/Projects/odds").to_str().unwrap(),
    );
    let prob_other = history.chain.calculate_probability_from(
        &[],
        env.real_path("/home/user/Projects").to_str().unwrap(),
        env.real_path("/home/user/Projects/other").to_str().unwrap(),
    );

    assert!(prob_odds > prob_other);
}

#[test]
fn trigram_improves_prediction_over_bigram() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(
        &env,
        &[
            "/home/user",
            "/home/user/Projects",
            "/home/user/Projects/odds",
        ],
        30,
    );

    simulate_journey(
        &env,
        &[
            "/home/user/.config",
            "/home/user/Projects",
            "/home/user/Projects/other",
        ],
        30,
    );

    let history = History::load_or_new();

    let projects = env
        .real_path("/home/user/Projects")
        .to_str()
        .unwrap()
        .to_string();
    let home = env.real_path("/home/user").to_str().unwrap().to_string();
    let odds = env
        .real_path("/home/user/Projects/odds")
        .to_str()
        .unwrap()
        .to_string();

    let bigram_odds = history
        .chain
        .calculate_probability_from(&[], &projects, &odds);
    assert_eq!(bigram_odds, 0.5, "bigram should be exactly ambiguous");

    let trigram_odds_from_home =
        history
            .chain
            .calculate_probability_from(&[&home], &projects, &odds);
    assert!(
        trigram_odds_from_home > 0.8,
        "trigram from home should strongly predict odds: {}",
        trigram_odds_from_home
    );
}

#[test]
fn recency_beats_frequency_for_stale_directories() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(&env, &["/home/user/Projects/old"], 100);

    {
        let old_path = env.real_path("/home/user/Projects/old");
        let mut history = History::load_or_new();
        for entry in history.entries.iter_mut() {
            if entry.path == old_path {
                entry.last_visited = 0;
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

    assert!(
        new_score > old_score,
        "recent should beat stale: new={} old={}",
        new_score,
        old_score
    );
}

#[test]
fn multi_token_finds_correct_directory() {
    let _lock = TEST_MUTEX.lock().unwrap();
    let env = TestEnv::new();

    simulate_journey(
        &env,
        &[
            "/home/user/Projects/odds",
            "/home/user/Projects/odds/src",
            "/home/user/Projects/other/src",
        ],
        20,
    );

    let result = env.top_candidate(&["odds", "src"]);
    assert_eq!(result, Some(env.real_path("/home/user/Projects/odds/src")));
}
