use cdd::{
    args::Cli,
    discovery::{self},
    navigator, paths,
    persistence::{History, Session},
    picker,
    ranking::{self, ConfidenceRules, MlWeights},
};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if cli.handle_init() {
        return;
    } else if let Some(token) = &cli.token {
        let mut session = Session::load_or_new();
        let mut history = History::load_or_new(); 

        let max_results = 9;
        let max_depth = 5;

        // Do a regular cd if it's an explicit path.
        if let Some(dir) = paths::detect_explicit_path(token) {
            navigator::do_jump(&dir, &mut history, &mut session);
            return;
        }

        let history_candidates = history.history_candidates(token);

        let ranked_candidates = ranking::rank_candidates(
            history_candidates,
            &history,
            &session,
            &MlWeights::default(),
            max_results,
        );

        // If confident auto jump.
        if let Some(choice) = picker::confident_pick(&ranked_candidates, ConfidenceRules::default()) {
            navigator::do_jump(&choice.candidate.path, &mut history, &mut session);
            return;
        }

        // Bounded discovery.
        let discovery_candidates = discovery::discover(token, max_depth, max_results);

        let ranked_candidates = ranking::rank_candidates(
            discovery_candidates,
            &history,
            &session,
            &MlWeights::default(),
            max_results,
        );

        // Picker.
        navigator::pick_and_jump(&ranked_candidates, &mut history, &mut session);
    }
}
