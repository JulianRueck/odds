use clap::Parser;

use odds::{
    args::Cli,
    discovery::{self},
    navigation::{navigator, picker}, 
    paths,
    persistence::{History, Session},
    ranking::{ConfidenceRules, ranker},
};

fn main() {
    let cli = Cli::parse();

    if cli.handle_init() {
        return;
    } else if let Some(token) = &cli.token {
        let mut session = Session::load_or_new();
        let mut history = History::load_or_new();

        const MAX_RESULTS: usize = 9;
        const MAX_DEPTH: usize = 5;

        // Do a regular cd if it's an explicit path.
        if let Some(dir) = paths::detect_explicit_path(token) {
            navigator::do_jump(&dir, &mut history, &mut session);
            return;
        }

        let history_candidates = history.history_candidates(token);

        let ranked_candidates = ranker::rank_candidates(
            history_candidates,
            &history,
            &session, 
            MAX_RESULTS,
        );

        // If confident auto jump.
        if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default())
        {
            navigator::do_jump(&choice.candidate.path, &mut history, &mut session);
            return;
        }

        // Bounded discovery.
        let discovery_candidates = discovery::discover(token, MAX_DEPTH, MAX_RESULTS);

        let ranked_candidates = ranker::rank_candidates(
            discovery_candidates,
            &history,
            &session,
            MAX_RESULTS,
        );

        // Picker.
        navigator::pick_and_jump(&ranked_candidates, &mut history, &mut session);
    }
}
